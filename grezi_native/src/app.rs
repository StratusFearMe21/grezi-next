use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::{
        atomic::AtomicUsize,
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use arc_swap::ArcSwapOption;
use eframe::{
    egui::{self, mutex::Mutex, Modifiers, Pos2, Rect},
    egui_wgpu,
};
use egui_glyphon::{glyphon::FontSystem, GlyphonRendererCallback};
use grezi_egui::GrzResolvedSlide;
use keyframe::functions::EaseOutCubic;

#[derive(Clone)]
pub struct AppHandle {
    pub index: Arc<AtomicUsize>,
    pub resolved: Arc<ArcSwapOption<GrzResolvedSlide>>,
    pub font_system: Arc<Mutex<FontSystem>>,
    pub custom_key_sender: Sender<egui::Event>,
    pub egui_ctx: egui::Context,
}

impl AppHandle {
    pub fn new(
        custom_key_sender: Sender<egui::Event>,
        context: egui::Context,
        font_system: Arc<Mutex<FontSystem>>,
    ) -> Self {
        Self {
            index: Arc::new(0.into()),
            resolved: Arc::new(ArcSwapOption::empty()),
            font_system,
            custom_key_sender,
            egui_ctx: context,
        }
    }

    pub fn next_slide(&self) {
        self.index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.resolved.store(None);
    }

    pub fn previous_slide(&self) {
        self.index
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        self.resolved.store(None);
    }
}

pub struct App {
    pub custom_key_events: Receiver<egui::Event>,
    pub time: f64,
    pub max_rect: Rect,
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
                if self.shared_data.resolved.load().is_none() {
                    self.time = input.time;
                }
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
                            self.shared_data.next_slide();
                            self.time = input.time;
                        }
                        egui::Event::PointerButton {
                            pressed: true, pos, ..
                        } => {
                            if let Some(first_pos) = self.first_pointer_pos {
                                if let Some(resolved) = self.shared_data.resolved.load().deref() {
                                    let rect = Rect::from_min_max(first_pos, *pos)
                                        .translate(-resolved.size.min.to_vec2())
                                        / resolved.factor;

                                    eprintln!("{:?}", rect);
                                    self.first_pointer_pos = None;
                                }
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
                                .index
                                .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                            if self
                                .root
                                .slideshow
                                .slides
                                .get_index(
                                    self.shared_data
                                        .index
                                        .load(std::sync::atomic::Ordering::Relaxed),
                                )
                                .is_none()
                            {
                                self.shared_data.index.store(
                                    self.root.slideshow.slides.len() - 1,
                                    std::sync::atomic::Ordering::Relaxed,
                                );
                            }
                            // When navigating to the previous slide
                            // skip over all slides marked with the
                            // `next()` action
                            while self
                                .root
                                .slideshow
                                .slides
                                .get_index(
                                    self.shared_data
                                        .index
                                        .load(std::sync::atomic::Ordering::Relaxed),
                                )
                                .map(|(_, s)| s.slide_params.next)
                                .unwrap_or_default()
                            {
                                self.shared_data
                                    .index
                                    .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            self.shared_data.resolved.store(None);
                        }
                        egui::Event::Key {
                            key: egui::Key::B,
                            pressed: true,
                            ..
                        } => {
                            self.shared_data
                                .index
                                .store(0, std::sync::atomic::Ordering::Relaxed);
                            self.shared_data.resolved.store(None);
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
                        _ => {}
                    }
                }
            });

            let mut index = self
                .shared_data
                .index
                .load(std::sync::atomic::Ordering::Relaxed);
            if self.shared_data.resolved.load().is_none() || max_rect != self.max_rect {
                self.max_rect = max_rect;
                let mut new_slide;
                loop {
                    new_slide = GrzResolvedSlide::resolve_slide(
                        &self.root.slideshow,
                        self.shared_data.font_system.lock().deref_mut(),
                        ui.ctx(),
                        index,
                    );

                    if new_slide.is_none() {
                        index = index.saturating_sub(1);
                        if index == 0 {
                            self.shared_data
                                .index
                                .store(index, std::sync::atomic::Ordering::Relaxed);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.shared_data.resolved.store(new_slide.map(Arc::new));
            }

            let mut buffers = Vec::new();

            if let Some(resolved) = self.shared_data.resolved.load().deref() {
                // ui.set_clip_rect(resolved.size);
                let time = ui.input(|i| i.time - self.time);
                resolved.draw(ui, time, self.max_rect, &EaseOutCubic, &mut buffers);
                if resolved.max_time > time {
                    ctx.request_repaint();
                } else if resolved.params.next {
                    ctx.request_repaint();
                    self.shared_data
                        .custom_key_sender
                        .send(egui::Event::Key {
                            key: egui::Key::ArrowRight,
                            pressed: true,
                            physical_key: None,
                            repeat: false,
                            modifiers: Modifiers::NONE,
                        })
                        .unwrap();
                }
            }

            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                max_rect,
                GlyphonRendererCallback { buffers },
            ));
        });
    }
}
