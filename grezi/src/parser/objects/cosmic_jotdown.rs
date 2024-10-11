use std::{borrow::Cow, cmp, sync::Arc};

use eframe::egui::{mutex::RwLock, Pos2, Rect, Vec2};
use egui_glyphon::glyphon::{
    cosmic_text::{self, Align},
    Affinity, AttrsOwned, Cursor,
};

use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Style, Weight};
use helix_core::unicode::segmentation::UnicodeSegmentation;
use jotdown::{Container, Event, ListKind, OrderedListNumbering, OrderedListStyle};
use nominals::{LetterLower, LetterUpper, Nominal, RomanLower, RomanUpper};

pub use super::serde_suck::*;
use super::{measure_buffer, VerticalSpacing};
use rangemap::RangeMap;
use serde::{Deserialize, Serialize};

pub struct JotdownBufferIter<'a, T: Iterator<Item = Event<'a>>> {
    pub djot: T,
    pub attrs: Attrs<'a>,
    pub indent: Vec<Indent>,
}

struct JotdownIntoBuffer<'a, 'b, T: Iterator<Item = Event<'a>>> {
    pub djot: &'b mut T,
    pub attrs: Attrs<'a>,
    pub metrics: Metrics,
    pub indent: &'b mut Vec<Indent>,
    pub link_start: Cursor,
    pub urls: Vec<(std::ops::Range<Cursor>, Cow<'a, str>)>,
    pub location: Cursor,
    pub added: bool,
    pub top_level_container: Option<Container<'a>>,
}

#[derive(Default, Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Indent {
    #[serde(with = "ListKindOption")]
    pub modifier: Option<ListKind>,
    pub indent: f32,
}

pub const INDENT_AMOUNT: f32 = 4.0 / 3.0;

impl<'a, 'b, T: Iterator<Item = Event<'a>>> Iterator for JotdownIntoBuffer<'a, 'b, T> {
    type Item = (&'a str, Attrs<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.djot.next() {
            match event {
                Event::LeftSingleQuote => {
                    self.added = true;
                    self.location.index += "‘".len();
                    return Some(("‘", self.attrs.clone()));
                }
                Event::LeftDoubleQuote => {
                    self.added = true;
                    self.location.index += "“".len();
                    return Some(("“", self.attrs.clone()));
                }
                Event::RightSingleQuote => {
                    self.added = true;
                    self.location.index += "’".len();
                    return Some(("’", self.attrs.clone()));
                }
                Event::RightDoubleQuote => {
                    self.added = true;
                    self.location.index += "”".len();
                    return Some(("”", self.attrs.clone()));
                }
                Event::Ellipsis => {
                    self.added = true;
                    self.location.index += "…".len();
                    return Some(("…", self.attrs.clone()));
                }
                Event::EmDash => {
                    self.added = true;
                    self.location.index += "—".len();
                    return Some(("—", self.attrs.clone()));
                }
                Event::EnDash => {
                    self.added = true;
                    self.location.index += "–".len();
                    return Some(("–", self.attrs.clone()));
                }
                Event::Softbreak | Event::NonBreakingSpace => {
                    self.added = true;
                    self.location.index += " ".len();
                    return Some((" ", self.attrs.clone()));
                }
                Event::Hardbreak | Event::ThematicBreak(_) => {
                    self.added = true;
                    self.location.index = 0;
                    self.location.line += 1;
                    return Some(("\n", self.attrs.clone()));
                }
                Event::Str(Cow::Borrowed(s)) | Event::Symbol(Cow::Borrowed(s)) => {
                    self.added = true;
                    self.location.index += s.len();
                    return Some((s, self.attrs.clone()));
                }
                Event::Start(container, _) => match container {
                    Container::Heading { level, .. } => {
                        let l = match level {
                            1 => 2.0,
                            2 => 1.5,
                            3 => 1.17,
                            4 => 1.0,
                            5 => 0.83,
                            _ => 0.67,
                        };
                        self.metrics = Metrics::new(l, l * 1.2);
                        self.top_level_container.get_or_insert(container);
                    }
                    Container::Emphasis => self.attrs = self.attrs.style(Style::Italic),
                    Container::Strong => self.attrs = self.attrs.weight(Weight::BOLD),
                    Container::Verbatim => self.attrs = self.attrs.family(Family::Monospace),
                    Container::ListItem { .. } | Container::Paragraph => {
                        self.top_level_container.get_or_insert(container);
                    }
                    Container::List { kind, .. } => self.indent.push(Indent {
                        indent: self.indent.last().copied().unwrap_or_default().indent
                            + (INDENT_AMOUNT * self.metrics.font_size),
                        modifier: Some(kind),
                    }),
                    Container::Link(_, _) => {
                        self.link_start = self.location;
                        self.attrs = self.attrs.color(Color::rgb(96, 198, 233));
                    }
                    _ => {}
                },
                Event::End(container) => match container {
                    Container::Emphasis => self.attrs = self.attrs.style(Style::Normal),
                    Container::Strong => self.attrs = self.attrs.weight(Weight::NORMAL),
                    Container::Verbatim => self.attrs = self.attrs.family(Family::SansSerif),
                    Container::List { .. } => {
                        self.indent.pop();
                    }
                    Container::Heading { .. } | Container::Paragraph => {
                        if self.added {
                            return None;
                        }
                    }
                    Container::Link(Cow::Borrowed(url), _) => {
                        let mut location = self.location;
                        location.index += 1;
                        self.urls
                            .push((self.link_start..location, Cow::Borrowed(url)));
                        self.attrs = self.attrs.color(Color::rgb(255, 255, 255));
                    }
                    _ => {}
                },
                Event::Blankline | Event::Escape | Event::FootnoteReference(_) => {}
                Event::Str(Cow::Owned(_)) | Event::Symbol(Cow::Owned(_)) => panic!(),
            }
        }

        return None;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RichText(pub String, #[serde(with = "AttrsSerde")] pub AttrsOwned);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JotdownItem {
    pub indent: Indent,
    pub buffer: Vec<RichText>,
    #[serde(with = "MetricsSerde")]
    pub metrics: Metrics,
    pub margin: f32,
    pub url_map: Option<RangeMap<CursorSerde, String>>,
}

