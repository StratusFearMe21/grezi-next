use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    str::FromStr,
};

use crate::layout::UnresolvedLayout;
use eframe::epaint::text::cursor::PCursor;

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::{
    actions::Actions,
    objects::{Object, ObjectState, ResolvedObject},
    viewboxes::{LineUp, ViewboxIn},
    AstObject, FieldName, NodeKind, PassThroughHasher,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlideObj {
    pub object: u64,
    pub locations: [(LineUp, ViewboxIn); 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

#[derive(Debug, Clone)]
pub struct ResolvedSlideObj {
    pub object: ResolvedObject,
    pub locations: [[f32; 2]; 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_slides(
    mut tree_cursor: GrzCursor<'_>,
    hasher: &ahash::RandomState,
    on_screen: &mut HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    objects: &mut HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
    source: &ropey::Rope,
    errors_present: &mut Vec<super::Error>,
    viewboxes: &HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<AstObject, super::Error> {
    tree_cursor.goto_first_child()?;
    tree_cursor.goto_first_child()?;
    let mut slide_objects = Vec::new();
    let mut slide_on_screen: HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    while tree_cursor.field_id() == Some(FieldName::Objects as u16) {
        match parse_slide_object(
            tree_cursor.fork(),
            hasher,
            on_screen,
            objects,
            source,
            viewboxes,
        ) {
            Ok(object) => {
                slide_on_screen.insert(object.object, slide_objects.len());
                slide_objects.push(object);
            }
            Err(e) => errors_present.push(e),
        }
        tree_cursor.goto_next_sibling()?;
    }
    tree_cursor.goto_parent();
    tree_cursor.goto_next_sibling()?;
    tree_cursor.goto_first_child()?;
    // Draws Entering objects first, then OnScreen, then Exiting
    slide_objects.sort_by_key(|obj| obj.state);
    for (index, slide_object) in slide_objects.iter().enumerate() {
        *slide_on_screen.get_mut(&slide_object.object).unwrap() = index;
    }
    let mut max_time = 1.0;
    let mut actions = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
        match parse_slide_function(
            tree_cursor.fork(),
            hasher,
            source,
            &mut slide_objects,
            &mut max_time,
            &slide_on_screen,
            errors_present,
        ) {
            Ok(Some(slide_functions)) => actions.push(slide_functions),
            Err(e) => errors_present.push(e),
            _ => {}
        }
        tree_cursor.goto_next_sibling()?;
    }
    core::mem::swap(&mut slide_on_screen, on_screen);
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    Ok(AstObject::Slide {
        objects: slide_objects,
        actions,
        max_time,
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_slide_object(
    mut tree_cursor: GrzCursor<'_>,
    hasher: &ahash::RandomState,
    on_screen: &mut HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    objects: &mut HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
    source: &ropey::Rope,
    viewboxes: &HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<SlideObj, super::Error> {
    use super::Error;

    tree_cursor.goto_first_child()?;
    let object_name = {
        let mut hasher = hasher.build_hasher();
        std::hash::Hash::hash(
            &source.byte_slice(tree_cursor.node().byte_range()),
            &mut hasher,
        );
        hasher.finish()
    };
    let object = objects
        .get_mut(&object_name)
        .ok_or_else(|| Error::NotFound(tree_cursor.node().range().into()))?;
    tree_cursor.goto_next_sibling()?;
    let viewbox =
        super::viewboxes::parse_viewbox_ident(source, &mut tree_cursor, hasher, viewboxes)?;
    tree_cursor.goto_next_sibling()?;
    let from: Option<ViewboxIn>;
    match NodeKind::from(tree_cursor.node().kind_id()) {
        NodeKind::SlideFrom => {
            tree_cursor.goto_first_child()?;
            from = Some(super::viewboxes::parse_viewbox_ident(
                source,
                &mut tree_cursor,
                hasher,
                viewboxes,
            )?);
            tree_cursor.goto_parent();
            tree_cursor.goto_next_sibling()?;
        }
        _ => from = None,
    }

    let mut state = if on_screen.contains_key(&object_name) {
        ObjectState::OnScreen
    } else {
        ObjectState::Entering
    };

    let (locations, line_up_now) = {
        let edges: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();

        let lineup_first;
        let viewbox_first = from.unwrap_or_else(|| object.viewbox.unwrap_or(viewbox));
        let line_up_now;
        if &edges[..1] == "[" || &edges[..1] == "{" {
            let object_position = object
                .position
                .ok_or_else(|| Error::ImplicitEdge(tree_cursor.node().range().into()))?;
            lineup_first = object_position;
            line_up_now = object_position;
            (
                [(line_up_now, viewbox_first), (lineup_first, viewbox)],
                line_up_now,
            )
        } else {
            let mut lineup_first_locations = match edges.get(..2) {
                Some(s) => {
                    lineup_first = LineUp::from_str(s).unwrap();
                    (lineup_first, viewbox_first)
                }
                None => {
                    lineup_first = object
                        .position
                        .ok_or_else(|| Error::ImplicitEdge(tree_cursor.node().range().into()))?;
                    (lineup_first, viewbox_first)
                }
            };
            let lineup_second = match edges.get(2..4) {
                Some(s) => {
                    line_up_now = LineUp::from_str(s).unwrap();
                    (line_up_now, viewbox)
                }
                None => match edges.get(2..) {
                    Some("|") => {
                        line_up_now = lineup_first;
                        state = ObjectState::Exiting;
                        lineup_first_locations = {
                            let lineup = object
                                .position
                                .ok_or_else(|| Error::BadExit(tree_cursor.node().range().into()))?;
                            (lineup, viewbox_first)
                        };
                        (lineup_first, viewbox)
                    }
                    _ => {
                        line_up_now = object.position.ok_or_else(|| {
                            Error::ImplicitEdge(tree_cursor.node().range().into())
                        })?;
                        lineup_first_locations = (line_up_now, viewbox_first);
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

    Ok(SlideObj {
        object: object_name,
        locations,
        state,
        scaled_time: [0.0, 0.5],
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_slide_function(
    mut tree_cursor: GrzCursor<'_>,
    hasher: &ahash::RandomState,
    source: &ropey::Rope,
    slide_objects: &mut Vec<SlideObj>,
    max_time: &mut f32,
    slide_on_screen: &HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    errors_present: &mut Vec<super::Error>,
) -> Result<Option<Actions>, super::Error> {
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
                    for object in slide_objects.iter_mut().skip(1) {
                        *max_time += scaler;
                        min_time += scaler;
                        object.scaled_time[0] = min_time;
                    }
                    Ok(None)
                }
                _ => Err(super::Error::InvalidParameter(
                    tree_cursor.node().range().into(),
                )),
            }
        }
        "time" => {
            tree_cursor.goto_next_sibling()?;
            let time: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
            match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::NumberLiteral => {
                    let time: f32 = (time.parse::<f32>().unwrap() - 0.5).abs();
                    *max_time += time;
                    for object in slide_objects.iter_mut() {
                        object.scaled_time[1] += time;
                    }
                    Ok(None)
                }
                _ => Err(super::Error::InvalidParameter(
                    tree_cursor.node().range().into(),
                )),
            }
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
                .ok_or_else(|| super::Error::NotFound(tree_cursor.node().range().into()))?;
            tree_cursor.goto_next_sibling()?;

            let locations = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    let from =
                        match parse_highlight_location(tree_cursor.fork(), source).map_err(|_| {
                            super::Error::InvalidParameter(tree_cursor.node().range().into())
                        }) {
                            Ok(from) => from,
                            Err(e) => {
                                errors_present.push(e);
                                PCursor::default()
                            }
                        };
                    tree_cursor.goto_next_sibling()?;
                    match NodeKind::from(tree_cursor.node().kind_id()) {
                        NodeKind::StringLiteral => {
                            let to = match parse_highlight_location(tree_cursor.fork(), source)
                                .map_err(|_| {
                                    super::Error::InvalidParameter(
                                        tree_cursor.node().range().into(),
                                    )
                                }) {
                                Ok(to) => to,
                                Err(e) => {
                                    errors_present.push(e);
                                    PCursor::default()
                                }
                            };
                            Some([from, to])
                        }
                        _ => None,
                    }
                }
                // "number_literal" => &source[tree_cursor.node().byte_range()],
                _ => None,
            };
            Ok(Some(Actions::Highlight {
                locations,
                index: *object,
                persist: true,
            }))
        }
        _ => Err(super::Error::ActionNotFound(
            tree_cursor.node().range().into(),
        )),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_highlight_location(
    mut tree_cursor: GrzCursor<'_>,
    source: &ropey::Rope,
) -> Result<PCursor, ()> {
    tree_cursor.goto_first_child().or(Err(()))?;
    let value: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    let (line, column) = value.split_once(':').ok_or(())?;
    Ok(PCursor {
        paragraph: line.parse().or(Err(()))?,
        offset: column.parse().or(Err(()))?,
        prefer_next_row: true,
    })
}
