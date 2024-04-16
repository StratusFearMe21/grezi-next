use std::{
    fmt::Debug,
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use eframe::{
    egui::{Image, ImageFit, Ui},
    emath::Align2,
    epaint::{Color32, Pos2, Rect, Vec2},
};
use egui_glyphon::glyphon::{cosmic_text::Align, Attrs, Buffer, Family, Style, Weight};
use serde::{Deserialize, Serialize};

use self::cosmic_jotdown::ResolvedJotdownItem;

#[cfg(not(target_arch = "wasm32"))]
use super::{
    highlighting::{self, HelixCell},
    GrzCursor,
};
use super::{viewboxes::ViewboxIn, NodeKind};

pub mod cosmic_jotdown;
pub mod serde_suck;
use cosmic_jotdown::JotdownItem;
use serde_suck::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Object {
    pub position: Option<Align2>,
    pub viewbox: Option<ViewboxIn>,
    pub object: ObjectType,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum VerticalSpacing {
    Normal,
    Even,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Text {
        job: Vec<JotdownItem>,
        font_size: f32,
        line_height: Option<f32>,
        #[serde(with = "AlignSerde")]
        align: Option<Align>,
        spacing: VerticalSpacing,
    },
    Image {
        uri: String,
        bytes: Arc<[u8]>,
        scale: Option<Vec2>,
        tint: Color32,
    },
    Rect {
        color: Color32,
        height: f32,
    },
    Spinner,
}

#[derive(Clone)]
pub enum ResolvedObject {
    Text(Vec<ResolvedJotdownItem>),
    Image {
        image: Image<'static>,
        scale: Option<Vec2>,
        tint: Color32,
    },
    Anim {
        anim: egui_anim::Anim,
        scale: Option<Vec2>,
        tint: Color32,
    },
    Rect {
        color: Color32,
        rect: Rect,
    },
    Spinner,
}

pub fn measure_buffer(buffer: &Buffer, vb: Vec2) -> Rect {
    let mut rtl = false;
    let (width, total_lines) =
        buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                if run.rtl {
                    rtl = true;
                }
                (run.line_w.max(width), total_lines + 1)
            });

    let (max_width, max_height) = buffer.size();

    let size = Vec2::new(
        if rtl { vb.x } else { width.min(max_width) },
        (total_lines as f32 * buffer.metrics().line_height).min(max_height),
    );

    Rect::from_min_size(Pos2::ZERO, size)
}

