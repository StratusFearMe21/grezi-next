use std::path::Path;

use ecolor::Color32;
use eframe::{
    egui::{Context, FontDefinitions, TextFormat},
    epaint::{
        text::{LayoutJob, TextWrapping},
        FontFamily, FontId,
    },
};
use helix_core::tree_sitter::Parser;
use indexmap::IndexSet;

use crate::{
    layout::{Constraint, UnresolvedLayout},
    SlideShow,
};

use super::{
    color::Color,
    objects::add_font,
    slides::SlideObj,
    viewboxes::{LineUp, ViewboxIn},
    GrzCursor,
};
use num_enum::FromPrimitive;

include!(concat!(env!("OUT_DIR"), "/kinds_ntbib.rs"));

pub fn parse_citations(
    path: &Path,
    ctx: &Context,
    hasher: &ahash::RandomState,
    slideshow: &mut SlideShow,
    fonts: &mut FontDefinitions,
    font_strings: &IndexSet<String, ahash::RandomState>,
) -> Result<(), super::Error> {
    if let Ok(file) = path
        .parent()
        .and_then(|p| Some(p.join(format!("{}_citations.html", path.file_name()?.to_str()?))))
        .ok_or(())
        .and_then(|p| std::fs::read_to_string(p).map_err(|_| ()))
    {
        if path.exists() {
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_ntbib::language()).unwrap();
            let find = file.find("<pre").unwrap_or_default();
            let file = &file[find..];
            let mut italics = false;
            let mut pre = false;
            let font = FontId::proportional(24.0);
            let mut layout = LayoutJob {
                wrap: TextWrapping {
                    max_rows: u32::MAX as usize,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut header = "";
            let mut in_header = false;
            let mut height = 0.0;
            let italic_family = FontFamily::Name("Ubuntu:light:italic".into());
            add_font(fonts, font_strings, ctx, &italic_family).unwrap();
            if let Some(tree) = parser.parse(file, None) {
                let mut tree_cursor = GrzCursor::new(&tree);

                tree_cursor.goto_first_child()?;

                'parserloop: loop {
                    if tree_cursor.node().kind_id() != NodeKind::Element as u16 {
                        break;
                    }

                    tree_cursor.goto_first_child()?;

                    match NodeKind::from(tree_cursor.node().kind_id()) {
                        NodeKind::TagStart => {
                            tree_cursor.goto_first_child()?;
                            match &file[tree_cursor.node().byte_range()] {
                                "pre" => pre = true,
                                "i" => italics = true,
                                "p style='text-align:center;'" => in_header = true,
                                _ => {}
                            }
                            tree_cursor.goto_parent();
                        }
                        NodeKind::TagEnd => {
                            tree_cursor.goto_first_child()?;
                            match &file[tree_cursor.node().byte_range()] {
                                "pre" => {
                                    pre = false;
                                    if !layout.is_empty() {
                                        layout.append(
                                            "\n",
                                            0.0,
                                            TextFormat {
                                                font_id: if italics {
                                                    FontId::new(24.0, italic_family.clone())
                                                } else {
                                                    font.clone()
                                                },
                                                line_height: Some(2.0),
                                                italics: false,
                                                ..Default::default()
                                            },
                                        );
                                        height += 24.0 * 3.0;
                                    }
                                }
                                "i" => italics = false,
                                "p" => in_header = false,
                                _ => {}
                            }
                            tree_cursor.goto_parent();
                        }
                        _ => break,
                    }

                    tree_cursor.goto_next_sibling()?;

                    if tree_cursor.node().kind_id() != NodeKind::Content as u16 {
                        break;
                    }

                    if !(layout.is_empty()
                        && file[tree_cursor.node().byte_range()]
                            .as_bytes()
                            .iter()
                            .all(|b| b.is_ascii_whitespace()))
                        && pre
                        && !in_header
                    {
                        layout.append(
                            &file[tree_cursor.node().byte_range()],
                            0.0,
                            TextFormat {
                                font_id: if italics {
                                    FontId::new(24.0, italic_family.clone())
                                } else {
                                    font.clone()
                                },
                                line_height: Some(2.0),
                                color: Color32::WHITE,
                                italics: false,
                                ..Default::default()
                            },
                        );
                    } else if in_header && header.is_empty() {
                        header = &file[tree_cursor.node().byte_range()];
                    }

                    tree_cursor.goto_parent();

                    loop {
                        match tree_cursor.goto_next_sibling() {
                            Ok(false) => break 'parserloop,
                            Ok(true) => break,
                            Err(e) => return Err(e),
                        }
                    }
                }
            }

            let mut pops = 0;
            let mut iter = layout.sections.iter().rev();
            loop {
                if &layout.text[iter.next().map(|r| r.byte_range.clone()).unwrap_or(0..1)] == "\n" {
                    layout.text.pop();
                    pops += 1;
                } else {
                    break;
                }
            }

            for _ in 0..pops {
                layout.sections.pop();
            }

            let text_object = hasher.hash_one("__citation__text__object__");
            let text_object_header = hasher.hash_one("__citation__text__object__header__");
            let vb = hasher.hash_one("__citation__vb__");
            slideshow.viewboxes.insert(
                vb,
                UnresolvedLayout {
                    direction: crate::layout::Direction::Vertical,
                    margin: 15.0,
                    constraints: vec![Constraint::Length(148.0), Constraint::Min(0.0)],
                    expand_to_fill: true,
                    split_on: super::viewboxes::ViewboxIn::Size,
                },
            );
            slideshow.objects.insert(
                text_object,
                super::objects::Object {
                    position: None,
                    viewbox: None,
                    source_obj: None,
                    object: super::objects::ObjectType::Text {
                        layout_job: layout,
                        source: false,
                    },
                },
            );
            slideshow.objects.insert(
                text_object_header,
                super::objects::Object {
                    position: None,
                    viewbox: None,
                    source_obj: None,
                    object: super::objects::ObjectType::Text {
                        layout_job: {
                            let mut job = LayoutJob::simple(
                                header.to_string(),
                                FontId::proportional(48.0),
                                Color32::WHITE,
                                f32::MAX,
                            );
                            job.wrap.max_rows = u32::MAX as usize;
                            job
                        },
                        source: false,
                    },
                },
            );
            slideshow.slide_show.push(super::AstObject::Slide {
                objects: vec![
                    SlideObj {
                        object: text_object,
                        locations: [
                            (LineUp::BottomLeft, ViewboxIn::Size),
                            (LineUp::TopLeft, ViewboxIn::Custom(vb, 1)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: super::objects::ObjectState::Entering,
                    },
                    SlideObj {
                        object: text_object_header,
                        locations: [
                            (LineUp::CenterCenter, ViewboxIn::Size),
                            (LineUp::CenterCenter, ViewboxIn::Custom(vb, 0)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: super::objects::ObjectState::Entering,
                    },
                ],
                actions: Vec::new(),
                bg: (Color::default(), None),
                max_time: 0.5,
                next: false,
            });
            Ok(())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