#[derive(Clone)]
pub struct ResolvedJotdownItem {
    pub indent: Indent,
    pub buffer: Arc<RwLock<Buffer>>,
    pub metrics: Metrics,
    pub relative_bounds: Rect,
    pub url_map: Option<Arc<RangeMap<CursorSerde, String>>>,
}

pub fn resolve_paragraphs(
    paragaphs: &[JotdownItem],
    viewbox: Vec2,
    font_system: &mut FontSystem,
    metrics: Metrics,
    align: Option<Align>,
    factor: f32,
    spacing: VerticalSpacing,
) -> (Vec2, Vec<ResolvedJotdownItem>) {
    let mut size = Vec2::new(0.0, 0.0);
    let mut last_margin = 99999.0;
    let mut first = true;
    let mut in_list = false;
    let mut list_number = 0;
    let mut paragraphs = paragaphs
        .iter()
        .flat_map(|jbuffer| {
            let mut buffer =
                jbuffer
                    .clone()
                    .resolve(font_system, viewbox.x, metrics, align, factor);
            let margin = jbuffer.margin * buffer.metrics.line_height;

            let margin_top = (margin - last_margin).max(0.0);
            buffer.relative_bounds = buffer
                .relative_bounds
                .translate(Vec2::new(buffer.indent.indent, size.y + margin_top));

            let buffer_size = buffer.relative_bounds.size();
            let buffer_indent = buffer.indent.indent;
            let result_buffers = if let Some(mut list_kind) = buffer.indent.modifier {
                if let ListKind::Ordered { start, .. } = &mut list_kind {
                    if !in_list {
                        list_number = *start;
                    } else {
                        list_number += 1;
                    }

                    *start = list_number;
                }

                in_list = true;

                let mut list_buffer = Buffer::new(font_system, buffer.metrics);
                list_buffer.set_text(
                    font_system,
                    make_list_number(list_kind).as_ref(),
                    Attrs::new().family(Family::SansSerif),
                    Shaping::Advanced,
                );

                list_buffer.set_wrap(font_system, cosmic_text::Wrap::WordOrGlyph);
                list_buffer.set_size(font_system, Some(f32::MAX), Some(f32::MAX));

                list_buffer.shape_until_scroll(font_system, false);

                let list_buffer_metrics = buffer.metrics;
                let indent = (buffer_indent) - (INDENT_AMOUNT * list_buffer_metrics.font_size);
                [
                    Some(buffer),
                    Some(ResolvedJotdownItem {
                        indent: Indent {
                            modifier: None,
                            indent,
                        },
                        relative_bounds: measure_buffer(
                            &list_buffer,
                            Vec2::new(f32::MAX, f32::MAX),
                        )
                        .translate(Vec2::new(indent, size.y + margin_top)),
                        buffer: Arc::new(RwLock::new(list_buffer)),
                        metrics: list_buffer_metrics,
                        url_map: None,
                    }),
                ]
            } else {
                in_list = false;
                [Some(buffer), None]
            };

            size.y += buffer_size.y + (margin_top + margin);
            last_margin = margin;
            first = false;
            size.x = size.x.max(buffer_size.x + buffer_indent);

            result_buffers
        })
        .filter_map(|p| p)
        .collect::<Vec<_>>();

    size.y -= last_margin;

    paragraphs.iter_mut().for_each(|paragraph| {
        paragraph
            .relative_bounds
            .set_width(size.x - paragraph.indent.indent);
        let mut buffer = paragraph.buffer.write();
        buffer.set_size(
            font_system,
            Some(size.x - paragraph.indent.indent),
            Some(f32::MAX),
        );
        buffer.shape_until_scroll(font_system, false);
    });

    if matches!(spacing, VerticalSpacing::Even) {
        let mut height: f32 = paragraphs.iter().map(|p| p.relative_bounds.height()).sum();

        height = (viewbox.y - height) / (paragraphs.len() - 1) as f32;

        let mut height_at = 0.0;

        paragraphs.iter_mut().for_each(|p| {
            p.relative_bounds = Rect::from_min_size(
                Pos2::new(p.relative_bounds.min.x, height_at),
                p.relative_bounds.size(),
            );

            height_at += height + p.relative_bounds.height();
        });
    }

    (size, paragraphs)
}

