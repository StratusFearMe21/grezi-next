use std::{ops::Deref, sync::Arc};

use egui::{mutex::RwLock, Align2, Pos2, Rect, Vec2};
use egui_glyphon::{
    cosmic_text::{
        fontdb::ID, Align, Attrs, Buffer, CacheKeyFlags, Color, Cursor, Family, FontSystem,
        Metrics, Stretch, Style, Weight,
    },
    measure_buffer,
};
use grezi_parser::text::{TextParagraph, TextSection, TextTag};
use smallvec::SmallVec;
use unicode_segmentation::UnicodeSegmentation;

use crate::object::ResolvedObjInner;

const INDENT_AMOUNT: f32 = 75.0;
const MARGIN_MULTIPLIER: f32 = 0.5;
const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;

#[derive(Clone, Copy)]
pub enum ResolvedTextTag {
    SectStart,
    SectEnd,
    ListStart,
    ListEnd,
    ListItemStart,
    ListItemEnd,
    ListBodyStart,
    ListBodyEnd,
    BlockquoteStart,
    BlockquoteEnd,
    Paragraph(usize),
    Untagged(usize),
    Code(usize),
    Heading(u16, usize),
    Label(usize),
}

impl ResolvedTextTag {
    pub fn offset(mut self, amount: usize) -> Self {
        match &mut self {
            Self::Paragraph(buffer_index)
            | Self::Untagged(buffer_index)
            | Self::Heading(_, buffer_index)
            | Self::Label(buffer_index) => *buffer_index += amount,
            _ => {}
        }

        self
    }
}

pub struct ResolvedBuffer {
    pub buffer: Arc<RwLock<Buffer>>,
    pub buffer_rect: Rect,
    /// Is the buffer a list number or
    /// bullet point
    pub marker: bool,
}

pub fn resolve_text_job(
    sections: &[TextSection],
    line_height: Option<f32>,
    alignment: Align,
    font_system: &mut FontSystem,
    max_width: f32,
) -> (Vec2, ResolvedObjInner) {
    let mut job: SmallVec<[ResolvedBuffer; 1]> = SmallVec::new();
    let mut tags: SmallVec<[ResolvedTextTag; 3]> = SmallVec::new();
    // First pass:
    //
    // Translates text on X axis and creates
    // buffers
    let size_x = resolve_text_job_first_pass(
        sections,
        &mut job,
        &mut tags,
        font_system,
        line_height,
        alignment,
        Vec2::ZERO,
        max_width,
        0.0,
    );

    let align_egui = match alignment {
        Align::Left | Align::Justified => egui::Align::LEFT,
        Align::Right | Align::End => egui::Align::RIGHT,
        Align::Center => egui::Align::Center,
    };

    let mut fonts: SmallVec<[ID; 8]> = SmallVec::new();
    let mut last_margin = 0.0;
    let mut translation = Vec2::ZERO;
    let mut last_buffer_marker = false;
    let mut last_buffer_size = Rect::ZERO;

    // Second pass:
    //
    // Translates text on Y axis and aligns
    // after all buffer sizes are known
    //
    // Also collects all fonts used
    for buffer in &mut job {
        let font_size = {
            let buffer = buffer.buffer.read();
            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    if !fonts.contains(&glyph.font_id) {
                        fonts.push(glyph.font_id);
                    }
                }
            }
            buffer.metrics().font_size
        };
        // We use buffer.min.y to store whether the buffer that came before
        // it was a list number or not
        if last_buffer_marker {
            translation.y += buffer.buffer_rect.height() - last_buffer_size.height();
        } else {
            translation.y += last_margin + buffer.buffer_rect.height();
        }
        last_buffer_marker = buffer.marker;
        last_buffer_size = buffer.buffer_rect;
        buffer.buffer_rect = Align2([align_egui, egui::Align::BOTTOM]).align_size_within_rect(
            buffer.buffer_rect.size(),
            Rect::from_min_size(
                Pos2::new(buffer.buffer_rect.min.x, 0.0),
                Vec2::new(size_x - buffer.buffer_rect.min.x, translation.y),
            ),
        );
        last_margin = font_size * MARGIN_MULTIPLIER;
    }

    (
        Vec2::new(size_x, translation.y),
        ResolvedObjInner::Text { job, tags, fonts },
    )
}

