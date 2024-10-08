use std::{
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    sync::Arc,
};

use ecolor::Color32;
use eframe::{emath::Align2, epaint::Rect};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::{objects::ObjectState, AstObject, NodeKind, PassThroughHasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Actions {
    Highlight {
        locations: Option<[[usize; 2]; 2]>,
        index: usize,
        persist: bool,
        color: Color32,
    },
    Line {
        objects: [usize; 2],
        locations: [Align2; 2],
        color: Color32,
    },
    SpeakerNotes(Arc<str>),
}

#[derive(Debug, Clone)]
pub enum ResolvedActions {
    Highlight {
        locations: Rect,
        persist: bool,
        locations_of_object: [[f32; 2]; 2],
        scaled_time: [f32; 2],
        color: Color32,
    },
    Line {
        locations_of_objects: [[[f32; 2]; 2]; 2],
        scaled_times: [[f32; 2]; 2],
        color: Color32,
        state: ObjectState,
        scale: f32,
    },
    SpeakerNotes(Arc<str>),
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_actions(
    tree_cursor: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
    hasher: &ahash::RandomState,
    on_screen: &HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>>,
    slide_in_ast: u64,
    errors_present: &mut Vec<super::Error>,
) -> Result<AstObject, super::Error> {
    tree_cursor.goto_first_child()?;
    let mut actions = Vec::new();
    let mut next = false;
    while tree_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
        tree_cursor.fork(|cursor| {
            match parse_single_action(cursor, source, hasher, on_screen, &mut next) {
                Ok(Some(action)) => actions.push(action),
                Err(e) => errors_present.push(e),
                _ => {}
            }
        });
        tree_cursor.goto_next_sibling()?;
    }
    tree_cursor.goto_parent();
    Ok(AstObject::Action {
        actions,
        slide_in_ast,
        next,
    })
}

pub const HIGHLIGHT_COLOR_DEFAULT: Color32 = {
    let color = Color32::LIGHT_YELLOW;
    Color32::from_rgba_premultiplied(
        (color.r() as f32 * 0.5 + 0.5) as u8,
        (color.g() as f32 * 0.5 + 0.5) as u8,
        (color.b() as f32 * 0.5 + 0.5) as u8,
        (color.a() as f32 * 0.5 + 0.5) as u8,
    )
};

#[cfg(not(target_arch = "wasm32"))]
fn parse_single_action(
    action_walker: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
    hasher: &ahash::RandomState,
    on_screen: &HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>>,
    next: &mut bool,
) -> Result<Option<Actions>, super::Error> {
    use std::borrow::Cow;

    use cssparser::ParserInput;

    use crate::parser::PointFromRange;

    use super::color::DefaultColorParser;

    action_walker.goto_first_child()?;
    let function_name = source.byte_slice(action_walker.node().byte_range());

    if function_name == "highlight" {
        action_walker.goto_next_sibling()?;
        let object_name = {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(
                &source.byte_slice(action_walker.node().byte_range()),
                &mut hasher,
            );
            hasher.finish()
        };
        let object = on_screen.get(&object_name).ok_or_else(|| {
            super::Error::NotFound(PointFromRange::new(
                action_walker.node().range().into(),
                source,
            ))
        })?;
        action_walker.goto_next_sibling()?;

        let locations = match NodeKind::from(action_walker.node().kind_id()) {
            NodeKind::StringLiteral => {
                let from = action_walker
                    .fork(|cursor| parse_highlight_location(cursor, source))
                    .map_err(|_| {
                        super::Error::InvalidParameter(PointFromRange::new(
                            action_walker.node().range().into(),
                            source,
                        ))
                    })?;
                action_walker.goto_next_sibling()?;
                let to = match NodeKind::from(action_walker.node().kind_id()) {
                    NodeKind::StringLiteral => action_walker
                        .fork(|cursor| parse_highlight_location(cursor, source))
                        .map_err(|_| {
                            super::Error::InvalidParameter(PointFromRange::new(
                                action_walker.node().range().into(),
                                source,
                            ))
                        })?,
                    // "number_literal" => &source[tree_cursor.node().byte_range()],
                    _ => {
                        return Err(super::Error::InvalidParameter(PointFromRange::new(
                            action_walker.node().range().into(),
                            source,
                        )))
                    }
                };
                match (from, to) {
                    (Some(from), Some(to)) => Some([from, to]),
                    _ => None,
                }
            }
            // "number_literal" => &source[tree_cursor.node().byte_range()],
            _ => None,
        };
        action_walker.goto_next_sibling()?;
        let color: Color32 = match NodeKind::from(action_walker.node().kind_id()) {
            NodeKind::StringLiteral => {
                let value: Cow<'_, str> = source
                    .byte_slice(
                        action_walker
                            .node()
                            .child(1 /* second child */)
                            .unwrap_or(action_walker.node())
                            .byte_range(),
                    )
                    .into();
                let t = super::color::parse_color_with(
                    &mut DefaultColorParser::new(Some(&mut HIGHLIGHT_COLOR_DEFAULT.into())),
                    &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                )
                .map_err(|e| {
                    super::Error::ColorError(
                        PointFromRange::new(action_walker.node().range().into(), source),
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
            persist: false,
            color,
        }))
    } else if function_name == "speaker_notes" {
        action_walker.goto_next_sibling()?;
        Ok(Some(Actions::SpeakerNotes(
            source
                .byte_slice(action_walker.node().byte_range())
                .to_string()
                .into(),
        )))
    } else if function_name == "next" {
        *next = true;
        Ok(None)
    } else {
        return Err(super::Error::ActionNotFound(PointFromRange::new(
            action_walker.node().range().into(),
            source,
        )));
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_highlight_location(
    tree_cursor: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
) -> Result<Option<[usize; 2]>, ()> {
    use std::borrow::Cow;

    if tree_cursor.goto_first_child().or(Err(()))? {
        let value: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
        let (line, column) = value.split_once(':').ok_or(())?;
        Ok(Some([
            line.parse().or(Err(()))?,
            column.parse().or(Err(()))?,
        ]))
    } else {
        Ok(None)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn id(walker: &mut GrzCursor<'_>) -> Result<u64, super::Error> {
    walker.goto_first_child()?;
    let id = walker.node().id() as u64;
    walker.goto_parent();
    Ok(id)
}