impl ResolvedObject {
    pub fn bounds(&self, vb: Vec2, ui: &mut Ui) -> Rect {
        match self {
            ResolvedObject::Text(_) => unreachable!(),
            ResolvedObject::Image { image, .. } => {
                Rect::from_min_size(eframe::egui::pos2(0.0, 0.0), {
                    let mut size = None;
                    while size.is_none() {
                        size = image.load_and_calc_size(ui, vb);
                    }
                    size.unwrap()
                })
            }
            ResolvedObject::Anim { anim, .. } => {
                Rect::from_min_size(eframe::egui::pos2(0.0, 0.0), {
                    let mut size = None;
                    let image = Image::from_uri(anim.find_img(ui.ctx()));
                    while size.is_none() {
                        size = image.load_and_calc_size(ui, vb);
                    }
                    size.unwrap()
                })
            }
            ResolvedObject::Rect { rect, .. } => *rect,
            ResolvedObject::Spinner => Rect::from_min_size(
                eframe::egui::pos2(0.0, 0.0),
                ImageFit::Exact(vb).resolve(vb, Vec2::new(1.0, 1.0)),
            ),
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
    source: &helix_core::ropey::Rope,
    helix_cell: &mut Option<HelixCell>,
    hasher: &ahash::RandomState,
    ctx: &eframe::egui::Context,
    errors_present: &mut Vec<super::Error>,
    file_path: &std::path::Path,
    mut insert_fn: impl FnMut(u64, Object),
) -> Result<(), super::Error> {
    use std::borrow::Cow;

    use crate::parser::PointFromRange;

    use super::color::DefaultColorParser;
    use cssparser::ParserInput;
    use eframe::egui::load::Bytes;
    use helix_core::tree_sitter::{Node, Point, Range};
    use lsp_types::Url;

    tree_cursor.goto_first_child()?;
    let name = source.byte_slice(tree_cursor.node().byte_range());
    tree_cursor.goto_next_sibling()?;
    let obj_range = tree_cursor.node().range();
    let obj_type: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    tree_cursor.goto_next_sibling()?;
    tree_cursor.goto_first_child()?;
    let parameters = std::iter::from_fn(
        || -> Option<Result<(Cow<'_, str>, Node<'_>), super::Error>> {
            if tree_cursor.node().kind_id() == NodeKind::ObjParam as u16 {
                if let Err(e) = tree_cursor.goto_first_child() {
                    return Some(Err(e));
                }
                let key: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
                if let Err(e) = tree_cursor.goto_next_sibling() {
                    return Some(Err(e));
                }
                let value = tree_cursor.node();
                tree_cursor.goto_parent();
                if let Err(e) = tree_cursor.goto_next_sibling() {
                    return Some(Err(e));
                }
                Some(Ok((key, value)))
            } else {
                None
            }
        },
    );
    let object = match obj_type.as_ref() {
        "Rect" => {
            let mut tint = None;
            let mut height = None;
            for parameter in parameters {
                let parameter = parameter?;
                let value: Cow<'_, str> = source
                    .byte_slice(
                        parameter
                            .1
                            .child(1 /* second child */)
                            .unwrap_or(parameter.1)
                            .byte_range(),
                    )
                    .into();
                match parameter.0.as_ref() {
                    "color" => {
                        let t = super::color::parse_color_with(
                            &mut DefaultColorParser::new(Some(
                                &mut crate::parser::color::Color::LinSrgb(
                                    [1.0, 1.0, 1.0, 1.0].into(),
                                ),
                            )),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(
                                PointFromRange::new(parameter.1.range().into(), source),
                                format!("{:?}", e),
                            )
                        })?;

                        tint = Some(t.1.into());
                    }
                    "height" => {
                        height = Some(value.parse::<f32>().map_err(|_| {
                            super::Error::KnownMissing(
                                PointFromRange::new(parameter.1.range().into(), source),
                                "Valid floating point number".into(),
                            )
                        })?)
                    }
                    _ => {}
                }
            }
            ObjectType::Rect {
                color: tint.unwrap_or(Color32::WHITE),
                height: height.ok_or_else(|| {
                    super::Error::KnownMissing(
                        PointFromRange::new(obj_range.into(), source),
                        "Rectangle height".into(),
                    )
                })?,
            }
        }
        "Image" => {
            let mut uri = Cow::Borrowed("");
            let mut tint = None;
            let mut scale = None;
            for parameter in parameters {
                let parameter = parameter?;
                let value: Cow<'_, str> = source
                    .byte_slice(
                        parameter
                            .1
                            .child(1 /* second child */)
                            .unwrap_or(parameter.1)
                            .byte_range(),
                    )
                    .into();
                match parameter.0.as_ref() {
                    "value" => uri = value,
                    "scale" => {
                        let split = value.split_once('x').ok_or_else(|| {
                            super::Error::InvalidParameter(PointFromRange::new(
                                parameter.1.range().into(),
                                source,
                            ))
                        })?;

                        let w: f32 = split.0.parse().map_err(|_| {
                            super::Error::InvalidParameter(PointFromRange::new(
                                parameter.1.range().into(),
                                source,
                            ))
                        })?;

                        let h: f32 = split.1.parse().map_err(|_| {
                            super::Error::InvalidParameter(PointFromRange::new(
                                parameter.1.range().into(),
                                source,
                            ))
                        })?;

                        scale = Some(Vec2::new(w, h));
                    }
                    "tint" => {
                        let t = super::color::parse_color_with(
                            &mut DefaultColorParser::new(Some(
                                &mut crate::parser::color::Color::LinSrgb(
                                    [1.0, 1.0, 1.0, 1.0].into(),
                                ),
                            )),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(
                                PointFromRange::new(parameter.1.range().into(), source),
                                format!("{:?}", e),
                            )
                        })?;

                        tint = Some(t.1.into());
                    }
                    _ => {}
                }
            }
            if uri.is_empty() {
                return Err(super::Error::KnownMissing(
                    PointFromRange::new(obj_range.into(), source),
                    "value".into(),
                ));
            }
            if let Ok(mut new_uri) = Url::parse(&uri) {
                if new_uri.scheme() == "file" {
                    new_uri.set_path(
                        file_path
                            .parent()
                            .unwrap()
                            .join(&new_uri.path()[1..])
                            .as_os_str()
                            .to_str()
                            .unwrap(),
                    );
                    uri = Cow::Owned(new_uri.to_string());
                }
            }
            let bytes = loop {
                match ctx.try_load_bytes(&uri) {
                    Ok(poll) => match poll {
                        eframe::egui::load::BytesPoll::Ready { bytes, .. } => {
                            break match bytes {
                                Bytes::Static(s) => s.into(),
                                Bytes::Shared(b) => Arc::clone(&b),
                            }
                        }
                        eframe::egui::load::BytesPoll::Pending { .. } => {}
                    },
                    Err(e) => {
                        return Err(super::Error::ImageError(
                            PointFromRange::new(tree_cursor.node().range().into(), source),
                            e,
                        ))
                    }
                }
            };
            ObjectType::Image {
                uri: uri.into_owned(),
                bytes,
                scale,
                tint: tint.unwrap_or(Color32::WHITE),
            }
        }
        "Paragraph" | "Header" => {
            let mut text = None;
            let mut line_height = None;
            let mut align = None;
            let mut spacing = VerticalSpacing::Normal;
            let mut font = (
                Cow::Borrowed(""),
                Attrs::new(),
                Range {
                    start_byte: 0,
                    end_byte: 0,
                    start_point: Point::default(),
                    end_point: Point::default(),
                },
            );
            let mut font_size = match obj_type.as_ref() {
                "Paragraph" => 48.0,
                "Header" => 64.0,
                _ => unreachable!(),
            };
            let mut language = None;
            for parameter in parameters {
                let parameter = parameter?;
                let mut value: Cow<'_, str> = source
                    .byte_slice(
                        parameter
                            .1
                            .child(1 /* second child */)
                            .unwrap_or(parameter.1)
                            .byte_range(),
                    )
                    .into();
                match parameter.0.as_ref() {
                    "value" | "code" => text = Some(parameter.1),
                    "align" => match value.as_ref() {
                        "left" | "Left" => align = Some(Align::Left),
                        "center" | "Center" => align = Some(Align::Center),
                        "right" | "Right" => align = Some(Align::Right),
                        "justified" | "Justified" => align = Some(Align::Justified),
                        "end" | "End" => align = Some(Align::End),
                        _ => {
                            errors_present.push(super::Error::InvalidParameter(
                                PointFromRange::new(parameter.1.range().into(), source),
                            ));
                            continue;
                        }
                    },
                    // "color" => {
                    //     let c = super::color::parse_color_with(
                    //         &mut DefaultColorParser::new(Some(
                    //             &mut crate::parser::color::Color::LinSrgb(
                    //                 [1.0, 1.0, 1.0, 1.0].into(),
                    //             ),
                    //         )),
                    //         &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                    //     )
                    //     .map_err(|e| {
                    //         super::Error::ColorError(
                    //             PointFromRange::new(parameter.1.range().into(), source),
                    //             format!("{:?}", e),
                    //         )
                    //     })?;

                    //     color = Some(c.1.into());
                    // }
                    "vertical_spacing" => match value.as_ref() {
                        "normal" => spacing = VerticalSpacing::Normal,
                        "even" => spacing = VerticalSpacing::Even,
                        _ => {
                            errors_present.push(super::Error::InvalidParameter(
                                PointFromRange::new(parameter.1.range().into(), source),
                            ));
                            continue;
                        }
                    },
                    "font_family" => {
                        {
                            core::mem::swap(&mut value, &mut font.0);
                        }
                        let query =
                            fontstr_to_query(unsafe { std::mem::transmute(font.0.as_ref()) });

                        font.1 = query;
                        font.2 = parameter.1.range();
                    }
                    "font_size" => {
                        font_size = value.parse::<f32>().map_err(|_| {
                            super::Error::KnownMissing(
                                PointFromRange::new(parameter.1.range().into(), source),
                                "Valid floating point number".into(),
                            )
                        })?
                    }
                    "line_height" => {
                        line_height = Some(value.parse::<f32>().map_err(|_| {
                            super::Error::KnownMissing(
                                PointFromRange::new(parameter.1.range().into(), source),
                                "Valid floating point number".into(),
                            )
                        })?);
                    }
                    "language" => {
                        language = Some((
                            value,
                            parameter
                                .1
                                .child(1 /* second child */)
                                .ok_or_else(|| {
                                    super::Error::KnownMissing(
                                        PointFromRange::new(parameter.1.range().into(), source),
                                        "Valid string containing a programming language".into(),
                                    )
                                })?
                                .range(),
                        ))
                    }
                    _ => {}
                }
            }
            let text = if let Some(t) = text {
                t
            } else {
                return Err(super::Error::KnownMissing(
                    PointFromRange::new(obj_range.into(), source),
                    "value".into(),
                ));
            };
            let job: Vec<JotdownItem> = match language {
                Some(lang) => vec![highlighting::highlight_text(
                    text,
                    lang,
                    font.1.clone(),
                    helix_cell,
                    source,
                    hasher,
                )?],
                _ => {
                    let mut walker = GrzCursor::from_node(text, source);
                    walker.goto_first_child()?;

                    let mut string_content = String::new();

                    loop {
                        match NodeKind::from(walker.node().kind_id()) {
                            NodeKind::StringContent | NodeKind::RawStringContent => {
                                source
                                    .byte_slice(walker.node().byte_range())
                                    .chunks()
                                    .for_each(|chunk| string_content.push_str(chunk));
                            }
                            NodeKind::EscapeSequence => {
                                source
                                    .byte_slice(
                                        walker.node().byte_range().start + 1
                                            ..walker.node().byte_range().end,
                                    )
                                    .chunks()
                                    .for_each(|chunk| string_content.push_str(chunk));
                            }
                            _ => break,
                        }
                        walker.goto_next_sibling()?;
                    }

                    cosmic_jotdown::jotdown_into_buffers(jotdown::Parser::new(&string_content))
                        .collect()
                }
            };
            ObjectType::Text {
                job,
                font_size,
                line_height,
                align,
                spacing,
            }
        }
        _ => {
            return Err(super::Error::NotFound(PointFromRange::new(
                obj_range.into(),
                source,
            )))
        }
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();

    insert_fn(
        {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(&name, &mut hasher);
            hasher.finish()
        },
        Object {
            position: None,
            viewbox: None,
            object,
        },
    );

    Ok(())
}

pub fn fontstr_to_query<'a>(family: &'a str) -> Attrs<'a> {
    let mut query = Attrs::new();

    let mut split = family.split(':');
    let base = split.next().unwrap();

    match base {
        "serif" => query.family = Family::Serif,
        "sans-serif" => query.family = Family::SansSerif,
        "cursive" => query.family = Family::Cursive,
        "fantasy" => query.family = Family::Fantasy,
        "monospace" => query.family = Family::Monospace,
        name => query.family = Family::Name(name),
    }

    for s in split {
        match s {
            "normal" => query.style = Style::Normal,
            "italic" => query.style = Style::Italic,
            "oblique" => query.style = Style::Oblique,
            // Thin weight (100), the thinnest value.
            "thin" => query.weight = Weight::THIN,
            // Extra light weight (200).
            "extra_light" => query.weight = Weight::EXTRA_LIGHT,
            // Light weight (300).
            "light" => query.weight = Weight::LIGHT,
            // Normal (400).
            "normal" => query.weight = Weight::NORMAL,
            // Medium weight (500, higher than normal).
            "medium" => query.weight = Weight::MEDIUM,
            // Semibold weight (600).
            "semibold" => query.weight = Weight::SEMIBOLD,
            // Bold weight (700).
            "bold" => query.weight = Weight::BOLD,
            // Extra-bold weight (800).
            "extra_bold" => query.weight = Weight::EXTRA_BOLD,
            // Black weight (900), the thickest value.
            "black" => query.weight = Weight::BLACK,
            _ => {}
        }
    }

    query
}
