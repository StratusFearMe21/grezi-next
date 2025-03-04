use std::{ops::Deref, sync::Arc};

use egui::{load::TexturePoll, Align2, ImageSize, Pos2, Rect, Stroke, Vec2};
use egui_glyphon::{
    cosmic_text::{fontdb::ID, Affinity, Align, Cursor, FontSystem},
    BufferWithTextArea,
};
use grezi_parser::{
    actions::{DrawableAction, SlideParams},
    object::ObjInner,
    slide::{SlideVb, VbIdentifier, ViewboxRef, BASE_SIZE},
    GrzRoot,
};
use indexmap::IndexMap;
use keyframe::EasingFunction;
use object::{ResolvedObjInner, ResolvedObject};
use smallvec::SmallVec;
use text::{resolve_text_job, selection_rects};

mod object;
mod text;

pub struct GrzResolvedSlide {
    objects: IndexMap<smartstring::alias::String, ResolvedObject>,
    pub params: SlideParams,
    pub size: Rect,
    pub factor: f32,
    pub max_time: f64,
}

impl GrzResolvedSlide {
    pub fn fonts_used(&self) -> SmallVec<[ID; 8]> {
        let mut fonts_used = SmallVec::new();

        for obj in self.objects.values() {
            match &obj.inner {
                ResolvedObjInner::Text { fonts, .. } => {
                    for font in fonts {
                        if !fonts_used.contains(font) {
                            fonts_used.push(*font);
                        }
                    }
                }
                _ => {}
            }
        }

        fonts_used
    }
}