impl JotdownItem {
    pub fn resolve(
        mut self,
        font_system: &mut FontSystem,
        width: f32,
        metrics: Metrics,
        align: Option<Align>,
        factor: f32,
    ) -> ResolvedJotdownItem {
        self.indent.indent *= factor;
        self.indent.indent *= metrics.font_size;

        let buffer = self.make_buffer(font_system, width, metrics, align);

        ResolvedJotdownItem {
            indent: self.indent,
            relative_bounds: measure_buffer(&buffer, Vec2::new(width, f32::MAX)),
            buffer: Arc::new(RwLock::new(buffer)),
            metrics: Metrics::new(
                self.metrics.font_size * metrics.font_size,
                self.metrics.line_height * metrics.line_height,
            ),
            url_map: self.url_map.map(|m| Arc::new(m)),
        }
    }

    pub fn new_default(text: Vec<RichText>) -> Self {
        Self {
            indent: Indent {
                modifier: None,
                indent: 0.0,
            },
            buffer: text,
            metrics: Metrics::new(1.0, 1.2),
            url_map: None,
            margin: 0.0,
        }
    }
}

impl JotdownItem {
    pub fn make_buffer(
        &self,
        font_system: &mut FontSystem,
        width: f32,
        metrics: Metrics,
        align: Option<Align>,
    ) -> Buffer {
        let mut buffer = Buffer::new(
            font_system,
            Metrics::new(
                metrics.font_size * self.metrics.font_size,
                metrics.line_height * self.metrics.line_height,
            ),
        );
        buffer.set_rich_text(
            font_system,
            self.buffer.iter().map(|r| (r.0.as_ref(), r.1.as_attrs())),
            // Default attrs are not used
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        buffer.set_wrap(font_system, cosmic_text::Wrap::WordOrGlyph);
        buffer.set_size(
            font_system,
            Some(width - self.indent.indent),
            Some(f32::MAX),
        );

        for line in &mut buffer.lines {
            line.set_align(align);
        }

        buffer.shape_until_scroll(font_system, false);
        buffer
    }
}

impl<'a, T: Iterator<Item = Event<'a>>> Iterator for JotdownBufferIter<'a, T> {
    type Item = JotdownItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut jot = JotdownIntoBuffer {
            djot: &mut self.djot,
            attrs: self.attrs.clone(),
            indent: &mut self.indent,
            metrics: Metrics::new(1.0, 1.2),
            added: false,
            link_start: Cursor::default(),
            location: Cursor {
                affinity: Affinity::After,
                ..Cursor::default()
            },
            urls: Vec::new(),
            top_level_container: None,
        };

        let buffer = (&mut jot)
            .map(|r| RichText(r.0.to_owned(), AttrsOwned::new(r.1)))
            .collect::<Vec<_>>();

