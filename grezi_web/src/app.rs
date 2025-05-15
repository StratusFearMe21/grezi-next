use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use eframe::{
    egui::{self, Modifiers, Spinner},
    egui_wgpu,
};
use egui_glyphon::GlyphonRendererCallback;
use grezi_egui::{get_size_and_factor, GrzResolvedSlide};
use keyframe::functions::EaseOutCubic;

use crate::AppHandle;

pub struct App {
    pub time: f64,
    pub slide_index: usize,
    pub resolved_slide: Option<GrzResolvedSlide>,
    pub shared_data: AppHandle,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let max_rect = ui.max_rect();

            ctx.input(|input| {
                for event in input.events.iter().map(|e| Cow::Borrowed(e)) {
                    match event.as_ref() {
                        egui::Event::PointerButton {
                            pressed: true,
                            modifiers: Modifiers::NONE,
                            ..
                        }
                        | egui::Event::Key {
                            key: egui::Key::ArrowRight | egui::Key::Space,
                            pressed: true,
                            ..
                        } => {
                            self.slide_index += 1;
                            self.time = input.time;
                            self.resolved_slide = None;
                        }
                        egui::Event::Key {
                            key: egui::Key::ArrowLeft,
                            pressed: true,
                            ..
                        } => {
                            self.slide_index = self.slide_index.saturating_sub(1);
                            self.resolved_slide = None;
                        }
                        egui::Event::Key {
                            key: egui::Key::B,
                            pressed: true,
                            ..
                        } => {
                            self.slide_index = 0;
                            self.resolved_slide = None;
                        }
                        egui::Event::Key {
                            key: egui::Key::N,
                            pressed: true,
                            ..
                        } => {
                            self.resolved_slide = None;
                        }
                        egui::Event::Key {
                            key: egui::Key::R,
                            modifiers: Modifiers::NONE,
                            pressed: true,
                            ..
                        } => {
                            self.time = input.time;
                        }
                        _ => {}
                    }
                }
            });

            let mut buffers = Vec::new();

            if let Some(ref root) = *self.shared_data.slideshow.load() {
                if self.resolved_slide.is_none() {
                    loop {
                        self.resolved_slide = GrzResolvedSlide::resolve_slide(
                            root.deref(),
                            self.shared_data.font_system.lock().deref_mut(),
                            ctx,
                            self.slide_index,
                        );

                        if self.resolved_slide.is_none() {
                            self.slide_index = self.slide_index.saturating_sub(1);
                            if self.slide_index == 0 {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                let time = ui.input(|i| i.time - self.time);
                if let Some(ref resolved) = self.resolved_slide {
                    resolved.draw(max_rect, ui, time, &EaseOutCubic, &mut buffers, None);
                    if resolved.max_time > time {
                        ctx.request_repaint();
                    } else if let Some(next) = resolved.params.next {
                        if resolved.max_time + next > time {
                            ctx.request_repaint_after_secs(
                                ((resolved.max_time + next) - time) as f32,
                            );
                        } else {
                            self.slide_index += 1;
                            self.time = ui.input(|i| i.time);
                            self.resolved_slide = None;
                        }
                    }
                }
            } else {
                let (size, factor) = get_size_and_factor(max_rect);
                Spinner::new().paint_at(&ui, size.shrink(25.0 * factor));
            }

            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                max_rect,
                GlyphonRendererCallback { buffers },
            ));
        });
    }
}