impl GrzResolvedSlide {
    pub fn resolve_slide(
        root: &GrzRoot,
        size: Rect,
        font_system: &mut FontSystem,
        ctx: &egui::Context,
        index: usize,
    ) -> Option<Self> {
        let size = Align2::CENTER_CENTER.align_size_within_rect(
            ImageSize {
                max_size: size.size(),
                ..Default::default()
            }
            .calc_size(size.size(), BASE_SIZE.max.to_vec2()),
            size,
        );
        let (_, slide) = root.slides.get_index(index)?;
        let mut objects = IndexMap::new();

        let mut min_time = 0.0;
        let mut max_time = slide.slide_params.time;

        let scale_factor = if size.width() > size.height() {
            size.height() / BASE_SIZE.max.y
        } else {
            size.width() / BASE_SIZE.max.x
        };

        for (obj_name, slide_obj) in &slide.objects {
            let first_viewbox = match slide_obj.vb_from.as_ref().unwrap_or_else(|| {
                &SlideVb::Viewbox(ViewboxRef {
                    vb_name: VbIdentifier::Size,
                    subbox: 0,
                })
            }) {
                SlideVb::Viewbox(vb) => match &vb.vb_name {
                    VbIdentifier::Named(n) => {
                        root.viewboxes.get(n).unwrap().0.get(vb.subbox).unwrap()
                    }
                    VbIdentifier::Size => &BASE_SIZE,
                    VbIdentifier::Rect(r) => r,
                },
                SlideVb::InnerVb { split, subbox } => split.get(*subbox).unwrap(),
            };
            let second_viewbox = match slide_obj.viewbox.as_ref() {
                Some(SlideVb::Viewbox(vb)) => match &vb.vb_name {
                    VbIdentifier::Named(n) => {
                        root.viewboxes.get(n).unwrap().0.get(vb.subbox).unwrap()
                    }
                    VbIdentifier::Size => &BASE_SIZE,
                    VbIdentifier::Rect(r) => r,
                },
                Some(SlideVb::InnerVb { split, subbox }) => split.get(*subbox).unwrap(),
                None => panic!("Second viewbox not present"),
            };

            let first_viewbox = scale_viewbox(*first_viewbox, size, scale_factor);
            let second_viewbox = scale_viewbox(*second_viewbox, size, scale_factor);

            let obj = root.objects.get(obj_name).unwrap();

            let mut min_size = first_viewbox.size();
            let mut max_size = second_viewbox.size();

            let inner = match &obj.parameters {
                ObjInner::Rect { color, height } => {
                    min_size.y *= *height;
                    max_size.y *= *height;
                    ResolvedObjInner::Rect { color: *color }
                }
                ObjInner::Image {
                    data,
                    url,
                    scale,
                    tint,
                } => {
                    let image = egui::Image::from_bytes(url.to_string(), Arc::clone(data));
                    let image_poll;
                    loop {
                        match image.load_for_size(ctx, max_size).unwrap() {
                            TexturePoll::Pending { .. } => {}
                            TexturePoll::Ready { texture } => {
                                image_poll = texture;
                                break;
                            }
                        }
                    }

                    min_size = scale
                        .map(|s: f32| Vec2::splat(s * scale_factor))
                        .unwrap_or(min_size);
                    max_size = scale
                        .map(|s: f32| Vec2::splat(s * scale_factor))
                        .unwrap_or(max_size);

                    min_size = ImageSize {
                        max_size: min_size,
                        ..Default::default()
                    }
                    .calc_size(min_size, image_poll.size);

                    max_size = ImageSize {
                        max_size,
                        ..Default::default()
                    }
                    .calc_size(max_size, image_poll.size);

                    ResolvedObjInner::Image { image, tint: *tint }
                }
                ObjInner::Text {
                    job,
                    line_height,
                    align,
                } => {
                    let (size, obj) = resolve_text_job(
                        job,
                        *line_height,
                        match align {
                            grezi_parser::text::Align::Left => Align::Left,
                            grezi_parser::text::Align::Right => Align::Right,
                            grezi_parser::text::Align::Center => Align::Center,
                            grezi_parser::text::Align::Justified => Align::Justified,
                            grezi_parser::text::Align::End => Align::End,
                        },
                        font_system,
                        max_size.x,
                        scale_factor,
                    );
                    min_size = size;
                    max_size = size;
                    obj
                }
            };

            let min_pos = slide_obj
                .positions
                .from_alignment
                .unwrap_or(Align2::CENTER_CENTER)
                .align_size_within_rect(min_size, first_viewbox);
            let max_pos = slide_obj
                .positions
                .to_alignment
                .unwrap_or(Align2::CENTER_CENTER)
                .align_size_within_rect(max_size, second_viewbox);

            objects.insert(
                obj_name.clone(),
                ResolvedObject::new(
                    &slide.slide_params,
                    min_time,
                    min_pos,
                    max_pos,
                    slide_obj.positions.state,
                    inner,
                ),
            );

            if first_viewbox != second_viewbox
                || slide_obj.positions.to_alignment != slide_obj.positions.from_alignment
            {
                min_time += slide.slide_params.stagger;
                max_time += slide.slide_params.stagger;
            }
        }

        for action in &slide.actions {
            match action {
                DrawableAction::Highlight {
                    object: obj_name,
                    locations,
                    color,
                } => {
                    if let Some(object) = objects.get(obj_name) {
                        let mut rects = smallvec::smallvec![Rect::from_min_size(
                            Pos2::ZERO,
                            object.params.max_pos.size()
                        )];
                        if let Some(locations) = locations {
                            if let ResolvedObject {
                                inner: ResolvedObjInner::Text { job, .. },
                                ..
                            } = object
                            {
                                let mut markers_passed = 0;
                                for paragraph in locations[0][0]..=locations[1][0] {
                                    let buffer;
                                    loop {
                                        let Some(buf) = job.get(paragraph + markers_passed) else {
                                            continue;
                                        };

                                        if !buf.marker {
                                            buffer = buf;
                                            break;
                                        } else {
                                            markers_passed += 1;
                                        }
                                    }

                                    let mut start = Cursor::new(0, 0);
                                    let mut end = Cursor::new_with_affinity(
                                        usize::MAX,
                                        usize::MAX,
                                        Affinity::After,
                                    );

                                    if paragraph == locations[0][0] {
                                        rects.clear();
                                        start = Cursor::new(locations[0][1], locations[0][2]);
                                    }
                                    if paragraph == locations[1][0] {
                                        end = Cursor::new(locations[1][1], locations[1][2] + 1);
                                    }

                                    selection_rects(
                                        buffer.buffer.read().deref(),
                                        (start, end),
                                        buffer.buffer_rect,
                                        &mut rects,
                                    );
                                }
                            } else {
                                tracing::warn!("Highlight locations only apply to text objects");
                            }
                        }
                        let mut name = smartstring::alias::String::from("__highlight__");
                        name.push_str(obj_name.as_str());
                        objects.insert(
                            name,
                            object.new_based_on_this(ResolvedObjInner::Highlight {
                                rects,
                                color: *color,
                            }),
                        );
                    }
                }
                DrawableAction::Line {
                    objects: obj_names,
                    locations,
                    color,
                } => {
                    let mut name = smartstring::alias::String::from("__line__");
                    name.push_str(obj_names[0].as_str());
                    name.push_str(obj_names[1].as_str());
                    let Some(first_obj) = objects.get(&obj_names[0]) else {
                        continue;
                    };
                    let Some(second_obj) = objects.get(&obj_names[1]) else {
                        continue;
                    };
                    objects.insert(
                        name,
                        ResolvedObject::new(
                            &slide.slide_params,
                            second_obj.params.min_time,
                            Rect::ZERO,
                            Rect::ZERO,
                            second_obj.params.state,
                            ResolvedObjInner::Line {
                                objects: [first_obj.params, second_obj.params],
                                origin_positions: *locations,
                                stroke: Stroke::new(2.5 * scale_factor, *color),
                            },
                        ),
                    );
                }
            }
        }

        Some(Self {
            objects,
            max_time,
            size,
            factor: scale_factor,
            params: slide.slide_params.clone(),
        })
    }
}

fn scale_viewbox(viewbox: Rect, size: Rect, factor: f32) -> Rect {
    (viewbox * factor).translate(size.min.to_vec2())
}

impl GrzResolvedSlide {
    pub fn draw<E: EasingFunction>(
        &self,
        ui: &mut egui::Ui,
        time: f64,
        easing_function: &E,
        buffers: &mut Vec<BufferWithTextArea>,
    ) {
        for object in self.objects.values() {
            object.draw(ui, time, easing_function, buffers);
        }
    }
}
