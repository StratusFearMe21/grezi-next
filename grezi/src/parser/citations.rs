use std::{borrow::Cow, path::Path};

use eframe::emath::Align2;
use egui_glyphon::glyphon::{Attrs, AttrsOwned, Color, Family};
use helix_core::{tree_sitter::Parser, Rope};

use crate::{
    resolver::layout::{Constraint, Direction, UnresolvedLayout},
    SlideShow,
};

use super::{
    objects::cosmic_jotdown::{JotdownItem, RichText},
    slides::SlideObj,
    viewboxes::ViewboxIn,
    GrzCursor,
};
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
        let text_object_header = hasher.hash_one("__citation__text__object__header__");
        let vb = hasher.hash_one("__citation__vb__");

        if path.exists() {
            let mut parser = Parser::new();
            parser.set_language(&tree_sitter_ntbib::language()).unwrap();
            let find = file.find("<pre").unwrap_or_default();
            let file = &file[find..];
            let mut italics = false;
            let mut pre = false;
            let mut job = Vec::new();
            let mut header = Cow::Borrowed("");
            let mut in_header = false;
            let mut lines = 0;
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
                                p if p.starts_with("p style=") => in_header = true,
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
                                        lines += 1;
                                        job.push(RichText(
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
                                            }),
                                        ));
                                        if lines % 16 == 0 {
                                            let text_object = hasher
                                                .hash_one(("__citation__text__object__", lines));
                                            let mut finished_job = Vec::new();
                                            std::mem::swap(&mut job, &mut finished_job);

                                            add_to_slideshow(
                                                slideshow,
                                                text_object,
                                                text_object_header,
                                                vb,
                                                finished_job,
                                            );
                                        }
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
                        job.push(RichText(
                            html_escape::decode_html_entities(
                                &file[tree_cursor.node().byte_range()],
                            )
                            .into_owned(),
                            AttrsOwned::new({
                                let attrs = Attrs::new()
                                    .color(Color::rgb(255, 255, 255))
                                    .family(egui_glyphon::glyphon::Family::SansSerif);
                                if italics {
                                    attrs.style(egui_glyphon::glyphon::Style::Italic)
                                } else {
                                    attrs
                                }
                            }),
                        ));
                    } else if in_header && header.is_empty() {
                        header = html_escape::decode_html_entities(
                            &file[tree_cursor.node().byte_range()],
                        );
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

            let text_object = hasher.hash_one(("__citation__text__object__", lines));
            add_to_slideshow(slideshow, text_object, text_object_header, vb, job);
            slideshow.viewboxes.insert(
                vb,
                UnresolvedLayout {
                    direction: Direction::Vertical,
                    margin: 15.0,
                    constraints: vec![Constraint::Length(148.0), Constraint::Min(0.0)],
                    expand_to_fill: true,
                    split_on: super::viewboxes::ViewboxIn::Size,
                },
            );
            slideshow.objects.insert(
                text_object_header,
                super::objects::Object {
                    position: None,
                    viewbox: None,
                    object: super::objects::ObjectType::Text {
                        job: vec![JotdownItem::new_default(vec![RichText(
                            header.to_string(),
                            AttrsOwned::new(
                                Attrs::new()
                                    .family(Family::SansSerif)
                                    .color(Color::rgb(255, 255, 255)),
                            ),
                        )])],
                        font_size: 48.0,
                        line_height: None,
                        align: None,
                        spacing: super::objects::VerticalSpacing::Normal,
                    },
                },
            );
            Ok(())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn add_to_slideshow(
    slideshow: &mut SlideShow,
    text_object: u64,
    text_object_header: u64,
    vb: u64,
    mut job: Vec<RichText>,
) {
    if job.is_empty() {
        return;
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

    slideshow.objects.insert(
        text_object,
        super::objects::Object {
            position: None,
            viewbox: None,
            object: super::objects::ObjectType::Text {
                job: vec![JotdownItem::new_default(job)],
                font_size: 24.0,
                line_height: Some(2.0),
                align: None,
                spacing: super::objects::VerticalSpacing::Normal,
            },
        },
    );

    let mut key = u64::MAX;

    while slideshow.slide_show.contains_key(&key) {
        key -= 1;
    }

    slideshow.slide_show.insert(
        key,
        super::AstObject::Slide {
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
        },
    );
}
