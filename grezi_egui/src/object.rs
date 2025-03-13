use std::sync::Arc;

use egui::{Align2, Color32, CornerRadius, Image, Rect, Stroke};
use egui_glyphon::{cosmic_text::fontdb::ID, BufferWithTextArea};
use grezi_parser::{actions::SlideParams, slide::ObjState};
use keyframe::EasingFunction;
use smallvec::SmallVec;

use crate::text::ResolvedBuffer;

#[derive(Clone, Copy)]
pub struct ResolvedObjPositions {
    pub min_time: f64,
    pub max_time: f64,
    /// These rects will be the size of the object
    /// in pixels on the screen
    pub min_pos: Rect,
    pub max_pos: Rect,
    pub state: ObjState,
}

/// `ResolvedObject` represents an object to be drawn
/// after the scale factor and object bounds have been
/// taken into account
pub struct ResolvedObject {
    pub params: ResolvedObjPositions,
    pub inner: ResolvedObjInner,
}

impl ResolvedObject {
    pub fn new(
        params: &SlideParams,
        min_time: f64,
        min_pos: Rect,
        max_pos: Rect,
        state: ObjState,
        inner: ResolvedObjInner,
    ) -> Self {
        Self {
            params: ResolvedObjPositions {
                min_time,
                max_time: params.time,
                min_pos,
                max_pos,
                state,
            },
            inner,
        }
    }

    pub fn new_based_on_this(&self, inner: ResolvedObjInner) -> Self {
        Self {
            params: ResolvedObjPositions {
                min_time: self.params.min_time,
                max_time: self.params.max_time,
                min_pos: self.params.min_pos,
                max_pos: self.params.max_pos,
                state: self.params.state,
            },
            inner,
        }
    }
}

pub enum ResolvedObjInner {
    Text {
        job: SmallVec<[ResolvedBuffer; 1]>,
        fonts: SmallVec<[ID; 8]>,
    },
    Image {
        image: Image<'static>,
        tint: Color32,
    },
    Rect {
        color: Color32,
        stroke: Stroke,
    },
    Highlight {
        rects: SmallVec<[Rect; 2]>,
        color: Color32,
        state: ObjState,
    },
    Line {
        objects: [ResolvedObjPositions; 2],
        origin_positions: [Align2; 2],
        stroke: Stroke,
    },
}