pub fn resolve_text_job_first_pass(
    sections: &[TextSection],
    job: &mut SmallVec<[ResolvedBuffer; 1]>,
    tags: &mut SmallVec<[ResolvedTextTag; 3]>,
    font_system: &mut FontSystem,
    line_height: Option<f32>,
    alignment: Align,
    translation: Vec2,
    max_width: f32,
    mut size_x: f32,
) -> f32 {
    if translation.x == 0.0 {
        tags.push(ResolvedTextTag::SectStart);
    }
    for section in sections {
        match section {
            TextSection::Paragraph(p) => {
                let (paragraph, buffer_size) = resolve_text_paragraph(
                    p,
                    line_height,
                    alignment,
                    font_system,
                    max_width - translation.x,
                );
                size_x = size_x.max(buffer_size.width() + translation.x);

                match p.tag {
                    Some(TextTag::Paragraph) => tags.push(ResolvedTextTag::Paragraph(job.len())),
                    Some(TextTag::Heading(hl)) => {
                        tags.push(ResolvedTextTag::Heading(hl, job.len()))
                    }
                    Some(TextTag::Label) => tags.push(ResolvedTextTag::Label(job.len())),
                    Some(TextTag::Code) => tags.push(ResolvedTextTag::Code(job.len())),
                    None => tags.push(ResolvedTextTag::Untagged(job.len())),
                }
                job.push(ResolvedBuffer {
                    buffer: Arc::new(RwLock::new(paragraph)),
                    buffer_rect: buffer_size.translate(translation),
                    marker: false,
                });
            }
            TextSection::Blockquote(bq) => {
                tags.push(ResolvedTextTag::BlockquoteStart);
                size_x = size_x.max(resolve_text_job_first_pass(
                    bq,
                    job,
                    tags,
                    font_system,
                    line_height,
                    alignment,
                    translation + Vec2::new(INDENT_AMOUNT, 0.0),
                    max_width - INDENT_AMOUNT,
                    size_x,
                ));
                tags.push(ResolvedTextTag::BlockquoteEnd);
            }
            TextSection::List(list) => {
                tags.push(ResolvedTextTag::ListStart);
                for (list_number, list_item) in list {
                    tags.push(ResolvedTextTag::ListItemStart);
                    let (paragraph, buffer_size) = resolve_text_paragraph(
                        list_number,
                        line_height,
                        alignment,
                        font_system,
                        max_width - translation.x,
                    );
                    size_x = size_x.max(buffer_size.width());

                    match list_number.tag {
                        Some(TextTag::Label) => tags.push(ResolvedTextTag::Label(job.len())),
                        Some(t) => tracing::warn!("Wacky tag `{:?}` on list label", t),
                        None => tags.push(ResolvedTextTag::Untagged(job.len())),
                    }
                    job.push(ResolvedBuffer {
                        buffer: Arc::new(RwLock::new(paragraph)),
                        buffer_rect: buffer_size.translate(translation),
                        marker: true,
                    });

                    tags.push(ResolvedTextTag::ListBodyStart);
                    size_x = size_x.max(resolve_text_job_first_pass(
                        list_item,
                        job,
                        tags,
                        font_system,
                        line_height,
                        alignment,
                        translation + Vec2::new(INDENT_AMOUNT, 0.0),
                        max_width - INDENT_AMOUNT,
                        size_x,
                    ));
                    tags.push(ResolvedTextTag::ListBodyEnd);
                    tags.push(ResolvedTextTag::ListItemEnd);
                }
                tags.push(ResolvedTextTag::ListEnd);
            }
        }
    }
    if translation.x == 0.0 {
        tags.push(ResolvedTextTag::SectEnd);
    }
    size_x
}

