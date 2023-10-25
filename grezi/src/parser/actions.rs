use std::{
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
};

use eframe::epaint::{text::cursor::PCursor, Rect};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::{AstObject, NodeKind, PassThroughHasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Actions {
    Highlight {
        locations: Option<[PCursor; 2]>,
        index: usize,
        persist: bool,
    },
}

#[derive(Debug, Clone)]
pub enum ResolvedActions {
    Highlight { locations: Rect, persist: bool },
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_actions(
    mut tree_cursor: GrzCursor<'_>,
    source: &ropey::Rope,
    hasher: &ahash::RandomState,
    on_screen: &HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    slide_in_ast: usize,
    errors_present: &mut Vec<super::Error>,
) -> Result<AstObject, super::Error> {
    tree_cursor.goto_first_child()?;
    let mut actions = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::ActionObj as u16 {
        match parse_single_action(tree_cursor.fork(), source, hasher, on_screen) {
            Ok(action) => actions.push(action),
            Err(e) => errors_present.push(e),
        }
        tree_cursor.goto_next_sibling()?;
    }
    tree_cursor.goto_parent();
    Ok(AstObject::Action {
        actions,
        slide_in_ast,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_single_action(
    mut action_walker: GrzCursor<'_>,
    source: &ropey::Rope,
    hasher: &ahash::RandomState,
    on_screen: &HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
) -> Result<Actions, super::Error> {
    action_walker.goto_first_child()?;
    let object_name = {
        let mut hasher = hasher.build_hasher();
        std::hash::Hash::hash(
            &source.byte_slice(action_walker.node().byte_range()),
            &mut hasher,
        );
        hasher.finish()
    };
    let object = on_screen
        .get(&object_name)
        .ok_or_else(|| super::Error::NotFound(action_walker.node().range().into()))?;
    action_walker.goto_next_sibling()?;
    let function_name = source.byte_slice(action_walker.node().byte_range());

    if function_name == "highlight" {
        action_walker.goto_next_sibling()?;

        let locations = match NodeKind::from(action_walker.node().kind_id()) {
            NodeKind::StringLiteral => {
                let from =
                    parse_highlight_location(action_walker.fork(), source).map_err(|_| {
                        super::Error::InvalidParameter(action_walker.node().range().into())
                    })?;
                action_walker.goto_next_sibling()?;
                let to = match NodeKind::from(action_walker.node().kind_id()) {
                    NodeKind::StringLiteral => {
                        parse_highlight_location(action_walker.fork(), source).map_err(|_| {
                            super::Error::InvalidParameter(action_walker.node().range().into())
                        })?
                    }
                    // "number_literal" => &source[tree_cursor.node().byte_range()],
                    _ => {
                        return Err(super::Error::InvalidParameter(
                            action_walker.node().range().into(),
                        ))
                    }
                };
                Some([from, to])
            }
            // "number_literal" => &source[tree_cursor.node().byte_range()],
            _ => None,
        };
        Ok(Actions::Highlight {
            locations,
            index: *object,
            persist: false,
        })
    } else {
        return Err(super::Error::ActionNotFound(
            action_walker.node().range().into(),
        ));
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_highlight_location(
    mut tree_cursor: GrzCursor<'_>,
    source: &ropey::Rope,
) -> Result<PCursor, ()> {
    use std::borrow::Cow;

    tree_cursor.goto_first_child().or(Err(()))?;
    let value: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    let (line, column) = value.split_once(':').ok_or(())?;
    Ok(PCursor {
        paragraph: line.parse().or(Err(()))?,
        offset: column.parse().or(Err(()))?,
        prefer_next_row: true,
    })
}
