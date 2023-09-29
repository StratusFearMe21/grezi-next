use std::{
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use eframe::{
    egui::TextFormat,
    emath::Align,
    epaint::{
        text::{LayoutJob, TextWrapping},
        Color32, FontFamily, FontId, Galley, Rect, Stroke, Vec2,
    },
};
use pulldown_cmark::Tag;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use super::{
    highlighting::{self, HelixCell},
    GrzCursor,
};
use super::{
    viewboxes::{LineUp, ViewboxIn},
    FieldName, NodeKind,
};

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub position: Option<LineUp>,
    pub viewbox: Option<ViewboxIn>,
    pub object: ObjectType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Text { layout_job: LayoutJob },
}

#[derive(Debug)]
pub enum ResolvedObject {
    Text(Arc<Galley>),
}

impl ResolvedObject {
    pub fn bounds(&self) -> Rect {
        match self {
            ResolvedObject::Text(galley) => galley
                .rect
                .translate(Vec2::new(-galley.rect.min.x, -galley.rect.min.y))
                .expand(1.0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Serialize)]
pub enum ObjectState {
    Entering = 2,
    OnScreen = 1,
    Exiting = 0,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_objects(
    tree_cursor: &mut GrzCursor<'_>,
    source: &str,
    helix_cell: &mut Option<HelixCell>,
    hasher: &ahash::RandomState,
) -> (u64, Object) {
    tree_cursor.goto_first_child();
    let name = &source[tree_cursor.node().byte_range()];
    tree_cursor.goto_next_sibling();
    let obj_type = &source[tree_cursor.node().byte_range()];
    tree_cursor.goto_next_sibling();
    tree_cursor.goto_first_child();
    let parameters = std::iter::from_fn(|| {
        if tree_cursor.field_id() == Some(FieldName::Parameters as u16) {
            let key = &source[tree_cursor.node().byte_range()];
            tree_cursor.goto_next_sibling();
            let value = match NodeKind::from(tree_cursor.node().kind_id()) {
                NodeKind::StringLiteral => {
                    let value = tree_cursor.node();
                    value
                }
                NodeKind::NumberLiteral => tree_cursor.node(),
                _ => todo!(),
            };
            tree_cursor.goto_next_sibling();
            Some((key, value))
        } else {
            None
        }
    });
    let object = match obj_type {
        "Paragraph" | "Header" => {
            let mut text = None;
            let mut align = Align::LEFT;
            let mut font = FontFamily::Proportional;
            let mut font_size = match obj_type {
                "Paragraph" => 48.0,
                "Header" => 64.0,
                _ => unreachable!(),
            };
            let mut language = None;
            for parameter in parameters {
                let value = &source[parameter
                    .1
                    .child(1 /* second child */)
                    .unwrap()
                    .byte_range()];
                match parameter.0 {
                    "value" => text = Some(parameter.1),
                    "align" => match value {
                        "left" | "Left" => align = Align::LEFT,
                        "center" | "Center" => align = Align::Center,
                        "right" | "Right" => align = Align::RIGHT,
                        _ => todo!(),
                    },
                    "font_family" => font = FontFamily::Name(value.into()),
                    "font_size" => font_size = value.parse::<f32>().unwrap(),
                    "language" => language = Some(value),
                    _ => {}
                }
            }
            let text = text.unwrap();
            let layout_job = match language {
                Some(lang) => highlighting::highlight_text(
                    text,
                    lang,
                    align,
                    FontId::new(font_size, font.clone()),
                    helix_cell,
                    source,
                    hasher,
                ),
                _ => {
                    let mut layout_job = LayoutJob {
                        halign: align,
                        break_on_newline: true,
                        wrap: TextWrapping {
                            max_width: 0.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    let mut walker = GrzCursor::from_node(&text);
                    walker.goto_first_child();
                    let mut options = pulldown_cmark::Options::empty();
                    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);

                    let mut string_content = String::new();

                    loop {
                        match NodeKind::from(walker.node().kind_id()) {
                            NodeKind::StringContent => {
                                string_content.push_str(&source[walker.node().byte_range()]);
                            }
                            NodeKind::EscapeSequence => {
                                string_content.push_str(
                                    &source[walker.node().byte_range().start + 1
                                        ..walker.node().byte_range().end],
                                );
                            }
                            _ => break,
                        }
                        walker.goto_next_sibling();
                    }

                    let parser = pulldown_cmark::Parser::new_ext(&string_content, options);

                    let mut tags = Vec::new();

                    for event in parser {
                        match event {
                            pulldown_cmark::Event::Start(tag) => {
                                tags.push(tag);
                            }
                            pulldown_cmark::Event::End(_) => {
                                tags.pop();
                            }
                            pulldown_cmark::Event::Code(text) => {
                                layout_job.append(
                                    text.as_ref(),
                                    0.0,
                                    TextFormat {
                                        font_id: FontId::monospace(font_size),
                                        color: Color32::WHITE,
                                        background: Color32::TRANSPARENT,
                                        italics: false,
                                        underline: Stroke::NONE,
                                        strikethrough: Stroke::NONE,
                                        ..Default::default()
                                    },
                                );
                            }
                            pulldown_cmark::Event::Text(text) => {
                                layout_job.append(
                                    text.as_ref(),
                                    0.0,
                                    TextFormat {
                                        font_id: FontId::new(font_size, font.clone()),
                                        color: Color32::WHITE,
                                        background: Color32::TRANSPARENT,
                                        italics: tags.contains(&Tag::Emphasis),
                                        underline: Stroke::NONE,
                                        strikethrough: if tags.contains(&Tag::Strikethrough) {
                                            Stroke::new(3.0, Color32::WHITE)
                                        } else {
                                            Stroke::NONE
                                        },
                                        ..Default::default()
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    layout_job
                }
            };
            ObjectType::Text { layout_job }
        }
        _ => todo!(),
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    (
        {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(name, &mut hasher);
            hasher.finish()
        },
        Object {
            position: None,
            viewbox: None,
            object,
        },
    )
}
