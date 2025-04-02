use std::{borrow::Cow, ops::Deref, sync::mpsc::Receiver};

use eframe::{
    egui::{self, Modifiers, Pos2, Rect, Spinner},
    egui_wgpu,
};
use egui_glyphon::GlyphonRendererCallback;
use grezi_egui::get_size_and_factor;
use grezi_file_owner::AppHandle;
use keyframe::functions::EaseOutCubic;

use crate::FileOwnerMessage;

pub struct App {
    pub custom_key_events: Receiver<egui::Event>,
    pub time: f64,
    pub clip: bool,
    pub shared_data: AppHandle,
    /// This helps the user get a rectangle on the screen
    /// in the scale of the program
    pub first_pointer_pos: Option<Pos2>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let max_rect = ui.max_rect();

            ctx.input(|input| {
                for event in input
                    .events
                    .iter()
                    .map(|e| Cow::Borrowed(e))
                    .chain(self.custom_key_events.try_iter().map(|e| Cow::Owned(e)))
                {
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
                            self.shared_data
                                .root_owner_sender
                                .send(FileOwnerMessage::Next(false))
                                .unwrap();
                        }
                        egui::Event::PointerButton {
                            pressed: true, pos, ..
                        } => {
                            if let Some(first_pos) = self.first_pointer_pos {
                                let (size, factor) = get_size_and_factor(max_rect);
                                let rect = Rect::from_min_max(first_pos, *pos)
                                    .translate(-size.min.to_vec2())
                                    / factor;

                                eprintln!("{:?}", rect);
                                self.first_pointer_pos = None;
                            } else {
                                self.first_pointer_pos = Some(*pos);
                            }
                        }
                        egui::Event::Key {
                            key: egui::Key::ArrowLeft,
                            pressed: true,
                            ..
                        } => {
                            self.shared_data
                                .root_owner_sender
                                .send(FileOwnerMessage::Previous)
                                .unwrap();
                        }
                        egui::Event::Key {
                            key: egui::Key::B,
                            pressed: true,
                            ..
                        } => {
                            self.shared_data
                                .root_owner_sender
                                .send(FileOwnerMessage::Index {
                                    index: 0,
                                    reset_time: false,
                                })
                                .unwrap();
                        }
                        egui::Event::Key {
                            key: egui::Key::C,
                            pressed,
                            ..
                        } => {
                            self.clip = *pressed;
                        }
                        egui::Event::Key {
                            key: egui::Key::N,
                            pressed: true,
                            ..
                        } => {
                            self.shared_data.resolved.store(None);
                        }
                        egui::Event::Key {
                            key: egui::Key::R,
                            modifiers: Modifiers::NONE,
                            pressed: true,
                            ..
                        } => {
                            self.time = input.time;
                        }
                        egui::Event::Key {
                            key: egui::Key::R,
                            pressed: true,
                            ..
                        } => {
                            self.shared_data
                                .root_owner_sender
                                .send(FileOwnerMessage::ResetFile)
                                .unwrap();
                        }
                        _ => {}
                    }
                }
            });

            let mut buffers = Vec::new();

            if let Some(resolved) = self.shared_data.resolved.load().deref() {
                if self.clip {
                    ui.set_clip_rect(get_size_and_factor(max_rect).0);
                }
                let time = ui.input(|i| i.time - self.time);
                resolved.draw(max_rect, ui, time, &EaseOutCubic, &mut buffers, None);
                if resolved.max_time > time {
                    ctx.request_repaint();
                } else if let Some(next) = resolved.params.next {
                    if resolved.max_time + next > time {
                        ctx.request_repaint_after_secs(((resolved.max_time + next) - time) as f32);
                    } else {
                        self.shared_data
                            .root_owner_sender
                            .send(FileOwnerMessage::Next(true))
                            .unwrap();
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