impl ResolvedObject {
    pub fn draw<E: EasingFunction>(
        &self,
        ui: &mut egui::Ui,
        size: Rect,
        scale_factor: f32,
        time: f64,
        easing_function: &E,
        buffers: &mut Vec<egui_glyphon::BufferWithTextArea>,
    ) {
        let eased_time = if self.params.max_time > 0.0 {
            keyframe::ease_with_scaled_time::<_, _, E>(
                easing_function,
                0.0,
                1.0,
                time - self.params.min_time,
                self.params.max_time,
            )
        } else {
            1.0
        };

        let min_pos = scale_rect(self.params.min_pos, size, scale_factor);
        let max_pos = scale_rect(self.params.max_pos, size, scale_factor);

        let obj_pos = min_pos.lerp_towards(&max_pos, eased_time);

        let opacity = match self.params.state {
            ObjState::Entering => eased_time,
            ObjState::OnScreen => 1.0,
            ObjState::Exiting => 1.0 - eased_time,
        };

        if opacity <= 0.0 {
            return;
        }

        match &self.inner {
            ResolvedObjInner::Text { job, .. } => {
                for buffer in job {
                    let buffer_rect =
                        (buffer.buffer_rect * scale_factor).translate(obj_pos.min.to_vec2());
                    let mut buffer = BufferWithTextArea::new(
                        Arc::clone(&buffer.buffer),
                        buffer_rect,
                        opacity,
                        Color32::WHITE,
                        ui.ctx(),
                    );
                    buffer.scale *= scale_factor;
                    // ui.painter().rect_stroke(
                    //     buffer.rect / ui.ctx().pixels_per_point(),
                    //     CornerRadius::default(),
                    //     Stroke::new(2.0, Color32::CYAN.gamma_multiply(opacity)),
                    //     egui::StrokeKind::Inside,
                    // );
                    // ui.painter().rect_stroke(
                    //     buffer_rect,
                    //     CornerRadius::default(),
                    //     Stroke::new(2.0, Color32::RED.gamma_multiply(opacity)),
                    //     egui::StrokeKind::Inside,
                    // );
                    buffers.push(buffer);
                }
                // ui.painter().rect_stroke(
                //     obj_pos,
                //     CornerRadius::default(),
                //     Stroke::new(2.0, Color32::GREEN.gamma_multiply(opacity)),
                //     egui::StrokeKind::Outside,
                // );
            }
            ResolvedObjInner::Image { image, tint } => {
                let img = image
                    .clone()
                    // .fit_to_exact_size(obj_pos.size())
                    .tint(tint.gamma_multiply(opacity));
                img.paint_at(ui, obj_pos);
            }
            ResolvedObjInner::Rect { color, mut stroke } => {
                stroke.color = stroke.color.gamma_multiply(opacity);
                stroke.width *= scale_factor;
                ui.painter().rect(
                    obj_pos,
                    CornerRadius::default(),
                    color.gamma_multiply(opacity),
                    stroke,
                    egui::StrokeKind::Middle,
                );
            }
            ResolvedObjInner::Highlight {
                rects,
                color,
                state,
            } => {
                let total_width: f32 = rects.iter().map(|r| r.width() * scale_factor).sum();
                let width_to_draw = match state {
                    ObjState::Entering => total_width * eased_time,
                    ObjState::OnScreen => total_width,
                    ObjState::Exiting => total_width * (1.0 - eased_time),
                };
                let mut width_drawn = 0.0;
                for rect in rects.iter() {
                    let mut rect = *rect * scale_factor;
                    let rect_width = rect.width();
                    if width_to_draw > (width_drawn + rect_width) {
                        width_drawn += rect_width;
                        ui.painter().rect_filled(
                            rect.translate(obj_pos.min.to_vec2()),
                            CornerRadius::default(),
                            color.gamma_multiply(opacity),
                        );
                    } else {
                        rect.set_width(width_to_draw - width_drawn);
                        ui.painter().rect_filled(
                            rect.translate(obj_pos.min.to_vec2()),
                            CornerRadius::default(),
                            color.gamma_multiply(opacity),
                        );
                        break;
                    }
                }
            }
            ResolvedObjInner::Line {
                objects,
                origin_positions,
                mut stroke,
            } => {
                let first_time = if objects[0].max_time > 0.0 {
                    keyframe::ease_with_scaled_time::<f32, f64, E>(
                        easing_function,
                        0.0f32,
                        1.0f32,
                        time - objects[0].min_time,
                        objects[0].max_time,
                    )
                } else {
                    1.0
                };
                let first_obj_pos = scale_rect(objects[0].min_pos, size, scale_factor)
                    .lerp_towards(
                        &scale_rect(objects[0].max_pos, size, scale_factor),
                        first_time,
                    );
                let second_time = if objects[1].max_time > 0.0 {
                    keyframe::ease_with_scaled_time::<f32, f64, E>(
                        easing_function,
                        0.0f32,
                        1.0f32,
                        time - objects[1].min_time,
                        objects[1].max_time,
                    )
                } else {
                    1.0
                };
                let second_obj_pos = scale_rect(objects[1].min_pos, size, scale_factor)
                    .lerp_towards(
                        &scale_rect(objects[1].max_pos, size, scale_factor),
                        second_time,
                    );

                stroke.color = stroke.color.gamma_multiply(opacity);
                stroke.width *= scale_factor;
                let first_pos = origin_positions[0].pos_in_rect(&first_obj_pos);
                let second_pos = origin_positions[1].pos_in_rect(&second_obj_pos);

                ui.painter().line_segment(
                    [
                        first_pos,
                        match objects[1].state {
                            ObjState::Entering => first_pos.lerp(second_pos, second_time),
                            ObjState::OnScreen => second_pos,
                            ObjState::Exiting => second_pos.lerp(first_pos, second_time),
                        },
                    ],
                    stroke,
                );
            }
        }
    }
}

#[inline(always)]
fn scale_rect(viewbox: Rect, size: Rect, factor: f32) -> Rect {
    (viewbox * factor).translate(size.min.to_vec2())
}
