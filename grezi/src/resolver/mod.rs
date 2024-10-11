use std::{collections::HashMap, hash::BuildHasherDefault, io::Cursor, ops::DerefMut, sync::Arc};

use eframe::egui::{
    self, mutex::Mutex, Align2, Image, ImageSize, OpenUrl, Pos2, Rect, Rounding, Sense, SizeHint,
    Stroke, Ui, Vec2,
};
use egui_anim::Anim;
use egui_glyphon::{
    glyphon::{cosmic_text::BufferRef, Edit, Editor, FontSystem, Metrics},
    BufferWithTextArea,
};
use image::codecs::{png::PngDecoder, webp::WebPDecoder};
use keyframe::functions::{EaseOutCubic, EaseOutQuint, Linear};
use layout::{Constraint, Direction};
use rangemap::RangeMap;

use crate::{
    parser::{
        actions::{Actions, ResolvedActions},
        objects::{serde_suck::CursorSerde, ObjectState, ResolvedObject},
        slides::{ResolvedSlideObj, SlideObj},
        viewboxes::ViewboxIn,
        PassThroughHasher,
    },
    SlideShow,
};

pub mod layout;

pub struct Resolved {
    pub viewboxes: HashMap<u64, Layouts, BuildHasherDefault<PassThroughHasher>>,
    pub actions: Vec<ResolvedActions>,
    pub slide: Vec<ResolvedSlideObj>,
    pub speaker_notes: Option<Arc<str>>,
    pub window_size: Rect,
    pub draw_size: Rect,
}

impl Default for Resolved {
    fn default() -> Self {
        Self {
            viewboxes: Default::default(),
            actions: Default::default(),
            slide: Default::default(),
            speaker_notes: None,
            window_size: Rect::ZERO,
            draw_size: Rect::ZERO,
        }
    }
}

