use std::{
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use eframe::{
    egui::{Image, TextFormat, Ui},
    emath::Align,
    epaint::{
        text::{LayoutJob, TextWrapping},
        Color32, FontFamily, FontId, Galley, Rect, Stroke, Vec2,
    },
};
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Object {
    pub position: Option<LineUp>,
    pub viewbox: Option<ViewboxIn>,
    pub object: ObjectType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Text {
        layout_job: LayoutJob,
    },
    Image {
        uri: String,
        bytes: Arc<[u8]>,
        scale: Option<Vec2>,
        tint: Color32,
    },
}

#[derive(Debug, Clone)]
pub enum ResolvedObject {
    Text(Arc<Galley>),
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
}

impl ResolvedObject {
    pub fn bounds(&self, vb: Vec2, ui: &mut Ui) -> Rect {
        match self {
            ResolvedObject::Text(galley) => galley
                .rect
                .translate(Vec2::new(-galley.rect.min.x, -galley.rect.min.y))
                .expand(1.0),
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
) -> Result<(u64, Object), super::Error> {
    use std::borrow::Cow;

    use eframe::egui::load::Bytes;
    use helix_core::tree_sitter::Node;
    use lsp_types::Url;
    use pulldown_cmark::Tag;

    tree_cursor.goto_first_child()?;
    let name = source.byte_slice(tree_cursor.node().byte_range());
    tree_cursor.goto_next_sibling()?;
    let obj_range = tree_cursor.node().range();
    let obj_type: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    tree_cursor.goto_next_sibling()?;
    tree_cursor.goto_first_child()?;
    let parameters = std::iter::from_fn(
        || -> Option<Result<(Cow<'_, str>, Node<'_>), super::Error>> {
            if tree_cursor.field_id() == Some(FieldName::Parameters as u16) {
                let key: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
                if let Err(e) = tree_cursor.goto_next_sibling() {
                    return Some(Err(e));
                }
                let value = match NodeKind::from(tree_cursor.node().kind_id()) {
                    NodeKind::StringLiteral => {
                        let value = tree_cursor.node();
                        value
                    }
                    NodeKind::NumberLiteral => tree_cursor.node(),
                    _ => {
                        return Some(Err(super::Error::InvalidParameter(
                            tree_cursor.node().range().into(),
                        )))
                    }
                };
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
        "Image" => {
            let mut uri = String::new();
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
                    "value" => uri = value.into_owned(),
                    "scale" => {
                        let split = value.split_once('x').ok_or_else(|| {
                            super::Error::InvalidParameter(parameter.1.range().into())
                        })?;

                        let w: f32 = split.0.parse().map_err(|_| {
                            super::Error::InvalidParameter(parameter.1.range().into())
                        })?;

                        let h: f32 = split.1.parse().map_err(|_| {
                            super::Error::InvalidParameter(parameter.1.range().into())
                        })?;

                        scale = Some(Vec2::new(w, h));
                    }
                    "tint" => {
                        let t = value
                            .parse::<csscolorparser::Color>()
                            .map_err(|_| {
                                super::Error::InvalidParameter(parameter.1.range().into())
                            })?
                            .to_rgba8();

                        tint = Some(Color32::from_rgba_unmultiplied(t[0], t[1], t[2], t[3]));
                    }
                    _ => {}
                }
            }
            if uri.is_empty() {
                return Err(super::Error::KnownMissing(obj_range.into(), "value"));
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
                    uri = new_uri.to_string();
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
                            tree_cursor.node().range().into(),
                            e,
                        ))
                    }
                }
            };
            ObjectType::Image {
                uri,
                bytes,
                scale,
                tint: tint.unwrap_or(Color32::WHITE),
            }
        }
        "Paragraph" | "Header" => {
            let mut text = None;
            let mut align = Align::LEFT;
            let mut font = FontFamily::Proportional;
            let mut font_size = match obj_type.as_ref() {
                "Paragraph" => 48.0,
                "Header" => 64.0,
                _ => unreachable!(),
            };
            let mut language = None;
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
                    "value" => text = Some(parameter.1),
                    "align" => match value.as_ref() {
                        "left" | "Left" => align = Align::LEFT,
                        "center" | "Center" => align = Align::Center,
                        "right" | "Right" => align = Align::RIGHT,
                        _ => {
                            errors_present
                                .push(super::Error::InvalidParameter(parameter.1.range().into()));
                            continue;
                        }
                    },
                    "font_family" => {
                        font = match value.as_ref() {
                            "proportional" => FontFamily::Proportional,
                            "monospace" => FontFamily::Monospace,
                            _ => {
                                errors_present.push(super::Error::InvalidParameter(
                                    parameter.1.range().into(),
                                ));
                                continue;
                            }
                        };
                    }
                    "font_size" => font_size = value.parse::<f32>().unwrap(),
                    "language" => {
                        language = Some((
                            value,
                            parameter.1.child(1 /* second child */).unwrap().range(),
                        ))
                    }
                    _ => {}
                }
            }
            let text = if let Some(t) = text {
                t
            } else {
                return Err(super::Error::KnownMissing(obj_range.into(), "value"));
            };
            let layout_job = match language {
                Some(lang) => highlighting::highlight_text(
                    text,
                    lang,
                    align,
                    FontId::new(font_size, font.clone()),
                    helix_cell,
                    source,
                    hasher,
                )?,
                _ => {
                    let mut layout_job = LayoutJob {
                        halign: align,
                        break_on_newline: true,
                        wrap: TextWrapping {
                            max_width: 0.0,
                            max_rows: u32::MAX as usize,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    let mut walker = GrzCursor::from_node(text);
                    walker.goto_first_child()?;
                    let mut options = pulldown_cmark::Options::empty();
                    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);

                    let mut string_content = String::new();

                    loop {
                        match NodeKind::from(walker.node().kind_id()) {
                            NodeKind::StringContent => {
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

                    let parser = pulldown_cmark::Parser::new_ext(&string_content, options);

                    let mut tags = Vec::new();

                    macro_rules! layout_append {
                        ($layout_job:expr,$text:expr,$font_id:expr) => {
                            $layout_job.append(
                                $text,
                                0.0,
                                TextFormat {
                                    font_id: $font_id,
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
                        };
                    }

                    for event in parser {
                        match event {
                            pulldown_cmark::Event::Start(tag) => {
                                tags.push(tag);
                            }
                            pulldown_cmark::Event::End(_) => {
                                tags.pop();
                            }
                            pulldown_cmark::Event::Code(text) => {
                                layout_append!(
                                    layout_job,
                                    text.as_ref(),
                                    FontId::monospace(font_size)
                                );
                            }
                            pulldown_cmark::Event::SoftBreak => {
                                layout_append!(
                                    layout_job,
                                    "\n",
                                    FontId::new(font_size, font.clone())
                                );
                            }
                            pulldown_cmark::Event::HardBreak => {
                                layout_append!(
                                    layout_job,
                                    "\n\n",
                                    FontId::new(font_size, font.clone())
                                );
                            }
                            pulldown_cmark::Event::Text(text) => {
                                layout_append!(
                                    layout_job,
                                    text.as_ref(),
                                    FontId::new(font_size, font.clone())
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
        _ => return Err(super::Error::NotFound(obj_range.into())),
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    Ok((
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
    ))
}