pub fn resolve_text_paragraph(
    paragraph: &TextParagraph,
    line_height: Option<f32>,
    alignment: Align,
    font_system: &mut FontSystem,
    max_width: f32,
) -> (Buffer, Rect) {
    let mut buffer = Buffer::new(
        font_system,
        Metrics::new(
            paragraph.font_size,
            line_height.unwrap_or(paragraph.font_size * LINE_HEIGHT_MULTIPLIER),
        ),
    );

    buffer.set_size(font_system, Some(max_width), None);
    buffer.set_rich_text(
        font_system,
        paragraph.rich_text.iter().map(|(rich_span, attrs)| {
            (
                rich_span.as_str(),
                Attrs {
                    color_opt: {
                        let color = attrs.color.to_srgba_unmultiplied();
                        Some(Color::rgba(color[0], color[1], color[2], color[3]))
                    },
                    family: match &attrs.family {
                        grezi_parser::text::Family::Name(n) => Family::Name(n.deref()),
                        grezi_parser::text::Family::Serif => Family::Serif,
                        grezi_parser::text::Family::SansSerif => Family::SansSerif,
                        grezi_parser::text::Family::Cursive => Family::Cursive,
                        grezi_parser::text::Family::Fantasy => Family::Fantasy,
                        grezi_parser::text::Family::Monospace => Family::Monospace,
                    },
                    stretch: match attrs.stretch {
                        grezi_parser::text::Stretch::UltraCondensed => Stretch::UltraCondensed,
                        grezi_parser::text::Stretch::ExtraCondensed => Stretch::ExtraCondensed,
                        grezi_parser::text::Stretch::Condensed => Stretch::Condensed,
                        grezi_parser::text::Stretch::SemiCondensed => Stretch::SemiCondensed,
                        grezi_parser::text::Stretch::Normal => Stretch::Normal,
                        grezi_parser::text::Stretch::SemiExpanded => Stretch::SemiExpanded,
                        grezi_parser::text::Stretch::Expanded => Stretch::Expanded,
                        grezi_parser::text::Stretch::ExtraExpanded => Stretch::ExtraExpanded,
                        grezi_parser::text::Stretch::UltraExpanded => Stretch::UltraExpanded,
                    },
                    style: match attrs.style {
                        grezi_parser::text::Style::Normal => Style::Normal,
                        grezi_parser::text::Style::Italic => Style::Italic,
                        grezi_parser::text::Style::Oblique => Style::Oblique,
                    },
                    // TODO: Use for links
                    metadata: 0,
                    weight: Weight(attrs.weight.0),
                    cache_key_flags: CacheKeyFlags::empty(),
                    metrics_opt: None,
                },
            )
        }),
        Attrs::new(),
        egui_glyphon::cosmic_text::Shaping::Advanced,
    );

    buffer.shape_until_scroll(font_system, false);
    // Expand the buffer_size keeping the top left at 0,0
    let mut buffer_size = measure_buffer(&buffer);
    buffer_size.max += Vec2::splat(5.0);
    buffer.set_size(
        font_system,
        Some(buffer_size.width()),
        Some(buffer_size.height()),
    );
    for line in &mut buffer.lines {
        line.set_align(Some(alignment));
    }

    buffer.shape_until_scroll(font_system, true);

    (buffer, buffer_size)
}

pub fn selection_rects(
    buffer: &Buffer,
    selection_bounds: (Cursor, Cursor),
    buffer_rect: Rect,
    rects: &mut SmallVec<[Rect; 2]>,
) {
    let (start, end) = selection_bounds;
    for run in buffer.layout_runs() {
        let line_i = run.line_i;
        let line_top = run.line_top;
        let line_height = run.line_height;

        // Highlight selection
        if line_i >= start.line && line_i <= end.line {
            let mut range_opt: Option<(f32, f32)> = None;
            for glyph in run.glyphs.iter() {
                // Guess x offset based on characters
                let cluster = &run.text[glyph.start..glyph.end];
                let total = cluster.grapheme_indices(true).count();
                let mut c_x = glyph.x;
                let c_w = glyph.w / total as f32;
                for (i, c) in cluster
                    .grapheme_indices(true)
                    .skip_while(|g| g.1.split_whitespace().next().is_none())
                {
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
                            Pos2::new(min, line_top) + buffer_rect.min.to_vec2(),
                            Vec2::new((max - min).max(0.0), line_height),
                        ));
                        // f(
                        //     min,
                        //     line_top as i32,
                        //     0.0f32.max(max - min),
                        //     line_height as u32,
                        // );
                    }
                    c_x += c_w;
                }
            }

            if run.glyphs.is_empty() && end.line > line_i {
                // Highlight all of internal empty lines
                range_opt = Some((0.0, buffer.size().0.unwrap_or(0.0)));
            }

            if let Some((min, max)) = range_opt.take() {
                // if end.line > line_i {
                //     // Draw to end of line
                //     if run.rtl {
                //         min = 0.0;
                //     } else {
                //         max = buffer.size().0.unwrap_or(0.0);
                //     }
                // }
                rects.push(Rect::from_min_size(
                    Pos2::new(min, line_top) + buffer_rect.min.to_vec2(),
                    Vec2::new((max - min).max(0.0), line_height),
                ));
                // f(
                //     min,
                //     line_top as i32,
                //     0.0f32.max(max - min),
                //     line_height as u32,
                // );
            }
        }
    }
}
