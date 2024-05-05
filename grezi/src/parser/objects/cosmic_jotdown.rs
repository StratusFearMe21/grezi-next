use std::{borrow::Cow, sync::Arc};

use eframe::egui::{mutex::RwLock, Pos2, Rect, Vec2};
use egui_glyphon::glyphon::{
    cosmic_text::{self, Align},
    AttrsOwned,
};

use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Style, Weight};
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
    pub link_start: usize,
    pub urls: Vec<(std::ops::Range<usize>, Cow<'a, str>)>,
    pub location: usize,
    pub added: bool,
    pub top_level_container: Option<Container<'a>>,
}

#[derive(Default, Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Indent {
    #[serde(with = "ListKindOption")]
    pub modifier: Option<ListKind>,
    pub indent: f32,
}

pub const INDENT_AMOUNT: f32 = 64.0;

impl<'a, 'b, T: Iterator<Item = Event<'a>>> Iterator for JotdownIntoBuffer<'a, 'b, T> {
    type Item = (&'a str, Attrs<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.djot.next() {
            match event {
                Event::LeftSingleQuote => {
                    self.added = true;
                    self.location += "‘".len();
                    return Some(("‘", self.attrs.clone()));
                }
                Event::LeftDoubleQuote => {
                    self.added = true;
                    self.location += "“".len();
                    return Some(("“", self.attrs.clone()));
                }
                Event::RightSingleQuote => {
                    self.added = true;
                    self.location += "’".len();
                    return Some(("’", self.attrs.clone()));
                }
                Event::RightDoubleQuote => {
                    self.added = true;
                    self.location += "”".len();
                    return Some(("”", self.attrs.clone()));
                }
                Event::Ellipsis => {
                    self.added = true;
                    self.location += "…".len();
                    return Some(("…", self.attrs.clone()));
                }
                Event::EmDash => {
                    self.added = true;
                    self.location += "—".len();
                    return Some(("—", self.attrs.clone()));
                }
                Event::EnDash => {
                    self.added = true;
                    self.location += "–".len();
                    return Some(("–", self.attrs.clone()));
                }
                Event::Softbreak | Event::NonBreakingSpace => {
                    self.added = true;
                    self.location += " ".len();
                    return Some((" ", self.attrs.clone()));
                }
                Event::Str(Cow::Borrowed(s)) | Event::Symbol(Cow::Borrowed(s)) => {
                    self.added = true;
                    self.location += s.len();
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
                        self.metrics = Metrics::new(l, l * 1.1);
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
                            + INDENT_AMOUNT,
                        modifier: Some(kind),
                    }),
                    Container::Link(_, _) => {
                        self.link_start = self.location + 1;
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
                        self.urls
                            .push((self.link_start..self.location, Cow::Borrowed(url)));
                        self.attrs = self.attrs.color(Color::rgb(255, 255, 255));
                    }
                    _ => {}
                },
                Event::Hardbreak | Event::ThematicBreak(_) => {}
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
    pub url_map: Option<RangeMap<usize, String>>,
}

#[derive(Clone)]
pub struct ResolvedJotdownItem {
    pub indent: Indent,
    pub buffer: Arc<RwLock<Buffer>>,
    pub metrics: Metrics,
    pub relative_bounds: Rect,
    pub url_map: Option<RangeMap<usize, String>>,
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
                list_buffer.set_size(font_system, f32::MAX, f32::MAX);

                list_buffer.shape_until_scroll(font_system, false);

                let list_buffer_metrics = buffer.metrics;
                let indent = (buffer_indent) - (INDENT_AMOUNT * factor);
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
        buffer.set_size(font_system, size.x - paragraph.indent.indent, f32::MAX);
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

        let buffer = self.make_buffer(font_system, width, metrics, align);

        ResolvedJotdownItem {
            indent: self.indent,
            relative_bounds: measure_buffer(&buffer, Vec2::new(width, f32::MAX)),
            buffer: Arc::new(RwLock::new(buffer)),
            metrics: Metrics::new(
                self.metrics.font_size * metrics.font_size,
                self.metrics.line_height * metrics.line_height,
            ),
            url_map: self.url_map,
        }
    }

    pub fn new_default(text: Vec<RichText>) -> Self {
        Self {
            indent: Indent {
                modifier: None,
                indent: 0.0,
            },
            buffer: text,
            metrics: Metrics::new(1.0, 1.1),
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
        buffer.set_size(font_system, width - self.indent.indent, f32::MAX);

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
            metrics: Metrics::new(1.0, 1.1),
            added: false,
            link_start: 0,
            location: 0,
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
                    let mut map = RangeMap::new();
                    map.extend(
                        urls.into_iter()
                            .map(|(range, url)| (range.start..range.end + 1, url.into_owned())),
                    );
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
