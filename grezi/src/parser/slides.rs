use std::{
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    str::FromStr,
};

use eframe::epaint::text::cursor::PCursor;

use super::{
    actions::Actions,
    objects::{Object, ObjectState, ResolvedObject},
    viewboxes::{LineUp, ViewboxIn},
    AstObject, FieldName, GrzCursor, NodeKind, PassThroughHasher,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SlideObj {
    pub object: u64,
    pub locations: [(LineUp, ViewboxIn); 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

#[derive(Debug)]
pub struct ResolvedSlideObj {
    pub object: ResolvedObject,
    pub locations: [[f32; 2]; 2],
    pub scaled_time: [f32; 2],
    pub state: ObjectState,
}

pub fn parse_slides(
    tree_cursor: &mut GrzCursor<'_>,
    hasher: &ahash::RandomState,
    on_screen: &mut HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>>,
    objects: &mut HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
    source: &str,
) -> AstObject {
    tree_cursor.goto_first_child();
    tree_cursor.goto_first_child();
    let mut slide_objects = Vec::new();
    let mut slide_on_screen: HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    while tree_cursor.field_id() == Some(FieldName::Objects as u16) {
        tree_cursor.goto_first_child();
        let object_name = {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(&source[tree_cursor.node().byte_range()], &mut hasher);
            hasher.finish()
        };
        let object = { objects.get_mut(&object_name).unwrap() };
        tree_cursor.goto_next_sibling();
        let viewbox = &source[tree_cursor.node().byte_range()];
        let viewbox_node = NodeKind::from(tree_cursor.node().kind_id());
        tree_cursor.goto_next_sibling();
        tree_cursor.goto_first_child();
        let vb_index: usize = source[tree_cursor.node().byte_range()].parse().unwrap();
        let viewbox = match viewbox_node {
            NodeKind::Size => ViewboxIn::Size,
            NodeKind::Identifier => {
                let mut hasher = hasher.build_hasher();
                std::hash::Hash::hash(viewbox, &mut hasher);
                ViewboxIn::Custom(hasher.finish(), vb_index)
            }
            _ => todo!(),
        };
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling();
        let from: Option<ViewboxIn>;
        match NodeKind::from(tree_cursor.node().kind_id()) {
            NodeKind::SlideFrom => {
                tree_cursor.goto_first_child();
                let viewbox = &source[tree_cursor.node().byte_range()];
                let viewbox_node = NodeKind::from(tree_cursor.node().kind_id());
                tree_cursor.goto_next_sibling();
                tree_cursor.goto_first_child();
                let vb_index: usize = source[tree_cursor.node().byte_range()].parse().unwrap();
                tree_cursor.goto_parent();
                tree_cursor.goto_parent();
                tree_cursor.goto_next_sibling();
                from = match viewbox_node {
                    NodeKind::Size => Some(ViewboxIn::Size),
                    NodeKind::Identifier => {
                        let mut hasher = hasher.build_hasher();
                        std::hash::Hash::hash(viewbox, &mut hasher);
                        Some(ViewboxIn::Custom(hasher.finish(), vb_index))
                    }
                    _ => todo!(),
                };
            }
            _ => from = None,
        }

        let mut state = if on_screen.contains_key(&object_name) {
            ObjectState::OnScreen
        } else {
            ObjectState::Entering
        };

        let (locations, line_up_now) = {
            let edges = &source[tree_cursor.node().byte_range()];

            let lineup_first;
            let viewbox_first = from.unwrap_or_else(|| object.viewbox.unwrap_or(viewbox));
            let line_up_now;
            if &edges[..1] == "[" {
                lineup_first = object.position.unwrap();
                line_up_now = object.position.unwrap();
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
                        lineup_first = object.position.unwrap();
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
                                let lineup = object.position.unwrap();
                                (lineup, viewbox_first)
                            };
                            (lineup_first, viewbox)
                        }
                        _ => {
                            line_up_now = object.position.unwrap();
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
        slide_on_screen.insert(object_name, slide_objects.len());
        slide_objects.push(SlideObj {
            object: object_name,
            locations,
            state,
            scaled_time: [0.0, 0.5],
        });
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling();
    }
    tree_cursor.goto_parent();
    tree_cursor.goto_next_sibling();
    tree_cursor.goto_first_child();
    // Draws Entering objects first, then OnScreen, then Exiting
    slide_objects.sort_by_key(|obj| obj.state);
    for (index, slide_object) in slide_objects.iter().enumerate() {
        *slide_on_screen.get_mut(&slide_object.object).unwrap() = index;
    }
    let mut max_time = 1.0;
    let mut actions = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
        tree_cursor.goto_first_child();
        match &source[tree_cursor.node().byte_range()] {
            "stagger" => {
                tree_cursor.goto_next_sibling();
                let scaler: f32 = source[tree_cursor.node().byte_range()].parse().unwrap();
                let mut min_time = 0.0;
                for object in slide_objects.iter_mut().skip(1) {
                    max_time += scaler;
                    min_time += scaler;
                    object.scaled_time[0] = min_time;
                }
            }
            "time" => {
                tree_cursor.goto_next_sibling();
                let time: f32 = (source[tree_cursor.node().byte_range()]
                    .parse::<f32>()
                    .unwrap()
                    - 0.5)
                    .abs();
                max_time += time;
                for object in slide_objects.iter_mut() {
                    object.scaled_time[1] += time;
                }
            }
            "highlight" => {
                tree_cursor.goto_next_sibling();
                let object = slide_on_screen
                    .get({
                        let mut hasher = hasher.build_hasher();
                        std::hash::Hash::hash(
                            &source[tree_cursor.node().byte_range()],
                            &mut hasher,
                        );
                        &hasher.finish()
                    })
                    .unwrap();
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
                    persist: true,
                });
            }
            _ => todo!(),
        }
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling();
    }
    core::mem::swap(&mut slide_on_screen, on_screen);
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    AstObject::Slide {
        objects: slide_objects,
        actions,
        max_time,
    }
}
