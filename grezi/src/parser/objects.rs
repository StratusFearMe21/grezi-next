use std::{
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use eframe::{
    egui::{Image, ImageFit, TextFormat, Ui},
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
    NodeKind,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Object {
    pub position: Option<LineUp>,
    pub viewbox: Option<ViewboxIn>,
    pub source_obj: Option<u64>,
    pub object: ObjectType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Text {
        layout_job: LayoutJob,
        source: bool,
    },
    Image {
        uri: String,
        bytes: Arc<[u8]>,
        scale: Option<Vec2>,
        source: Option<LayoutJob>,
        tint: Color32,
    },
    Rect {
        color: Color32,
        height: f32,
    },
    Spinner,
}

impl ObjectType {
    fn apply_source(&mut self, index: usize) {
        match self {
            ObjectType::Text { layout_job, .. } => {
                layout_job.append(
                    &format!("{}.", index + 1),
                    0.0,
                    TextFormat {
                        font_id: FontId::proportional(24.0),
                        color: Color32::WHITE,
                        background: Color32::TRANSPARENT,
                        italics: false,
                        underline: Stroke::NONE,
                        strikethrough: Stroke::NONE,
                        valign: Align::TOP,
                        ..Default::default()
                    },
                );
            }
            ObjectType::Image { source, .. } => {
                let mut layout_job = LayoutJob {
                    wrap: TextWrapping {
                        max_rows: u32::MAX as usize,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                layout_job.append(
                    &format!("{}.", index + 1),
                    0.0,
                    TextFormat {
                        font_id: FontId::proportional(24.0),
                        color: Color32::WHITE,
                        background: Color32::TRANSPARENT,
                        italics: false,
                        underline: Stroke::NONE,
                        strikethrough: Stroke::NONE,
                        valign: Align::TOP,
                        ..Default::default()
                    },
                );
                *source = Some(layout_job);
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResolvedObject {
    Text(Arc<Galley>),
    Image {
        image: Image<'static>,
        scale: Option<Vec2>,
        source: Option<Arc<Galley>>,
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

impl ResolvedObject {
    pub fn bounds(&self, vb: Vec2, ui: &mut Ui) -> Rect {
        match self {
            ResolvedObject::Text(galley) => galley.rect.expand(1.0),
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
    fonts: &mut eframe::egui::FontDefinitions,
    font_db: &mut fontdb::Database,
    ctx: &eframe::egui::Context,
    errors_present: &mut Vec<super::Error>,
    file_path: &std::path::Path,
    sources: &mut indexmap::IndexSet<String, ahash::RandomState>,
    mut insert_fn: impl FnMut(u64, Object),
) -> Result<(), super::Error> {
    use std::borrow::Cow;

    use super::color::DefaultColorParser;
    use cssparser::ParserInput;
    use eframe::egui::load::Bytes;
    use helix_core::tree_sitter::{Node, Point, Range};
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
    let mut source_cited = None;
    let mut object = match obj_type.as_ref() {
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
                            &mut DefaultColorParser::new(None),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(parameter.1.range().into(), format!("{:?}", e))
                        })?;

                        tint = Some(t.1.into());
                    }
                    "height" => height = Some(value.parse::<f32>().unwrap()),
                    _ => {}
                }
            }
            ObjectType::Rect {
                color: tint.unwrap_or(Color32::WHITE),
                height: height.ok_or_else(|| {
                    super::Error::KnownMissing(obj_range.into(), "Rectangle height".into())
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
                        let t = super::color::parse_color_with(
                            &mut DefaultColorParser::new(None),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(parameter.1.range().into(), format!("{:?}", e))
                        })?;

                        tint = Some(t.1.into());
                    }
                    "source" => source_cited = Some(value),
                    _ => {}
                }
            }
            if uri.is_empty() {
                return Err(super::Error::KnownMissing(obj_range.into(), "value".into()));
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
                            tree_cursor.node().range().into(),
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
                source: None,
            }
        }
        "Paragraph" | "Header" => {
            let mut text = None;
            let mut color = None;
            let mut bg = None;
            let mut align = Align::LEFT;
            let mut font = (
                FontFamily::Proportional,
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
                    "value" | "code" => text = Some(parameter.1),
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
                    "color" => {
                        let c = super::color::parse_color_with(
                            &mut DefaultColorParser::new(None),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(parameter.1.range().into(), format!("{:?}", e))
                        })?;

                        color = Some(c.1.into());
                    }
                    "background" => {
                        let c = super::color::parse_color_with(
                            &mut DefaultColorParser::new(None),
                            &mut cssparser::Parser::new(&mut ParserInput::new(&value)),
                        )
                        .map_err(|e| {
                            super::Error::ColorError(parameter.1.range().into(), format!("{:?}", e))
                        })?;

                        bg = Some(c.1.into());
                    }
                    "font_family" => {
                        font = (
                            match value.as_ref() {
                                "proportional" => FontFamily::Proportional,
                                "monospace" => FontFamily::Monospace,
                                font => FontFamily::Name(font.into()),
                            },
                            parameter.1.range(),
                        );
                    }
                    "font_size" => font_size = value.parse::<f32>().unwrap(),
                    "language" => {
                        language = Some((
                            value,
                            parameter.1.child(1 /* second child */).unwrap().range(),
                        ))
                    }
                    "source" => source_cited = Some(value),
                    _ => {}
                }
            }
            add_font(fonts, font_db, ctx, &font.0)
                .map_err(|()| super::Error::KnownMissing(font.1.into(), "Font not found".into()))?;
            let text = if let Some(t) = text {
                t
            } else {
                return Err(super::Error::KnownMissing(obj_range.into(), "value".into()));
            };
            let layout_job = match language {
                Some(lang) => highlighting::highlight_text(
                    text,
                    lang,
                    align,
                    FontId::new(font_size, font.0.clone()),
                    helix_cell,
                    source,
                    hasher,
                )?,
                _ => {
                    let mut layout_job = LayoutJob {
                        halign: align,
                        break_on_newline: true,
                        wrap: TextWrapping {
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

                    let parser = pulldown_cmark::Parser::new_ext(&string_content, options);

                    let mut tags = Vec::new();
                    let mut item_number = None;
                    let mut li_printed_num = false;

                    let mut font_ids = Vec::with_capacity(2);
                    font_ids.push(FontId::new(font_size, font.0.clone()));

                    macro_rules! layout_append {
                        ($layout_job:expr,$text:expr,$font_id:expr) => {
                            if !$text.is_empty() {
                                if tags.contains(&Tag::Item) && !li_printed_num {
                                    li_printed_num = true;
                                    match item_number {
                                        Some(n) => $layout_job.append(
                                            &format!("{}. ", n),
                                            1.5,
                                            TextFormat {
                                                font_id: $font_id,
                                                color: color.unwrap_or(Color32::WHITE),
                                                background: bg.unwrap_or(Color32::TRANSPARENT),
                                                italics: false,
                                                underline: Stroke::NONE,
                                                strikethrough: if tags.contains(&Tag::Strikethrough)
                                                {
                                                    Stroke::new(5.0, Color32::WHITE)
                                                } else {
                                                    Stroke::NONE
                                                },
                                                ..Default::default()
                                            },
                                        ),
                                        None => $layout_job.append(
                                            "• ",
                                            1.5,
                                            TextFormat {
                                                font_id: $font_id,
                                                color: color.unwrap_or(Color32::WHITE),
                                                background: bg.unwrap_or(Color32::TRANSPARENT),
                                                italics: false,
                                                underline: Stroke::NONE,
                                                strikethrough: if tags.contains(&Tag::Strikethrough)
                                                {
                                                    Stroke::new(5.0, Color32::WHITE)
                                                } else {
                                                    Stroke::NONE
                                                },
                                                ..Default::default()
                                            },
                                        ),
                                    }
                                }
                                $layout_job.append(
                                    $text,
                                    0.0,
                                    TextFormat {
                                        font_id: $font_id,
                                        color: color.unwrap_or(Color32::WHITE),
                                        background: bg.unwrap_or(Color32::TRANSPARENT),
                                        italics: false,
                                        underline: Stroke::NONE,
                                        strikethrough: if tags.contains(&Tag::Strikethrough) {
                                            Stroke::new(5.0, Color32::WHITE)
                                        } else {
                                            Stroke::NONE
                                        },
                                        ..Default::default()
                                    },
                                );
                            }
                        };
                    }

                    for event in parser {
                        match event {
                            pulldown_cmark::Event::Start(tag) => {
                                match tag {
                                    Tag::List(l) => item_number = l.map(|l| l - 1),
                                    Tag::Item => {
                                        item_number.as_mut().map(|i| *i += 1);
                                    }
                                    Tag::Emphasis => {
                                        let family = match font.0.clone() {
                                            FontFamily::Name(n) => {
                                                FontFamily::Name(format!("{}:italic", n).into())
                                            }
                                            FontFamily::Proportional => {
                                                FontFamily::Name("Ubuntu:light:italic".into())
                                            }
                                            FontFamily::Monospace => FontFamily::Monospace,
                                        };
                                        add_font(fonts, font_db, ctx, &family).unwrap();
                                        font_ids.push(FontId::new(font_size, family));
                                    }
                                    Tag::Strong => {
                                        let family = if tags.contains(&Tag::Emphasis) {
                                            match font.0.clone() {
                                                FontFamily::Name(n) => FontFamily::Name(
                                                    format!("{}:bold:italic", n).into(),
                                                ),
                                                FontFamily::Proportional => {
                                                    FontFamily::Name("Ubuntu:bold:italic".into())
                                                }
                                                FontFamily::Monospace => FontFamily::Monospace,
                                            }
                                        } else {
                                            match font.0.clone() {
                                                FontFamily::Name(n) => {
                                                    FontFamily::Name(format!("{}:bold", n).into())
                                                }
                                                FontFamily::Proportional => {
                                                    FontFamily::Name("Ubuntu:bold".into())
                                                }
                                                FontFamily::Monospace => FontFamily::Monospace,
                                            }
                                        };
                                        add_font(fonts, font_db, ctx, &family).unwrap();
                                        font_ids.push(FontId::new(font_size, family));
                                    }
                                    _ => {}
                                }
                                tags.push(tag);
                            }
                            pulldown_cmark::Event::End(tag) => {
                                match tag {
                                    Tag::Emphasis | Tag::Strong => {
                                        font_ids.pop();
                                    }
                                    Tag::Item => li_printed_num = false,
                                    _ => {}
                                }
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
                                layout_append!(layout_job, "\n", font_ids.last().unwrap().clone());
                            }
                            pulldown_cmark::Event::HardBreak => {
                                layout_append!(
                                    layout_job,
                                    "\n\n",
                                    font_ids.last().unwrap().clone()
                                );
                            }
                            pulldown_cmark::Event::Text(text) => {
                                layout_append!(
                                    layout_job,
                                    text.as_ref(),
                                    font_ids.last().unwrap().clone()
                                );
                            }
                            _ => {}
                        }
                    }
                    layout_job
                }
            };
            ObjectType::Text {
                layout_job,
                source: false,
            }
        }
        _ => return Err(super::Error::NotFound(obj_range.into())),
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    let mut source_obj = None;
    if let Some(source) = source_cited {
        let mut hasher = hasher.build_hasher();
        std::hash::Hash::hash("__source__", &mut hasher);
        std::hash::Hash::hash(&source, &mut hasher);
        let s_obj = hasher.finish();
        let mut layout_job = LayoutJob {
            halign: Align::RIGHT,
            break_on_newline: true,
            wrap: TextWrapping {
                max_rows: u32::MAX as usize,
                ..Default::default()
            },
            ..Default::default()
        };
        let source_index = sources.get_index_of(source.as_ref()).unwrap_or_else(|| {
            let index = sources.len();
            sources.insert(source.clone().into_owned());
            index
        });
        let family = FontFamily::Name("Ubuntu:light:italic".into());
        add_font(fonts, font_db, ctx, &family).unwrap();
        layout_job.append(
            &format!("{}. ", source_index + 1),
            1.5,
            TextFormat {
                font_id: FontId::proportional(18.0),
                color: Color32::WHITE,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                valign: Align::TOP,
                ..Default::default()
            },
        );
        layout_job.append(
            source.as_ref(),
            0.0,
            TextFormat {
                font_id: FontId::new(24.0, family),
                color: Color32::WHITE,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                ..Default::default()
            },
        );
        object.apply_source(source_index);
        insert_fn(
            s_obj,
            Object {
                position: None,
                viewbox: None,
                object: ObjectType::Text {
                    layout_job,
                    source: true,
                },
                source_obj: None,
            },
        );
        source_obj = Some(s_obj);
    }

    insert_fn(
        {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(&name, &mut hasher);
            hasher.finish()
        },
        Object {
            position: None,
            viewbox: None,
            source_obj,
            object,
        },
    );

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn add_font(
    fonts: &mut eframe::egui::FontDefinitions,
    font_db: &mut fontdb::Database,
    ctx: &eframe::egui::Context,
    font: &FontFamily,
) -> Result<(), ()> {
    use eframe::egui::FontData;
    use fontdb::{Family, Query, Style, Weight};

    if !fonts.families.contains_key(font) {
        match font {
            FontFamily::Name(n) => {
                let mut split = n.split(':');
                let base = split.next().unwrap();

                let mut font_prop = Query::default();
                let families = [Family::Name(base)];
                font_prop.families = &families;
                for s in split {
                    match s {
                        "normal" => font_prop.style = Style::Normal,
                        "italic" => font_prop.style = Style::Italic,
                        "oblique" => font_prop.style = Style::Oblique,
                        // Thin weight (100), the thinnest value.
                        "thin" => font_prop.weight = Weight::THIN,
                        // Extra light weight (200).
                        "extra_light" => font_prop.weight = Weight::EXTRA_LIGHT,
                        // Light weight (300).
                        "light" => font_prop.weight = Weight::LIGHT,
                        // Normal (400).
                        "normal" => font_prop.weight = Weight::NORMAL,
                        // Medium weight (500, higher than normal).
                        "medium" => font_prop.weight = Weight::MEDIUM,
                        // Semibold weight (600).
                        "semibold" => font_prop.weight = Weight::SEMIBOLD,
                        // Bold weight (700).
                        "bold" => font_prop.weight = Weight::BOLD,
                        // Extra-bold weight (800).
                        "extra_bold" => font_prop.weight = Weight::EXTRA_BOLD,
                        // Black weight (900), the thickest value.
                        "black" => font_prop.weight = Weight::BLACK,
                        _ => {}
                    }
                }
                if let Some(fetched_font) = font_db.query(&font_prop) {
                    // Leaking the font makes it cheaper to clone the font definitions elsewhere
                    let (src, index) =
                        unsafe { font_db.make_shared_face_data(fetched_font).unwrap() };
                    let data: &'static [u8] = unsafe { &*Arc::into_raw(src) }.as_ref();
                    fonts.font_data.insert(n.to_string(), {
                        let mut font = FontData::from_static(data);
                        font.index = index;
                        font
                    });

                    fonts.families.insert(font.clone(), vec![n.to_string()]);
                    ctx.set_fonts(fonts.clone());
                } else {
                    return Err(());
                }
            }
            _ => {}
        }
    }
    Ok(())
}