impl Resolved {
    pub fn slideshow_end(size: Rect) -> Self {
        Self {
            window_size: size,
            ..Default::default()
        }
    }
    pub fn draw_slide(
        &self,
        ui: &mut Ui,
        time: f32,
        buffers: &mut Vec<BufferWithTextArea<Option<Arc<RangeMap<CursorSerde, String>>>>>,
        font_system: &mut FontSystem,
    ) {
        let current_clip = ui.clip_rect();
        ui.set_clip_rect(self.window_size);
        for obj in self.slide.iter() {
            let time = if obj.scaled_time[0] < time {
                (time - obj.scaled_time[0]).clamp(0.0, obj.scaled_time[1])
            } else {
                0.0
            };
            let obj_pos = Rect::from([
                Pos2::from(keyframe::ease_with_scaled_time(
                    EaseOutCubic,
                    obj.locations[0][0],
                    obj.locations[1][0],
                    time,
                    obj.scaled_time[1],
                )),
                Pos2::from(keyframe::ease_with_scaled_time(
                    EaseOutCubic,
                    obj.locations[0][1],
                    obj.locations[1][1],
                    time,
                    obj.scaled_time[1],
                )),
            ]);
            match &obj.object {
                ResolvedObject::Text(resolved_buffers) => {
                    let gamma_multiply = match obj.state {
                        ObjectState::Entering => keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            0.0,
                            1.0,
                            time,
                            obj.scaled_time[1],
                        ),
                        ObjectState::Exiting => keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            1.0,
                            0.0,
                            time,
                            obj.scaled_time[1],
                        ),
                        ObjectState::OnScreen => 1.0,
                    };
                    if gamma_multiply > 0.0 {
                        // ui.painter().debug_rect(obj_pos, Color32::RED, "");

                        for buffer in resolved_buffers {
                            let text_rect = buffer
                                .relative_bounds
                                .translate(obj_pos.min.to_vec2())
                                .expand(1.0);

                            // ui.painter().debug_rect(text_rect, Color32::GREEN, "");

                            buffers.push(BufferWithTextArea::new(
                                Arc::clone(&buffer.buffer),
                                text_rect,
                                gamma_multiply,
                                egui_glyphon::glyphon::Color::rgb(255, 255, 255),
                                ui.ctx(),
                                buffer.url_map.clone(),
                            ));

                            use egui_glyphon::glyphon;

                            if let Some(url_map) = &buffer.url_map {
                                let text_response = ui.allocate_rect(text_rect, Sense::click());
                                let mut buffer = buffer.buffer.write();
                                let mut editor =
                                    Editor::new(BufferRef::Borrowed(buffer.deref_mut()));
                                if text_response.hovered()
                                    && ui.input(|i| i.raw_scroll_delta == Vec2::ZERO)
                                {
                                    let mouse_pos = ui
                                        .input(|i| i.pointer.latest_pos().unwrap_or_default())
                                        - text_rect.min.to_vec2();

                                    editor.action(
                                        font_system,
                                        glyphon::Action::Click {
                                            x: mouse_pos.x as i32,
                                            y: mouse_pos.y as i32 - 3,
                                        },
                                    );

                                    let location: crate::parser::objects::serde_suck::CursorSerde =
                                        editor.cursor().into();

                                    if url_map.get(&location).is_some() {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }
                                }
                                if text_response.clicked() {
                                    let mouse_click = ui
                                        .input(|i| i.pointer.interact_pos().unwrap_or_default())
                                        - text_rect.min.to_vec2();

                                    editor.action(
                                        font_system,
                                        glyphon::Action::Click {
                                            x: mouse_click.x as i32,
                                            y: mouse_click.y as i32,
                                        },
                                    );

                                    let location: crate::parser::objects::serde_suck::CursorSerde =
                                        editor.cursor().into();
                                    if let Some(url) = url_map.get(&location) {
                                        ui.ctx().open_url(OpenUrl::new_tab(url));
                                        // clicked = false;
                                    }
                                }
                            }
                        }
                    }
                }
                ResolvedObject::Image {
                    image,
                    mut tint,
                    scale,
                } => {
                    let gamma = match obj.state {
                        ObjectState::Entering => keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            0.0,
                            1.0,
                            time,
                            obj.scaled_time[1],
                        ),
                        ObjectState::Exiting => keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            1.0,
                            0.0,
                            time,
                            obj.scaled_time[1],
                        ),
                        ObjectState::OnScreen => 1.0,
                    };
                    tint = tint.gamma_multiply(gamma);
                    if tint.a() > 0 {
                        image
                            .clone()
                            .fit_to_exact_size(scale.unwrap_or_else(|| obj_pos.size()))
                            .tint(tint)
                            .paint_at(ui, obj_pos);
                    }
                }
                ResolvedObject::Anim {
                    anim,
                    mut tint,
                    scale,
                } => {
                    match obj.state {
                        ObjectState::Entering => {
                            tint = tint.gamma_multiply(keyframe::ease_with_scaled_time(
                                EaseOutCubic,
                                0.0,
                                1.0,
                                time,
                                obj.scaled_time[1],
                            ));
                        }
                        ObjectState::Exiting => {
                            tint = tint.gamma_multiply(keyframe::ease_with_scaled_time(
                                EaseOutCubic,
                                1.0,
                                0.0,
                                time,
                                obj.scaled_time[1],
                            ));
                        }
                        ObjectState::OnScreen => {}
                    }
                    if tint.a() > 0 {
                        Image::from_uri(anim.find_img(ui.ctx()))
                            .fit_to_exact_size(scale.unwrap_or_else(|| obj_pos.size()))
                            .tint(tint)
                            .paint_at(ui, obj_pos);
                    }
                }
                ResolvedObject::Rect { color, rect } => {
                    ui.painter().rect_filled(
                        rect.translate(obj_pos.min.to_vec2()),
                        Rounding::ZERO,
                        *color,
                    );
                }
                ResolvedObject::Spinner => egui::Spinner::new().paint_at(ui, obj_pos),
            };
        }
        ui.set_clip_rect(current_clip);
    }

    pub fn draw_actions(&self, ui: &mut Ui, time: f32, export: bool) {
        for action in &self.actions {
            match action {
                crate::parser::actions::ResolvedActions::Highlight {
                    locations,
                    persist,
                    locations_of_object,
                    scaled_time,
                    color,
                } => {
                    let time = if scaled_time[0] < time {
                        time - scaled_time[0]
                    } else {
                        0.0
                    };
                    let obj_pos = Vec2::from(keyframe::ease_with_scaled_time(
                        EaseOutCubic,
                        locations_of_object[0],
                        locations_of_object[1],
                        if !(*persist || export) {
                            scaled_time[1]
                        } else {
                            time
                        },
                        scaled_time[1],
                    ));
                    ui.ctx().debug_painter().rect_filled(
                        Rect {
                            min: Pos2::new(
                                if *persist || export {
                                    locations.min.x
                                } else {
                                    keyframe::ease_with_scaled_time(
                                        Linear,
                                        locations.min.x,
                                        locations.max.x,
                                        time,
                                        scaled_time[1],
                                    )
                                },
                                locations.min.y,
                            ),
                            max: Pos2::new(
                                keyframe::ease_with_scaled_time(
                                    EaseOutQuint,
                                    locations.min.x,
                                    locations.max.x,
                                    time,
                                    scaled_time[1],
                                ),
                                locations.max.y,
                            ),
                        }
                        .translate(obj_pos),
                        Rounding::ZERO,
                        *color,
                    );
                }
                ResolvedActions::SpeakerNotes(_) => {}
                ResolvedActions::Line {
                    locations_of_objects,
                    scaled_times,
                    color,
                    state,
                    scale,
                } => {
                    let first_time = if scaled_times[0][0] < time {
                        (time - scaled_times[0][0]).clamp(0.0, scaled_times[0][1])
                    } else {
                        0.0
                    };
                    let second_time = if scaled_times[1][0] < time {
                        (time - scaled_times[1][0]).clamp(0.0, scaled_times[1][1])
                    } else {
                        0.0
                    };
                    let first_obj_pos = Pos2::from(keyframe::ease_with_scaled_time(
                        EaseOutCubic,
                        locations_of_objects[0][0],
                        locations_of_objects[0][1],
                        first_time,
                        scaled_times[0][1],
                    ));
                    let mut second_obj_pos = Pos2::from(keyframe::ease_with_scaled_time(
                        EaseOutCubic,
                        locations_of_objects[1][0],
                        locations_of_objects[1][1],
                        second_time,
                        scaled_times[1][1],
                    ));

                    let gamma = match state {
                        ObjectState::Entering => {
                            let y = keyframe::ease_with_scaled_time(
                                EaseOutCubic,
                                0.0,
                                1.0,
                                second_time,
                                scaled_times[1][1],
                            );
                            second_obj_pos = first_obj_pos.lerp(second_obj_pos, y);
                            y
                        }
                        ObjectState::Exiting => {
                            let y = keyframe::ease_with_scaled_time(
                                EaseOutCubic,
                                1.0,
                                0.0,
                                second_time,
                                scaled_times[1][1],
                            );
                            second_obj_pos = first_obj_pos.lerp(second_obj_pos, y);
                            y
                        }
                        ObjectState::OnScreen => 1.0,
                    };

                    if gamma > 0.0 {
                        ui.painter().line_segment(
                            [first_obj_pos, second_obj_pos],
                            Stroke::new(2.5 * *scale, color.gamma_multiply(gamma)),
                        );
                    }
                }
            }
        }
    }

    // TODO: Remove one of the get calls
    fn resolve_layout(
        &mut self,
        hash: u64,
        index: usize,
        size: Rect,
        slide_show: &SlideShow,
    ) -> Splits {
        match self.viewboxes.get(&hash) {
            None => {
                let split = match slide_show.viewboxes.get(&hash).unwrap().split_on {
                    ViewboxIn::Size => Splits {
                        unadjusted: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
                        adjusted: size,
                    },
                    ViewboxIn::Custom(hash, index) => {
                        self.resolve_layout(hash, index, size, slide_show)
                    }
                    ViewboxIn::Inherit(_) => unreachable!(),
                };

                let unresolved_layout = slide_show.viewboxes.get(&hash).unwrap();
                let constraints = unresolved_layout.constraints.clone();
                let layout = resolve_layout_raw(
                    size,
                    unresolved_layout.direction,
                    constraints,
                    split,
                    unresolved_layout.margin,
                );
                let splits = Splits {
                    unadjusted: layout.unadjusted[index],
                    adjusted: layout.adjusted[index],
                };
                self.viewboxes.insert(hash, layout);
                splits
            }
            Some(layout) => Splits {
                unadjusted: layout.unadjusted[index],
                adjusted: layout.adjusted[index],
            },
        }
    }

    pub fn resolve(
        slide: &[SlideObj],
        actions: (&[Actions], Option<&[Actions]>),
        ui: &mut Ui,
        size_raw: Rect,
        slide_show: &SlideShow,
        font_system: &mut FontSystem,
        resolved_images: Arc<
            Mutex<HashMap<u64, ResolvedObject, BuildHasherDefault<PassThroughHasher>>>,
        >,
        export: bool,
    ) -> Self {
        let mut resolved = Resolved::default();
        resolved.draw_size = {
            let size = size_raw.size();
            let size = ImageSize {
                max_size: size,
                ..Default::default()
            }
            .calc_size(size, Vec2::new(16.0, 9.0));
            Align2::CENTER_CENTER.align_size_within_rect(size, size_raw)
        };
        resolved.window_size = size_raw;
        let mut images = Vec::with_capacity(3);
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => resolve_layout_raw(
                    resolved.draw_size,
                    Direction::Horizontal,
                    vec![Constraint::Min(0.0)],
                    Splits {
                        unadjusted: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
                        adjusted: resolved.draw_size,
                    },
                    15.0,
                )
                .get_splits(0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, resolved.draw_size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => resolve_layout_raw(
                    resolved.draw_size,
                    Direction::Horizontal,
                    vec![Constraint::Min(0.0)],
                    Splits {
                        unadjusted: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
                        adjusted: resolved.draw_size,
                    },
                    15.0,
                )
                .get_splits(0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, resolved.draw_size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };

            let obj = slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                crate::parser::objects::ObjectType::Spinner => {
                    let size = ResolvedObject::Spinner.bounds(second_viewbox.adjusted.size(), ui);
                    let first_pos = object.locations[0]
                        .0
                        .align_size_within_rect(size.size(), first_viewbox.adjusted);
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = object.locations[1]
                        .0
                        .align_size_within_rect(size.size(), second_viewbox.adjusted);
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: ResolvedObject::Spinner,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                crate::parser::objects::ObjectType::Rect { color, height } => {
                    let mut rect = second_viewbox.adjusted.translate(Vec2::new(
                        -second_viewbox.adjusted.min.x,
                        -second_viewbox.adjusted.min.y,
                    ));
                    rect.max.y = *height * (resolved.draw_size.height() / 1080.0);
                    let resolved_obj = ResolvedObject::Rect {
                        color: *color,
                        rect,
                    };

                    let size = resolved_obj.bounds(second_viewbox.adjusted.size(), ui);
                    let first_pos = object.locations[0]
                        .0
                        .align_size_within_rect(size.size(), first_viewbox.adjusted);
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = object.locations[1]
                        .0
                        .align_size_within_rect(size.size(), second_viewbox.adjusted);
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: resolved_obj,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                crate::parser::objects::ObjectType::Text {
                    job,
                    font_size,
                    line_height,
                    align,
                    spacing,
                } => {
                    let factor = second_viewbox.adjusted.size() / second_viewbox.unadjusted.size();
                    let font_size = *font_size * factor.x;
                    let (size, resolved_job) =
                        crate::parser::objects::cosmic_jotdown::resolve_paragraphs(
                            job.as_slice(),
                            second_viewbox.adjusted.size(),
                            font_system,
                            Metrics::new(
                                font_size,
                                line_height.map_or(font_size * 1.2, |h| h * font_size),
                            ),
                            *align,
                            factor.x,
                            *spacing,
                        );

                    let resolved_obj = ResolvedObject::Text(resolved_job);
                    let first_pos = object.locations[0]
                        .0
                        .align_size_within_rect(size, first_viewbox.adjusted);

                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = object.locations[1]
                        .0
                        .align_size_within_rect(size, second_viewbox.adjusted);
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: resolved_obj,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                crate::parser::objects::ObjectType::Image {
                    uri,
                    bytes,
                    tint,
                    scale,
                } => {
                    match images.binary_search(&object.object) {
                        Err(index) | Ok(index) => images.insert(index, object.object),
                    }
                    let mut resolved_images = resolved_images.lock();
                    let resolved_obj = resolved_images
                        .entry(object.object)
                        .and_modify(|obj| match obj {
                            ResolvedObject::Image {
                                tint: t, scale: s, ..
                            }
                            | ResolvedObject::Anim {
                                tint: t, scale: s, ..
                            } => {
                                *t = *tint;
                                *s = *scale;
                            }
                            _ => {}
                        })
                        .or_insert_with(|| {
                            if !export {
                                match uri.rsplit_once('.') {
                                    Some((_, "gif")) => {
                                        ui.ctx().include_bytes(uri.clone(), Arc::clone(bytes));
                                        return ResolvedObject::Anim {
                                            anim: Anim::new(
                                                ui.ctx(),
                                                &format!("{}\0gif", uri),
                                                SizeHint::default(),
                                            ),
                                            tint: *tint,
                                            scale: *scale,
                                        };
                                    }
                                    Some((_, "apng" | "png")) => {
                                        ui.ctx().include_bytes(uri.clone(), Arc::clone(bytes));
                                        let decoder =
                                            PngDecoder::new(Cursor::new(bytes.as_ref())).unwrap();
                                        if decoder.is_apng().unwrap_or_default() {
                                            return ResolvedObject::Anim {
                                                anim: Anim::new(
                                                    ui.ctx(),
                                                    &format!("{}\0apng", uri),
                                                    SizeHint::default(),
                                                ),
                                                tint: *tint,
                                                scale: *scale,
                                            };
                                        }
                                    }
                                    Some((_, "webp")) => {
                                        ui.ctx().include_bytes(uri.clone(), Arc::clone(bytes));
                                        let decoder =
                                            WebPDecoder::new(Cursor::new(bytes.as_ref())).unwrap();
                                        if decoder.has_animation() {
                                            return ResolvedObject::Anim {
                                                anim: Anim::new(
                                                    ui.ctx(),
                                                    &format!("{}\0webp", uri),
                                                    SizeHint::default(),
                                                ),
                                                tint: *tint,
                                                scale: *scale,
                                            };
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ResolvedObject::Image {
                                image: Image::from_bytes(uri.clone(), Arc::clone(bytes)),
                                tint: *tint,
                                scale: *scale,
                            }
                        });

                    let first_size = resolved_obj
                        .bounds(scale.unwrap_or_else(|| first_viewbox.adjusted.size()), ui);
                    let second_size = resolved_obj
                        .bounds(scale.unwrap_or_else(|| second_viewbox.adjusted.size()), ui);
                    let first_pos = object.locations[0]
                        .0
                        .align_size_within_rect(first_size.size(), first_viewbox.adjusted);
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = object.locations[1]
                        .0
                        .align_size_within_rect(second_size.size(), second_viewbox.adjusted);
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: resolved_obj.clone(),
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
            }
        }
        for action in actions.0.into_iter().chain(actions.1.into_iter().flatten()) {
            match action {
                Actions::Highlight {
                    locations,
                    index,
                    persist,
                    color,
                } => {
                    let text_object = resolved.slide.get(*index).unwrap();
                    let locations = if let Some(locations) = locations {
                        let (from_rect, to_rect) = match &text_object.object {
                            ResolvedObject::Text(buffer) => (
                                {
                                    let buffer = buffer[0].buffer.read();
                                    let glyph = buffer.lines.get(locations[0][0]).unwrap();
                                    let glyph = glyph.layout_opt().as_ref().unwrap();
                                    let glyph = glyph
                                        .iter()
                                        .flat_map(|g| &g.glyphs)
                                        .take(locations[0][1] + 1)
                                        .last()
                                        .unwrap();

                                    Rect::from_min_size(
                                        Pos2::new(
                                            glyph.x,
                                            glyph.y
                                                + buffer.metrics().line_height
                                                    * locations[0][0] as f32,
                                        ),
                                        Vec2::new(0.0, glyph.y_offset),
                                    )
                                },
                                {
                                    let buffer = buffer[0].buffer.read();
                                    let glyph = buffer.lines.get(locations[1][0]).unwrap();
                                    let glyph = glyph.layout_opt().as_ref().unwrap();
                                    let glyph = glyph
                                        .iter()
                                        .flat_map(|g| &g.glyphs)
                                        .take(locations[1][1])
                                        .last()
                                        .unwrap();

                                    Rect::from_min_size(
                                        Pos2::new(
                                            glyph.x,
                                            glyph.y
                                                + buffer.metrics().line_height
                                                    * locations[1][0] as f32,
                                        ),
                                        Vec2::new(
                                            glyph.w,
                                            glyph.y_offset + buffer.metrics().line_height,
                                        ),
                                    )
                                },
                            ),
                            _ => todo!(),
                        };
                        from_rect.union(to_rect)
                    } else {
                        let to_rect = Rect::from([
                            Pos2::from(text_object.locations[1][0]),
                            Pos2::from(text_object.locations[1][1]),
                        ]);
                        Rect::from_min_size(Pos2::new(0.0, 0.0), to_rect.size())
                    };
                    let scaled_time = if text_object.scaled_time[1] < 0.1 {
                        [0.0, 0.0]
                    } else {
                        text_object.scaled_time
                    };

                    resolved.actions.push(ResolvedActions::Highlight {
                        locations,
                        persist: *persist,
                        locations_of_object: [
                            text_object.locations[0][0],
                            text_object.locations[1][0],
                        ],
                        scaled_time,
                        color: *color,
                    });
                }
                Actions::SpeakerNotes(speaker_notes) => {
                    resolved.speaker_notes = Some(Arc::clone(speaker_notes))
                }
                Actions::Line {
                    objects,
                    locations,
                    color,
                } => {
                    let object_one = resolved.slide.get(objects[0]).unwrap();
                    let object_two = resolved.slide.get(objects[1]).unwrap();

                    let first_obj_pos_min = Rect::from([
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_one.locations[0][0],
                            object_one.locations[1][0],
                            0.0,
                            object_one.scaled_time[1],
                        )),
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_one.locations[0][1],
                            object_one.locations[1][1],
                            0.0,
                            object_one.scaled_time[1],
                        )),
                    ]);
                    let first_obj_pos_max = Rect::from([
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_one.locations[0][0],
                            object_one.locations[1][0],
                            object_one.scaled_time[1],
                            object_one.scaled_time[1],
                        )),
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_one.locations[0][1],
                            object_one.locations[1][1],
                            object_one.scaled_time[1],
                            object_one.scaled_time[1],
                        )),
                    ]);

                    let second_obj_pos_min = Rect::from([
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_two.locations[0][0],
                            object_two.locations[1][0],
                            0.0,
                            object_two.scaled_time[1],
                        )),
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_two.locations[0][1],
                            object_two.locations[1][1],
                            0.0,
                            object_two.scaled_time[1],
                        )),
                    ]);
                    let second_obj_pos_max = Rect::from([
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_two.locations[0][0],
                            object_two.locations[1][0],
                            object_two.scaled_time[1],
                            object_two.scaled_time[1],
                        )),
                        Pos2::from(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            object_two.locations[0][1],
                            object_two.locations[1][1],
                            object_two.scaled_time[1],
                            object_two.scaled_time[1],
                        )),
                    ]);

                    resolved.actions.push(ResolvedActions::Line {
                        locations_of_objects: [
                            [
                                locations[0].pos_in_rect(&first_obj_pos_min).into(),
                                locations[0].pos_in_rect(&first_obj_pos_max).into(),
                            ],
                            [
                                locations[1].pos_in_rect(&second_obj_pos_min).into(),
                                locations[1].pos_in_rect(&second_obj_pos_max).into(),
                            ],
                        ],
                        scaled_times: [object_one.scaled_time, object_two.scaled_time],
                        color: *color,
                        state: object_two.state,
                        scale: (resolved.draw_size.width() / 1920.0),
                    });
                }
            }
        }
        resolved
    }
}

pub struct Layouts {
    pub unadjusted: Vec<Rect>,
    pub adjusted: Vec<Rect>,
}

impl Layouts {
    fn get_splits(&self, idx: usize) -> Splits {
        Splits {
            unadjusted: self.unadjusted[idx],
            adjusted: self.adjusted[idx],
        }
    }
}

struct Splits {
    unadjusted: Rect,
    adjusted: Rect,
}

fn resolve_layout_raw(
    size: Rect,
    direction: Direction,
    constraints: Vec<Constraint>,
    splits: Splits,
    margin: f32,
) -> Layouts {
    let unadjusted = layout::Layout::default()
        .direction(direction)
        .margin(margin)
        .constraints(&constraints)
        .split(splits.unadjusted)
        .unwrap();
    let factor = match direction {
        Direction::Horizontal => size.width() / 1920.0,
        Direction::Vertical => size.height() / 1080.0,
    };
    let adjusted = unadjusted
        .iter()
        .map(|r| (*r * factor).translate(size.min.to_vec2()))
        .collect();

    Layouts {
        unadjusted,
        adjusted,
    }
}
