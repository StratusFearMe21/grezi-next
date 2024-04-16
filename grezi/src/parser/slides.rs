use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    ops::Bound,
};

use crate::layout::UnresolvedLayout;

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::{
    actions::Actions,
    objects::{Object, ObjectState, ResolvedObject},
    viewboxes::ViewboxIn,
    AstObject, FieldName, NodeKind, PassThroughHasher,
};
use eframe::emath::Align2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlideObj {
    pub object: u64,
    pub locations: [(Align2, ViewboxIn); 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

#[derive(Clone)]
pub struct ResolvedSlideObj {
    pub object: ResolvedObject,
    pub locations: [[[f32; 2]; 2]; 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_slides(
    tree_cursor: &mut GrzCursor<'_>,
    hasher: &ahash::RandomState,
    on_screen: &mut HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>>,
    objects: &mut HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
    source: &helix_core::ropey::Rope,
    errors_present: &mut Vec<super::Error>,
    bg: (super::Color, Option<(std::time::Duration, super::Color)>),
    viewboxes: &mut HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<(AstObject, Option<(std::time::Duration, super::Color)>), super::Error> {
    use std::num::NonZeroU16;

    tree_cursor.goto_first_child()?;
    tree_cursor.goto_first_child()?;
    let mut slide_objects = Vec::new();
    let mut slide_on_screen: HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    while tree_cursor.field_id() == NonZeroU16::new(FieldName::Objects as u16) {
        tree_cursor.fork(|cursor| {
            match parse_slide_object(
                cursor,
                hasher,
                on_screen,
                objects,
                source,
                viewboxes,
                // SAFETY: We don't remove slides when parsing them
                unsafe {
                    slide_objects
                        .last()
                        .map(|o| crate::lsp::you_can::borrow_unchecked(o))
                },
                |object| {
                    slide_on_screen.insert(
                        object.object,
                        (slide_objects.len(), object.state == ObjectState::Exiting),
                    );
                    slide_objects.push(object);
                },
            ) {
                Ok(()) => {}
                Err(e) => errors_present.push(e),
            }
        });
        tree_cursor.goto_next_sibling()?;
    }
    tree_cursor.goto_parent();
    tree_cursor.goto_next_sibling()?;
    tree_cursor.goto_first_child()?;
    // Draws Entering objects first, then OnScreen, then Exiting
    // slide_objects.sort_by_key(|obj| obj.state);
    // for (index, slide_object) in slide_objects.iter().enumerate() {
    //     *slide_on_screen.get_mut(&slide_object.object).unwrap() = index;
    // }
    let mut max_time = 0.5;
    let mut actions = Vec::new();
    let mut next = false;
    while tree_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
        tree_cursor.fork(|cursor| {
            match parse_slide_function(
                cursor,
                hasher,
                source,
                &mut slide_objects,
                &mut next,
                &mut max_time,
                &slide_on_screen,
                errors_present,
            ) {
                Ok(Some(slide_functions)) => actions.push(slide_functions),
                Err(e) => errors_present.push(e),
                _ => {}
            }
        });
        tree_cursor.goto_next_sibling()?;
    }
    core::mem::swap(&mut slide_on_screen, on_screen);
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    Ok((
        AstObject::Slide {
            objects: slide_objects,
            actions,
            bg,
            max_time: max_time.max(bg.1.map_or(f32::MIN, |f| f.0.as_secs_f32())),
            next,
        },
        bg.1,
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_slide_object(
    tree_cursor: &mut GrzCursor<'_>,
    hasher: &ahash::RandomState,
    on_screen: &mut HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>>,
    objects: &mut HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
    source: &helix_core::ropey::Rope,
    viewboxes: &mut HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    last_obj: Option<&SlideObj>,
    mut insert_fn: impl FnMut(SlideObj),
) -> Result<(), super::Error> {
    use crate::parser::{viewboxes::align_from_str, PointFromRange};

    use super::Error;

    tree_cursor.goto_first_child()?;
    let name = source.byte_slice(tree_cursor.node().byte_range());
    let object_name = {
        let mut hasher = hasher.build_hasher();
        std::hash::Hash::hash(&name, &mut hasher);
        hasher.finish()
    };
    let object = objects.get_mut(&object_name).ok_or_else(|| {
        Error::NotFound(crate::parser::PointFromRange::new(
            tree_cursor.node().range().into(),
            source,
        ))
    })?;
    tree_cursor.goto_next_sibling()?;
    let mut range = (Bound::Unbounded, Bound::Unbounded);
    if tree_cursor.node().kind_id() == NodeKind::Range as u16 {
        range = parse_range(tree_cursor, source)?;
        tree_cursor.goto_next_sibling()?;
    }
    let mut idx_range = None;
    let mut viewbox = if tree_cursor.node().kind_id() == NodeKind::SlideVb as u16 {
        let inline_vb_hash_range = tree_cursor.node().range();
        tree_cursor.goto_first_child_raw()?;
        let current_char = source.byte_slice(tree_cursor.node().byte_range());
        tree_cursor.goto_next_sibling()?;
        let vb = if current_char == ":" {
            super::viewboxes::parse_viewbox_ident(source, tree_cursor, hasher, viewboxes)?
        } else if current_char == "|" {
            let vb_range = tree_cursor.node().range();
            let mut vb =
                super::viewboxes::parse_viewbox_inner(tree_cursor, source, hasher, viewboxes)?;
            let name = {
                let mut hasher = hasher.build_hasher();
                std::hash::Hash::hash(&name, &mut hasher);
                std::hash::Hash::hash(&inline_vb_hash_range, &mut hasher);
                hasher.finish()
            };
            vb.margin = 0.0;
            if vb.constraints.is_empty() {
                return Err(super::Error::KnownMissing(
                    PointFromRange::new(vb_range.into(), source),
                    "constraints".into(),
                ));
            }
            tree_cursor.goto_next_sibling()?;
            tree_cursor.goto_first_child()?;
            let vb_index: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
            let vb_index: usize = vb_index.parse().unwrap();
            tree_cursor.goto_parent();

            if vb.constraints.get(vb_index).is_none() {
                return Err(super::Error::NotFound(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                )));
            }
            viewboxes.insert(name, vb);

            ViewboxIn::Custom(name, vb_index)
        } else {
            object.viewbox.ok_or_else(|| {
                Error::ImplicitEdge(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))
            })?
        };
        idx_range = Some(tree_cursor.node().range());
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling()?;
        vb
    } else {
        object.viewbox.ok_or_else(|| {
            Error::ImplicitEdge(PointFromRange::new(
                tree_cursor.node().range().into(),
                source,
            ))
        })?
    };
    let mut from: Option<ViewboxIn>;
    match NodeKind::from(tree_cursor.node().kind_id()) {
        NodeKind::SlideFrom => {
            tree_cursor.goto_first_child()?;
            from = Some(super::viewboxes::parse_viewbox_ident(
                source,
                tree_cursor,
                hasher,
                viewboxes,
            )?);
            tree_cursor.goto_parent();
            tree_cursor.goto_next_sibling()?;
        }
        _ => from = None,
    }

    if let ViewboxIn::Inherit(index) = viewbox {
        viewbox = last_obj
            .ok_or_else(|| {
                Error::ImplicitEdge(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))
            })?
            .locations[1]
            .1;

        if let Some(idx) = index {
            if let ViewboxIn::Custom(vb, vb_idx) = &mut viewbox {
                if let Some(vb) = viewboxes.get(vb) {
                    if vb.constraints.get(idx).is_none() {
                        return Err(super::Error::NotFound(PointFromRange::new(
                            idx_range.unwrap().into(),
                            source,
                        )));
                    }
                    *vb_idx = idx
                }
            }
        }
    }
    if let Some(ViewboxIn::Inherit(index)) = from {
        let mut viewbox = last_obj
            .ok_or_else(|| {
                Error::ImplicitEdge(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))
            })?
            .locations[1]
            .1;
        if let Some(idx) = index {
            if let ViewboxIn::Custom(vb, vb_idx) = &mut viewbox {
                if let Some(vb) = viewboxes.get(vb) {
                    if vb.constraints.get(idx).is_none() {
                        return Err(super::Error::NotFound(PointFromRange::new(
                            idx_range.unwrap().into(),
                            source,
                        )));
                    }
                    *vb_idx = idx
                }
            }
        }
        from = Some(viewbox);
    }

    let mut state = if on_screen
        .get(&object_name)
        .map(|e| !e.1)
        .unwrap_or_default()
    {
        ObjectState::OnScreen
    } else {
        ObjectState::Entering
    };

    let (locations, line_up_now) = {
        let edges: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();

        let lineup_first;
        let viewbox_first = from.unwrap_or_else(|| object.viewbox.unwrap_or(viewbox));
        let line_up_now;
        if &edges[..1] == ":" || &edges[..1] == "|" || &edges[..1] == "{" || edges == name {
            let object_position = object.position.ok_or_else(|| {
                Error::ImplicitEdge(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))
            })?;
            lineup_first = object_position;
            line_up_now = object_position;
            (
                [(line_up_now, viewbox_first), (lineup_first, viewbox)],
                line_up_now,
            )
        } else {
            let mut lineup_first_locations = match edges.get(..2) {
                Some(s) => {
                    lineup_first = align_from_str(s).ok_or_else(|| {
                        Error::InvalidParameter(PointFromRange::new(
                            tree_cursor.node().range().into(),
                            source,
                        ))
                    })?;
                    (lineup_first, viewbox_first)
                }
                None => {
                    lineup_first = object.position.ok_or_else(|| {
                        Error::ImplicitEdge(PointFromRange::new(
                            tree_cursor.node().range().into(),
                            source,
                        ))
                    })?;
                    (lineup_first, viewbox_first)
                }
            };
            let lineup_second = match edges.get(2..4) {
                Some(s) => {
                    line_up_now = align_from_str(s).ok_or_else(|| {
                        Error::InvalidParameter(PointFromRange::new(
                            tree_cursor.node().range().into(),
                            source,
                        ))
                    })?;
                    (line_up_now, viewbox)
                }
                None => match edges.get(2..) {
                    Some("|") => {
                        line_up_now = lineup_first;
                        state = ObjectState::Exiting;
                        lineup_first_locations = {
                            let lineup = object.position.ok_or_else(|| {
                                Error::BadExit(PointFromRange::new(
                                    tree_cursor.node().range().into(),
                                    source,
                                ))
                            })?;
                            (lineup, viewbox_first)
                        };
                        (lineup_first, viewbox)
                    }
                    _ => {
                        line_up_now = lineup_first;
                        lineup_first_locations = (
                            object.position.ok_or_else(|| {
                                Error::ImplicitEdge(PointFromRange::new(
                                    tree_cursor.node().range().into(),
                                    source,
                                ))
                            })?,
                            viewbox_first,
                        );
                        (lineup_first, viewbox)
                    }
                },
            };
            ([lineup_first_locations, lineup_second], line_up_now)
        }
    };
    if state == ObjectState::Exiting {
        object.position = None;
        object.viewbox = None;
    } else {
        object.position = Some(line_up_now);
        object.viewbox = Some(viewbox);
    }

    insert_fn(SlideObj {
        object: object_name,
        locations,
        state,
        scaled_time: [0.0, 0.5],
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_slide_function(
    tree_cursor: &mut GrzCursor<'_>,
    hasher: &ahash::RandomState,
    source: &helix_core::ropey::Rope,
    slide_objects: &mut [SlideObj],
    next: &mut bool,
    max_time: &mut f32,
    slide_on_screen: &HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>>,
    errors_present: &mut Vec<super::Error>,
) -> Result<Option<Actions>, super::Error> {
    use cssparser::ParserInput;
    use ecolor::Color32;

    use crate::parser::{
        actions::HIGHLIGHT_COLOR_DEFAULT, color::DefaultColorParser, viewboxes::align_from_str,
        PointFromRange,
    };

    tree_cursor.goto_first_child()?;
    let key: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    match key.as_ref() {
        "stagger" => {
            tree_cursor.goto_next_sibling()?;
            let scaler: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
            match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::NumberLiteral => {
                    let scaler: f32 = scaler.parse().unwrap();
                    let mut min_time = 0.0;
                    for object in slide_objects.iter_mut() {
                        if object.locations[0] != object.locations[1] {
                            object.scaled_time[0] = min_time;
                            *max_time += scaler;
                            min_time += scaler;
                        }
                    }
                    Ok(None)
                }
                _ => Err(super::Error::InvalidParameter(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))),
            }
        }
        "time" => {
            tree_cursor.goto_next_sibling()?;
            let time: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
            match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::NumberLiteral => {
                    let time: f32 = time.parse::<f32>().unwrap() - 0.5;
                    *max_time += time;
                    for object in slide_objects.iter_mut() {
                        object.scaled_time[1] += time;
                    }
                    Ok(None)
                }
                _ => Err(super::Error::InvalidParameter(PointFromRange::new(
                    tree_cursor.node().range().into(),
                    source,
                ))),
            }
        }
        "next" => {
            tree_cursor.goto_next_sibling()?;
            *next = true;
            Ok(None)
        }
        "speaker_notes" => {
            tree_cursor.goto_next_sibling()?;
            Ok(Some(Actions::SpeakerNotes(
                source
                    .byte_slice(tree_cursor.node().byte_range())
                    .to_string()
                    .into(),
            )))
        }
        "highlight" => {
            tree_cursor.goto_next_sibling()?;
            let object = slide_on_screen
                .get({
                    let mut hasher = hasher.build_hasher();
                    std::hash::Hash::hash(
                        &source.byte_slice(tree_cursor.node().byte_range()),
                        &mut hasher,
                    );
                    &hasher.finish()
                })
                .ok_or_else(|| {
                    super::Error::NotFound(PointFromRange::new(
                        tree_cursor.node().range().into(),
                        source,
                    ))
                })?;
            tree_cursor.goto_next_sibling()?;

            let locations = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    let from = match tree_cursor
                        .fork(|cursor| super::actions::parse_highlight_location(cursor, source))
                        .map_err(|_| {
                            super::Error::InvalidParameter(PointFromRange::new(
                                tree_cursor.node().range().into(),
                                source,
                            ))
                        }) {
                        Ok(from) => from,
                        Err(e) => {
                            errors_present.push(e);
                            Some([0, 0])
                        }
                    };
                    tree_cursor.goto_next_sibling()?;
                    match NodeKind::from(tree_cursor.node().kind_id()) {
                        NodeKind::StringLiteral => {
                            let to = match tree_cursor
                                .fork(|cursor| {
                                    super::actions::parse_highlight_location(cursor, source)
                                })
                                .map_err(|_| {
                                    super::Error::InvalidParameter(PointFromRange::new(
                                        tree_cursor.node().range().into(),
                                        source,
                                    ))
                                }) {
                                Ok(to) => to,
                                Err(e) => {
                                    errors_present.push(e);
                                    Some([0, 0])
                                }
                            };
                            match (from, to) {
                                (Some(from), Some(to)) => Some([from, to]),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                // "number_literal" => &source[tree_cursor.node().byte_range()],
                _ => None,
            };

            tree_cursor.goto_next_sibling()?;
            let color: Color32 = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    let value: Cow<'_, str> = source
                        .byte_slice(
                            tree_cursor
                                .node()
                                .child(1 /* second child */)
                                .unwrap_or(tree_cursor.node())
                                .byte_range(),
                        )
                        .into();
                    let t = super::color::parse_color_with(
                        &mut DefaultColorParser::new(Some(&mut HIGHLIGHT_COLOR_DEFAULT.into())),
                        &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                    )
                    .map_err(|e| {
                        super::Error::ColorError(
                            PointFromRange::new(tree_cursor.node().range().into(), source),
                            format!("{:?}", e),
                        )
                    })?;

                    t.1.into()
                }
                _ => HIGHLIGHT_COLOR_DEFAULT,
            };

            Ok(Some(Actions::Highlight {
                locations,
                index: object.0,
                persist: true,
                color,
            }))
        }
        "line" => {
            tree_cursor.goto_next_sibling()?;
            let first_object = slide_on_screen
                .get({
                    let mut hasher = hasher.build_hasher();
                    std::hash::Hash::hash(
                        &source.byte_slice(tree_cursor.node().byte_range()),
                        &mut hasher,
                    );
                    &hasher.finish()
                })
                .ok_or_else(|| {
                    super::Error::NotFound(PointFromRange::new(
                        tree_cursor.node().range().into(),
                        source,
                    ))
                })?;
            tree_cursor.goto_next_sibling()?;

            let first_direction = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    tree_cursor.goto_first_child()?;
                    let dir: Cow<'_, str> =
                        source.byte_slice(tree_cursor.node().byte_range()).into();
                    let dir = align_from_str(dir.as_ref()).ok_or_else(|| {
                        super::Error::InvalidParameter(PointFromRange::new(
                            tree_cursor.node().range().into(),
                            source,
                        ))
                    })?;
                    tree_cursor.goto_parent();
                    Some(dir)
                }
                _ => None,
            };

            tree_cursor.goto_next_sibling()?;
            let second_object = slide_on_screen
                .get({
                    let mut hasher = hasher.build_hasher();
                    std::hash::Hash::hash(
                        &source.byte_slice(tree_cursor.node().byte_range()),
                        &mut hasher,
                    );
                    &hasher.finish()
                })
                .ok_or_else(|| {
                    super::Error::NotFound(PointFromRange::new(
                        tree_cursor.node().range().into(),
                        source,
                    ))
                })?;
            tree_cursor.goto_next_sibling()?;

            let second_direction = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    tree_cursor.goto_first_child()?;
                    let dir: Cow<'_, str> =
                        source.byte_slice(tree_cursor.node().byte_range()).into();
                    let dir = align_from_str(dir.as_ref()).ok_or_else(|| {
                        super::Error::InvalidParameter(PointFromRange::new(
                            tree_cursor.node().range().into(),
                            source,
                        ))
                    })?;
                    tree_cursor.goto_parent();
                    Some(dir)
                }
                _ => None,
            };

            tree_cursor.goto_next_sibling()?;

            let color: Color32 = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    let value: Cow<'_, str> = source
                        .byte_slice(
                            tree_cursor
                                .node()
                                .child(1 /* second child */)
                                .unwrap_or(tree_cursor.node())
                                .byte_range(),
                        )
                        .into();
                    let t = super::color::parse_color_with(
                        &mut DefaultColorParser::new(Some(&mut Color32::WHITE.into())),
                        &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                    )
                    .map_err(|e| {
                        super::Error::ColorError(
                            PointFromRange::new(tree_cursor.node().range().into(), source),
                            format!("{:?}", e),
                        )
                    })?;

                    t.1.into()
                }
                _ => Color32::WHITE,
            };

            Ok(first_direction.and_then(|first_diretion| {
                Some(Actions::Line {
                    objects: [first_object.0, second_object.0],
                    locations: [first_diretion, second_direction?],
                    color,
                })
            }))
        }
        _ => Err(super::Error::ActionNotFound(PointFromRange::new(
            tree_cursor.node().range().into(),
            source,
        ))),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_range(
    tree_cursor: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
) -> Result<(Bound<usize>, Bound<usize>), super::Error> {
    tree_cursor.goto_first_child_raw()?;

    let upper_bound: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    let upper_bound: Bound<usize> = if upper_bound == ".." {
        Bound::Unbounded
    } else {
        Bound::Included(upper_bound.parse().unwrap())
    };

    tree_cursor.goto_next_sibling_raw()?;

    let range_type: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();

    tree_cursor.goto_next_sibling_raw()?;

    let lower_bound: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    let lower_bound: Bound<usize> = if lower_bound == ".." {
        Bound::Unbounded
    } else {
        match range_type.as_ref() {
            "..=" => Bound::Included(lower_bound.parse().unwrap()),
            _ => Bound::Excluded(lower_bound.parse().unwrap()),
        }
    };

    tree_cursor.goto_parent();

    Ok((upper_bound, lower_bound))
}
