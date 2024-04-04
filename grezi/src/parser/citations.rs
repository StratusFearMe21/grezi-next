use std::path::Path;

use eframe::emath::Align2;
use egui_glyphon::glyphon::{Attrs, AttrsOwned, Color, Family};
use helix_core::{tree_sitter::Parser, Rope};

use crate::{
    layout::{Constraint, UnresolvedLayout},
    SlideShow,
};

use super::{slides::SlideObj, viewboxes::ViewboxIn, GrzCursor};
use num_enum::FromPrimitive;

include!(concat!(env!("OUT_DIR"), "/kinds_ntbib.rs"));

pub fn parse_citations(
    path: &Path,
    hasher: &ahash::RandomState,
    slideshow: &mut SlideShow,
) -> Result<(), super::Error> {
    if let Ok(file) = path
        .parent()
        .and_then(|p| Some(p.join(format!("{}_citations.html", path.file_name()?.to_str()?))))
        .ok_or(())
        .and_then(|p| std::fs::read_to_string(p).map_err(|_| ()))
    {
        if path.exists() {
            let mut parser = Parser::new();
            parser.set_language(&tree_sitter_ntbib::language()).unwrap();
            let find = file.find("<pre").unwrap_or_default();
            let file = &file[find..];
            let mut italics = false;
            let mut pre = false;
            let mut job = Vec::new();
            let mut header = "";
            let mut in_header = false;
            if let Some(tree) = parser.parse(file, None) {
                let t_r = Rope::new();
                let mut tree_cursor = GrzCursor::new(&tree, &t_r);

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
                                    if !job.is_empty() {
                                        job.push((
                                            "\n".to_owned(),
                                            AttrsOwned::new({
                                                let attrs = Attrs::new()
                                                    .color(Color::rgb(255, 255, 255))
                                                    .family(
                                                        egui_glyphon::glyphon::Family::SansSerif,
                                                    );
                                                if italics {
                                                    attrs
                                                        .style(egui_glyphon::glyphon::Style::Italic)
                                                } else {
                                                    attrs
                                                }
                                            })
                                            .into(),
                                        ));
                                        // height += 24.0 * 3.0;
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

                    if !(job.is_empty()
                        && file[tree_cursor.node().byte_range()]
                            .as_bytes()
                            .iter()
                            .all(|b| b.is_ascii_whitespace()))
                        && pre
                        && !in_header
                    {
                        job.push((
                            file[tree_cursor.node().byte_range()].to_owned(),
                            AttrsOwned::new({
                                let attrs = Attrs::new()
                                    .color(Color::rgb(255, 255, 255))
                                    .family(egui_glyphon::glyphon::Family::SansSerif);
                                if italics {
                                    attrs.style(egui_glyphon::glyphon::Style::Italic)
                                } else {
                                    attrs
                                }
                            })
                            .into(),
                        ));
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
            let mut iter = job.iter().rev();
            loop {
                if iter.next().unwrap().0 == "\n" {
                    pops += 1;
                } else {
                    break;
                }
            }

            for _ in 0..pops {
                job.pop();
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
                    object: super::objects::ObjectType::Text {
                        job,
                        font_size: 24.0,
                        line_height: Some(2.0),
                        align: None,
                    },
                },
            );
            slideshow.objects.insert(
                text_object_header,
                super::objects::Object {
                    position: None,
                    viewbox: None,
                    object: super::objects::ObjectType::Text {
                        job: vec![(
                            header.to_string(),
                            AttrsOwned::new(
                                Attrs::new()
                                    .family(Family::SansSerif)
                                    .color(Color::rgb(255, 255, 255)),
                            )
                            .into(),
                        )],
                        font_size: 48.0,
                        line_height: None,
                        align: None,
                    },
                },
            );
            slideshow.slide_show.push(super::AstObject::Slide {
                objects: vec![
                    SlideObj {
                        object: text_object,
                        locations: [
                            (Align2::LEFT_BOTTOM, ViewboxIn::Size),
                            (Align2::LEFT_TOP, ViewboxIn::Custom(vb, 1)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: super::objects::ObjectState::Entering,
                    },
                    SlideObj {
                        object: text_object_header,
                        locations: [
                            (Align2::CENTER_CENTER, ViewboxIn::Size),
                            (Align2::CENTER_CENTER, ViewboxIn::Custom(vb, 0)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: super::objects::ObjectState::Entering,
                    },
                ],
                actions: Vec::new(),
                bg: (super::color::Color::default(), None),
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
