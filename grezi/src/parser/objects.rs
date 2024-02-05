use std::{
    fmt::Debug,
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use eframe::{
    egui::{Image, ImageFit, Ui},
    emath::Align,
    epaint::{mutex::RwLock, Color32, FontFamily, FontId, Pos2, Rect, Vec2},
};
use egui_glyphon::glyphon::{Attrs, Buffer, Edit, Family, FontSystem, Shaping, Style, Weight};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

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
    pub object: ObjectType,
}

pub type Job = Vec<(String, Color32, String)>;

pub fn add_job_to_buffer(job: &Job, buffer: &mut Buffer, font_system: &mut FontSystem) {
    buffer.set_rich_text(
        font_system,
        job.iter().map(|job| {
            let c = job.1.to_srgba_unmultiplied();
            (
                job.0.as_str(),
                fontstr_to_query(job.2.as_str())
                    .color(egui_glyphon::glyphon::Color::rgba(c[0], c[1], c[2], c[3])),
            )
        }),
        Shaping::Advanced,
    );
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Text(Job, f32, Option<f32>),
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

pub struct Editor(pub RwLock<egui_glyphon::glyphon::Editor>);

impl AsRef<Buffer> for Editor {
    fn as_ref(&self) -> &Buffer {
        unsafe { std::mem::transmute(self.0.read().buffer()) }
    }
}

impl Debug for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Editor").finish_non_exhaustive()
    }
}

impl Editor {
    pub fn highlight_boxes(&self) -> Vec<Rect> {
        use std::cmp;

        let editor = self.0.read();
        let mut rects = Vec::new();

        let line_height = editor.buffer().metrics().line_height;

        for run in editor.buffer().layout_runs() {
            let line_i = run.line_i;
            let line_top = run.line_top;

            if let Some(select) = editor.select_opt() {
                let (start, end) = match select.line.cmp(&editor.cursor().line) {
                    cmp::Ordering::Greater => (editor.cursor(), select),
                    cmp::Ordering::Less => (select, editor.cursor()),
                    cmp::Ordering::Equal => {
                        /* select.line == editor.cursor.line */
                        if select.index < editor.cursor().index {
                            (select, editor.cursor())
                        } else {
                            /* select.index >= editor.cursor.index */
                            (editor.cursor(), select)
                        }
                    }
                };

                if line_i >= start.line && line_i <= end.line {
                    let mut range_opt: Option<(f32, f32)> = None;
                    for glyph in run.glyphs.iter() {
                        // Guess x offset based on characters
                        let cluster = &run.text[glyph.start..glyph.end];
                        let total = cluster.grapheme_indices(true).count();
                        let mut c_x = glyph.x;
                        let c_w = glyph.w / total as f32;
                        for (i, c) in cluster.grapheme_indices(true) {
                            let c_start = glyph.start + i;
                            let c_end = glyph.start + i + c.len();
                            if (start.line != line_i || c_end > start.index)
                                && (end.line != line_i || c_start < end.index)
                            {
                                range_opt = match range_opt.take() {
                                    Some((min, max)) => Some((min.min(c_x), max.max(c_x + c_w))),
                                    None => Some((c_x, (c_x + c_w))),
                                };
                            } else if let Some((min, max)) = range_opt.take() {
                                rects.push(Rect::from_min_size(
                                    Pos2::new(min, line_top),
                                    Vec2::new(0.0f32.max(max - min), line_height),
                                ));
                            }
                            c_x += c_w;
                        }
                    }

                    if run.glyphs.is_empty() && end.line > line_i {
                        // Highlight all of internal empty lines
                        range_opt = Some((0.0, editor.buffer().size().0));
                    }

                    if let Some((mut min, mut max)) = range_opt.take() {
                        if end.line > line_i {
                            // Draw to end of line
                            if run.rtl {
                                min = 0.0;
                            } else {
                                max = editor.buffer().size().0;
                            }
                        }
                        rects.push(Rect::from_min_size(
                            Pos2::new(min, line_top),
                            Vec2::new(0.0f32.max(max - min), line_height),
                        ));
                    }
                }
            }
        }
        rects
    }
}

#[derive(Debug, Clone)]
pub enum ResolvedObject {
    Text(Arc<Editor>),
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

    Rect::from_min_size(
        Pos2::ZERO,
        Vec2::new(
            if rtl { vb.x } else { width.min(max_width) },
            (total_lines as f32 * buffer.metrics().line_height).min(max_height),
        ),
    )
}

impl ResolvedObject {
    pub fn bounds(&self, vb: Vec2, ui: &mut Ui) -> Rect {
        match self {
            ResolvedObject::Text(buffer) => measure_buffer(buffer.0.read().buffer(), vb),
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
            }
        }
        "Paragraph" | "Header" => {
            let mut text = None;
            let mut color = None;
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
                    _ => {}
                }
            }
            let text = if let Some(t) = text {
                t
            } else {
                return Err(super::Error::KnownMissing(obj_range.into(), "value".into()));
            };
            let job = match language {
                Some(lang) => highlighting::highlight_text(
                    text,
                    lang,
                    FontId::new(font_size, font.0.clone()),
                    helix_cell,
                    source,
                    hasher,
                )?,
                _ => {
                    let mut job = Job::new();
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
                    font_ids.push(match font.0.clone() {
                        FontFamily::Name(n) => format!("{}", n),
                        FontFamily::Proportional => "sans-serif".to_owned(),
                        FontFamily::Monospace => "monospace".to_owned(),
                    });

                    macro_rules! layout_append {
                        ($job:expr,$text:expr,$font_id:expr) => {
                            if !$text.is_empty() {
                                if tags.contains(&Tag::Item) && !li_printed_num {
                                    li_printed_num = true;
                                    match item_number {
                                        Some(n) => $job.push((
                                            format!("{}. ", n),
                                            color.unwrap_or(Color32::WHITE),
                                            $font_id,
                                        )),
                                        None => $job.push((
                                            "• ".to_owned(),
                                            color.unwrap_or(Color32::WHITE),
                                            $font_id,
                                        )),
                                    }
                                }
                                $job.push(($text, color.unwrap_or(Color32::WHITE), $font_id));
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
                                        let family = format!("{}:italic", font_ids.last().unwrap());
                                        font_ids.push(family);
                                    }
                                    Tag::Strong => {
                                        let family = if tags.contains(&Tag::Emphasis) {
                                            format!("{}:bold:italic", font_ids.last().unwrap())
                                        } else {
                                            format!("{}:bold", font_ids.last().unwrap())
                                        };
                                        font_ids.push(family);
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
                                layout_append!(job, text.to_string(), "monospace".to_owned());
                            }
                            pulldown_cmark::Event::SoftBreak => {
                                layout_append!(
                                    job,
                                    "\n".to_string(),
                                    font_ids.last().unwrap().clone()
                                );
                            }
                            pulldown_cmark::Event::HardBreak => {
                                layout_append!(
                                    job,
                                    "\n\n".to_owned(),
                                    font_ids.last().unwrap().clone()
                                );
                            }
                            pulldown_cmark::Event::Text(text) => {
                                layout_append!(
                                    job,
                                    text.to_string(),
                                    font_ids.last().unwrap().clone()
                                );
                            }
                            _ => {}
                        }
                    }
                    job
                }
            };
            ObjectType::Text(job, font_size, None)
        }
        _ => return Err(super::Error::NotFound(obj_range.into())),
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
