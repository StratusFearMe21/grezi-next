use std::{
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
};

use eframe::epaint::{text::cursor::PCursor, Rect};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::{AstObject, NodeKind, PassThroughHasher};

#[derive(Debug, Serialize, Deserialize)]
pub enum Actions {
    Highlight {
        locations: Option<[PCursor; 2]>,
        index: usize,
        persist: bool,
    },
}

#[derive(Debug)]
pub enum ResolvedActions {
    Highlight { locations: Rect, persist: bool },
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_actions(
    tree_cursor: &mut GrzCursor<'_>,
    source: &str,
    hasher: &ahash::RandomState,
    on_screen: &HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    slide_in_ast: usize,
) -> AstObject {
    tree_cursor.goto_first_child();
    let mut actions = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::ActionObj as u16 {
        tree_cursor.goto_first_child();
        let object_name = {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(&source[tree_cursor.node().byte_range()], &mut hasher);
            hasher.finish()
        };
        let object = on_screen.get(&object_name).unwrap();
        tree_cursor.goto_next_sibling();
        let function_name = &source[tree_cursor.node().byte_range()];
        match function_name {
            "highlight" => {
                tree_cursor.goto_next_sibling();

                let locations = match NodeKind::from(tree_cursor.node().kind_id()) {
                    NodeKind::StringLiteral => {
                        tree_cursor.goto_first_child();
                        let value = &source[tree_cursor.node().byte_range()];
                        tree_cursor.goto_parent();
                        let (line, column) = value.split_once(':').unwrap();
                        let from = PCursor {
                            paragraph: line.parse().unwrap(),
                            offset: column.parse().unwrap(),
                            prefer_next_row: true,
                        };
                        tree_cursor.goto_next_sibling();
                        let to = match NodeKind::from(tree_cursor.node().kind_id()) {
                            NodeKind::StringLiteral => {
                                tree_cursor.goto_first_child();
                                let value = &source[tree_cursor.node().byte_range()];
                                tree_cursor.goto_parent();
                                let (line, column) = value.split_once(':').unwrap();
                                PCursor {
                                    paragraph: line.parse().unwrap(),
                                    offset: column.parse().unwrap(),
                                    prefer_next_row: true,
                                }
                            }
                            // "number_literal" => &source[tree_cursor.node().byte_range()],
                            _ => todo!(),
                        };
                        Some([from, to])
                    }
                    // "number_literal" => &source[tree_cursor.node().byte_range()],
                    _ => None,
                };
                actions.push(Actions::Highlight {
                    locations,
                    index: *object,
                    persist: false,
                });
            }
            _ => todo!(),
        }
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling();
    }
    tree_cursor.goto_parent();
    AstObject::Action {
        actions,
        slide_in_ast,
    }
}