        let urls = jot.urls;
        let top_level_containers = jot.top_level_container;
        let added = jot.added;
        let metrics = jot.metrics;
        let indent = self.indent.last().copied().unwrap_or_default();
        if !added {
            return None;
        } else {
            return Some(JotdownItem {
                indent,
                url_map: if urls.is_empty() {
                    None
                } else {
                    let mut map: RangeMap<CursorSerde, String> = RangeMap::new();
                    map.extend(urls.into_iter().map(|(range, url)| {
                        (range.start.into()..range.end.into(), url.into_owned())
                    }));
                    Some(map)
                },
                buffer,
                metrics,
                margin: match top_level_containers {
                    Some(Container::Heading { level, .. }) => match level {
                        1 => 0.34,
                        2 => 0.42,
                        3 => 0.5,
                        4 => 0.65,
                        5 => 0.85,
                        6 => 1.25,
                        _ => unreachable!(),
                    },
                    Some(Container::ListItem) => 0.5,
                    _ => 1.0,
                },
            });
        }
    }
}

pub fn jotdown_into_buffers<'a, T: Iterator<Item = Event<'a>>>(
    djot: T,
    attrs: &Attrs<'a>,
) -> JotdownBufferIter<'a, T> {
    JotdownBufferIter {
        djot,
        attrs: *attrs,
        indent: Vec::new(),
    }
}

pub fn make_list_number(list_kind: ListKind) -> Cow<'static, str> {
    match list_kind {
        ListKind::Unordered | ListKind::Task => Cow::Borrowed("•"),
        ListKind::Ordered {
            numbering,
            style,
            start: number,
        } => {
            use std::fmt::Write;
            let mut result = String::new();

            if matches!(style, OrderedListStyle::ParenParen) {
                result.push('(');
            }

            match numbering {
                OrderedListNumbering::Decimal => {
                    result.write_fmt(format_args!("{}", number)).unwrap()
                }
                OrderedListNumbering::AlphaLower => {
                    result
                        .write_fmt(format_args!("{}", (number - 1).to_nominal(&LetterLower)))
                        .unwrap();
                }
                OrderedListNumbering::AlphaUpper => {
                    result
                        .write_fmt(format_args!("{}", (number - 1).to_nominal(&LetterUpper)))
                        .unwrap();
                }
                OrderedListNumbering::RomanLower => {
                    result
                        .write_fmt(format_args!("{}", number.to_nominal(&RomanLower)))
                        .unwrap();
                }
                OrderedListNumbering::RomanUpper => {
                    result
                        .write_fmt(format_args!("{}", number.to_nominal(&RomanUpper)))
                        .unwrap();
                }
            }

            match style {
                OrderedListStyle::Period => result.push('.'),
                OrderedListStyle::Paren | OrderedListStyle::ParenParen => result.push(')'),
            }

            Cow::Owned(result)
        }
    }
}

pub fn link_area(buffer: &Buffer, start: Cursor, end: Cursor) -> Vec<Rect> {
    let mut rects = Vec::new();
    for run in buffer.layout_runs() {
        let line_i = run.line_i;
        let line_top = run.line_top;
        let line_height = run.line_height;

        // Highlight selection
        if line_i >= start.line && line_i <= end.line {
            let mut range_opt = None;
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
                            Some((min, max)) => {
                                Some((cmp::min(min, c_x as i32), cmp::max(max, (c_x + c_w) as i32)))
                            }
                            None => Some((c_x as i32, (c_x + c_w) as i32)),
                        };
                    } else if let Some((min, max)) = range_opt.take() {
                        rects.push(Rect::from_min_size(
                            Pos2::new(min as f32, line_top as f32),
                            Vec2::new(cmp::max(0, max - min) as f32, line_height as f32),
                        ));
                    }
                    c_x += c_w;
                }
            }

            if run.glyphs.is_empty() && end.line > line_i {
                // Highlight all of internal empty lines
                range_opt = Some((0, buffer.size().0.unwrap_or(0.0) as i32));
            }

            if let Some((mut min, mut max)) = range_opt.take() {
                if end.line > line_i {
                    // Draw to end of line
                    if run.rtl {
                        min = 0;
                    } else {
                        max = buffer.size().0.unwrap_or(0.0) as i32;
                    }
                }
                rects.push(Rect::from_min_size(
                    Pos2::new(min as f32, line_top as f32),
                    Vec2::new(cmp::max(0, max - min) as f32, line_height as f32),
                ));
            }
        }
    }

    rects
}
