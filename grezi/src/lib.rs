#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    io::Cursor,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
};

use arc_swap::{ArcSwap, ArcSwapOption};
use atomic_float::AtomicF32;
#[cfg(not(target_arch = "wasm32"))]
use crossbeam_queue::SegQueue;
use eframe::{
    egui::{self, Id, Image, ImageSize, Rect, Sense, SizeHint, Ui, ViewportBuilder, ViewportId},
    egui_wgpu,
    emath::Align2,
    epaint::{
        mutex::{Mutex, RwLock},
        Color32, PaintCallback, Pos2, Rounding, Stroke, Vec2,
    },
};
use egui_anim::Anim;
use egui_glyphon::{
    glyphon::{fontdb::ID, Buffer, Edit, FontSystem, Metrics},
    BufferWithTextArea, GlyphonRendererCallback,
};
// use frame_history::FrameHistory;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::ropey::Rope;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::tree_sitter::Tree;
use image::codecs::{png::PngDecoder, webp::WebPDecoder};
#[cfg(not(target_arch = "wasm32"))]
use indexmap::IndexSet;
use keyframe::functions::{EaseOutCubic, EaseOutQuint, Linear};
use layout::{Constraint, Direction, UnresolvedLayout};
use parser::{
    actions::{Actions, ResolvedActions},
    color::Color,
    objects::{Editor, Object, ObjectState, ObjectType},
    slides::{ResolvedSlideObj, SlideObj},
    viewboxes::{LineUp, ViewboxIn},
    AstObject, PassThroughHasher,
};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::parser::highlighting::HelixCell;
use crate::parser::objects::ResolvedObject;

#[cfg(not(target_arch = "wasm32"))]
pub mod cairo;
// mod frame_history;
mod layout;
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
pub mod parser;

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyEguiApp {
    pub slide_show: Arc<RwLock<SlideShow>>,
    pub next: Arc<AtomicBool>,
    pub restart_timer: Arc<AtomicBool>,
    #[cfg(not(target_arch = "wasm32"))]
    pub slide_show_file: Arc<Mutex<Rope>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub tree_info: Arc<Mutex<Option<Tree>>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub file_name: Arc<str>,
    #[cfg(not(target_arch = "wasm32"))]
    pub vb_dbg: Arc<AtomicU64>,
    #[cfg(not(target_arch = "wasm32"))]
    pub obj_dbg: Arc<AtomicU64>,
    pub index: Arc<AtomicUsize>,
    #[cfg(not(target_arch = "wasm32"))]
    pub helix_cell: Option<HelixCell>,
    #[cfg(not(target_arch = "wasm32"))]
    pub speaker_view: Arc<SpeakerView>,
    // Safe, I think, IDK
    pub resolved: Arc<ArcSwapOption<Resolved>>,
    pub resolved_images:
        Arc<Mutex<HashMap<u64, ResolvedObject, BuildHasherDefault<PassThroughHasher>>>>,
    pub time: f32,
    #[cfg(not(target_arch = "wasm32"))]
    pub lsp: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub parser: Arc<Mutex<helix_core::tree_sitter::Parser>>,
    pub export: bool,
    pub clear_color: Color32,
    pub font_system: Arc<Mutex<FontSystem>>,
    // pub frame_history: FrameHistory,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct SpeakerView {
    pub visible: AtomicBool,
    pub next_resolved: ArcSwapOption<Resolved>,
    pub current_resolved: ArcSwapOption<Resolved>,
    pub events: SegQueue<egui::Event>,
    pub max_rect: ArcSwap<Rect>,
    pub line: [AtomicF32; 3],
}

#[cfg(not(target_arch = "wasm32"))]
impl SpeakerView {
    fn clear_resolved(&self) {
        self.current_resolved.store(None);
        self.next_resolved.store(None);
    }

    fn ui(
        &self,
        ctx: &egui::Context,
        c_index: usize,
        slide_show: &SlideShow,
        speaker_notes: Option<Arc<str>>,
        font_system: Arc<Mutex<FontSystem>>,
        resolved_images: Arc<
            Mutex<HashMap<u64, ResolvedObject, BuildHasherDefault<PassThroughHasher>>>,
        >,
    ) {
        egui::TopBottomPanel::top("Speaker view")
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_gray(10))
                    .outer_margin(5.0),
            )
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Speaker view");
                });
            })
            .response
            .rect;
        egui::TopBottomPanel::bottom("Speaker Notes")
            .resizable(true)
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_gray(10))
                    .outer_margin(5.0),
            )
            .show(ctx, |ui| {
                if let Some(notes) = speaker_notes {
                    ui.horizontal_centered(|ui| ui.label(notes.deref()));
                }
            })
            .response
            .rect;

        let index = c_index + 1;
        egui::CentralPanel::default().show(ctx, |ui| {
            let ws = ui.max_rect();
            let (current_resolved, next_resolved) =
                if let (Some(current_resolved), Some(next_resolved)) = (
                    self.current_resolved.load_full().and_then(|r| {
                        if self.max_rect.load().deref().deref().ne(&ws) {
                            self.max_rect.store(Arc::new(ws));
                            None
                        } else {
                            Some(r)
                        }
                    }),
                    self.next_resolved.load_full(),
                ) {
                    (current_resolved, next_resolved)
                } else {
                    let layout = layout::Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(0.0)
                        .constraints(&[
                            Constraint::Ratio(1.0, 2.0),
                            Constraint::Length(5.0),
                            Constraint::Ratio(1.0, 2.0),
                        ])
                        .split(ws)
                        .unwrap();
                    let line_ct = layout[1].center_top();
                    let line_cb = layout[1].center_bottom();
                    self.line[0].store(line_ct.x, Ordering::Relaxed);
                    self.line[1].store(line_ct.y, Ordering::Relaxed);
                    self.line[2].store(line_cb.y, Ordering::Relaxed);
                    let current_resolved;
                    let next_resolved;
                    if let Some(slide) = slide_show.slide_show.get(index) {
                        ctx.request_repaint();
                        match slide {
                            AstObject::Slide {
                                objects: slide,
                                actions,
                                ..
                            } => {
                                let mut font_system = font_system.lock();
                                let res = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    layout[2],
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&resolved_images),
                                    false,
                                ));
                                self.next_resolved.store(Some(Arc::clone(&res)));
                                next_resolved = res;
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                            } => {
                                let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide { objects: slide, .. } => {
                                        let mut font_system = font_system.lock();
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            layout[2],
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&resolved_images),
                                            false,
                                        ));
                                        self.next_resolved.store(Some(Arc::clone(&res)));
                                        next_resolved = res;
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    } else {
                        next_resolved = Arc::new(Resolved::slideshow_end(layout[2]));
                        self.next_resolved.store(Some(Arc::clone(&next_resolved)));
                    }
                    if let Some(slide) = slide_show.slide_show.get(c_index) {
                        ctx.request_repaint();
                        match slide {
                            AstObject::Slide {
                                objects: slide,
                                actions,
                                ..
                            } => {
                                let mut font_system = font_system.lock();
                                let res = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    layout[0],
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&resolved_images),
                                    false,
                                ));
                                self.current_resolved.store(Some(Arc::clone(&res)));
                                current_resolved = res;
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                            } => {
                                let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide { objects: slide, .. } => {
                                        let mut font_system = font_system.lock();
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            layout[0],
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&resolved_images),
                                            false,
                                        ));
                                        self.current_resolved.store(Some(Arc::clone(&res)));
                                        current_resolved = res;
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    } else {
                        current_resolved = Arc::new(Resolved::slideshow_end(layout[0]));
                        self.current_resolved
                            .store(Some(Arc::clone(&current_resolved)));
                    }
                    (current_resolved, next_resolved)
                };
            if let Some(slide) = slide_show.slide_show.get(index) {
                match slide {
                    AstObject::Slide { bg: (bg, b), .. } => {
                        let color: Color32 = if let Some(b) = b {
                            bg.interpolate(b.1, f32::MAX, b.0.as_secs_f32()).into()
                        } else {
                            (*bg).into()
                        };
                        ui.painter().rect(
                            next_resolved.window_size,
                            Rounding::default(),
                            color,
                            Stroke::NONE,
                        );
                    }
                    AstObject::Action { slide_in_ast, .. } => {
                        let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                        match slide {
                            AstObject::Slide { bg: (bg, b), .. } => {
                                let color: Color32 = if let Some(b) = b {
                                    bg.interpolate(b.1, f32::MAX, b.0.as_secs_f32()).into()
                                } else {
                                    (*bg).into()
                                };
                                ui.painter().rect(
                                    next_resolved.window_size,
                                    Rounding::default(),
                                    color,
                                    Stroke::NONE,
                                );
                            }
                            _ => todo!(),
                        }
                    }
                }
            } else {
                ui.painter().rect(
                    next_resolved.window_size,
                    Rounding::default(),
                    Color32::BLACK,
                    Stroke::NONE,
                );
            }
            let mut buffers = Vec::new();
            {
                let mut font_system = font_system.lock();
                next_resolved.draw_slide(ui, f32::MAX, &mut buffers, font_system.deref_mut());
                next_resolved.draw_actions(ui, f32::MAX);
            }
            if let Some(slide) = slide_show.slide_show.get(c_index) {
                match slide {
                    AstObject::Slide { bg: (bg, b), .. } => {
                        let color: Color32 = if let Some(b) = b {
                            bg.interpolate(b.1, f32::MAX, b.0.as_secs_f32()).into()
                        } else {
                            (*bg).into()
                        };
                        ui.painter().rect(
                            current_resolved.window_size,
                            Rounding::default(),
                            color,
                            Stroke::NONE,
                        );
                    }
                    AstObject::Action { slide_in_ast, .. } => {
                        let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                        match slide {
                            AstObject::Slide { bg: (bg, b), .. } => {
                                let color: Color32 = if let Some(b) = b {
                                    bg.interpolate(b.1, f32::MAX, b.0.as_secs_f32()).into()
                                } else {
                                    (*bg).into()
                                };
                                ui.painter().rect(
                                    current_resolved.window_size,
                                    Rounding::default(),
                                    color,
                                    Stroke::NONE,
                                );
                            }
                            _ => todo!(),
                        }
                    }
                }
            } else {
                ui.painter().rect(
                    current_resolved.window_size,
                    Rounding::default(),
                    Color32::BLACK,
                    Stroke::NONE,
                );
            }
            {
                let mut font_system = font_system.lock();
                current_resolved.draw_slide(ui, f32::MAX, &mut buffers, font_system.deref_mut());
                current_resolved.draw_actions(ui, f32::MAX);
            }
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                ws,
                GlyphonRendererCallback { buffers },
            ));
            ui.painter().vline(
                self.line[0].load(Ordering::Relaxed),
                self.line[1].load(Ordering::Relaxed)..=self.line[2].load(Ordering::Relaxed),
                ui.style().visuals.widgets.noninteractive.bg_stroke,
            );
            egui::Window::new("Current Slide")
                .default_open(false)
                .fixed_pos(current_resolved.window_size.min)
                .show(ctx, |_| {});
            egui::Window::new("Next Slide")
                .default_open(false)
                .fixed_pos(next_resolved.window_size.min)
                .show(ctx, |_| {});
        });
        ctx.input(|i| {
            i.events
                .iter()
                .filter(|e| matches!(e, egui::Event::Key { .. }))
                .cloned()
                .for_each(|e| self.events.push(e))
        });
        if !self.events.is_empty() {
            ctx.request_repaint_of(ViewportId::ROOT);
        }
    }
}

pub struct Resolved {
    pub viewboxes: HashMap<u64, Layouts, BuildHasherDefault<PassThroughHasher>>,
    pub actions: Vec<ResolvedActions>,
    pub slide: Vec<ResolvedSlideObj>,
    pub speaker_notes: Option<Arc<str>>,
    pub window_size: Rect,
}

impl Default for Resolved {
    fn default() -> Self {
        Self {
            viewboxes: Default::default(),
            actions: Default::default(),
            slide: Default::default(),
            speaker_notes: None,
            window_size: Rect::ZERO,
        }
    }
}

impl Resolved {
    fn slideshow_end(size: Rect) -> Self {
        Self {
            window_size: size,
            ..Default::default()
        }
    }
    fn draw_slide(
        &self,
        ui: &mut Ui,
        time: f32,
        buffers: &mut Vec<BufferWithTextArea<Editor>>,
        font_system: &mut FontSystem,
    ) {
        ui.set_clip_rect(ui.max_rect());
        for (idx, obj) in self.slide.iter().enumerate() {
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
                ResolvedObject::Text(buffer) => {
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
                        // ui.painter()
                        //     .rect(obj_pos, Rounding::default(), Color32::RED, Stroke::NONE);
                        let response = ui.allocate_rect(obj_pos, Sense::click_and_drag());

                        if let Some(click) = response.interact_pointer_pos() {
                            if response.dragged() {
                                let drag = (click - obj_pos.min.to_vec2()) + response.drag_delta();
                                buffer.write().0.action(
                                    font_system,
                                    egui_glyphon::glyphon::Action::Drag {
                                        x: drag.x as i32,
                                        y: drag.y as i32,
                                    },
                                );
                            } else {
                                let click = click - obj_pos.min.to_vec2();
                                buffer.write().0.action(
                                    font_system,
                                    egui_glyphon::glyphon::Action::Click {
                                        x: click.x as i32,
                                        y: click.y as i32,
                                    },
                                );
                            }
                        }

                        let boxes = buffer.read().highlight_boxes();
                        for r#box in &boxes {
                            ui.painter().rect_filled(
                                r#box.translate(obj_pos.min.to_vec2()),
                                Rounding::default(),
                                Color32::BLUE,
                            );
                        }

                        let mut window_clicked = false;
                        {
                            let ref buffer_read = buffer.read().0;
                            if let Some(r#box) = boxes.last() {
                                if ui.input(|i| i.any_touches()) {
                                    egui::Window::new("copy_cosmic_text")
                                        .id(Id::new((idx, "copy_cosmic_text")))
                                        .title_bar(false)
                                        .fixed_pos(r#box.translate(obj_pos.min.to_vec2()).max)
                                        .resizable(false)
                                        .collapsible(false)
                                        .show(ui.ctx(), |ui| {
                                            if ui.button("Copy").clicked() {
                                                window_clicked = true;
                                                ui.ctx().output_mut(|t| {
                                                    t.copied_text = buffer_read
                                                        .copy_selection()
                                                        .unwrap_or_default()
                                                });
                                            }
                                        });
                                }
                            }

                            if buffer_read.select_opt().is_some() {
                                if ui.input_mut(|i| i.events.contains(&egui::Event::Copy)) {
                                    ui.ctx().output_mut(|t| {
                                        t.copied_text =
                                            buffer_read.copy_selection().unwrap_or_default()
                                    });
                                }
                            }
                        }

                        if response.clicked_elsewhere() && !window_clicked {
                            buffer.write().0.set_select_opt(None);
                        }

                        buffers.push(BufferWithTextArea::new(
                            Arc::clone(buffer),
                            obj_pos,
                            gamma_multiply,
                            egui_glyphon::glyphon::Color::rgb(255, 255, 255),
                            ui.ctx(),
                        ));
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
    }

    fn draw_actions(&self, ui: &mut Ui, time: f32) {
        for action in &self.actions {
            match action {
                parser::actions::ResolvedActions::Highlight {
                    locations,
                    persist,
                    locations_of_object,
                    scaled_time,
                    color,
                } => {
                    let time = if !*persist {
                        scaled_time[1]
                    } else if scaled_time[0] < time {
                        time - scaled_time[0]
                    } else {
                        0.0
                    };
                    let obj_pos = Vec2::from(keyframe::ease_with_scaled_time(
                        EaseOutCubic,
                        locations_of_object[0],
                        locations_of_object[1],
                        time,
                        scaled_time[1],
                    ));
                    ui.ctx().debug_painter().rect_filled(
                        Rect {
                            min: Pos2::new(
                                if *persist {
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

    fn resolve(
        slide: &[SlideObj],
        actions: &[Actions],
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
        resolved.window_size = size_raw;
        let size = {
            let size = size_raw.size();
            let size = ImageSize {
                max_size: size,
                ..Default::default()
            }
            .calc_size(size, Vec2::new(16.0, 9.0));
            Align2::CENTER_CENTER.align_size_within_rect(size, size_raw)
        };
        let mut images = Vec::with_capacity(3);
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => resolve_layout_raw(
                    size,
                    Direction::Horizontal,
                    vec![Constraint::Min(0.0)],
                    Splits {
                        unadjusted: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
                        adjusted: size,
                    },
                    15.0,
                )
                .get_splits(0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => resolve_layout_raw(
                    size,
                    Direction::Horizontal,
                    vec![Constraint::Min(0.0)],
                    Splits {
                        unadjusted: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
                        adjusted: size,
                    },
                    15.0,
                )
                .get_splits(0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };

            let obj = slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                parser::objects::ObjectType::Spinner => {
                    let size = ResolvedObject::Spinner.bounds(second_viewbox.adjusted.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: ResolvedObject::Spinner,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                parser::objects::ObjectType::Rect { color, height } => {
                    let mut rect = second_viewbox.adjusted.translate(Vec2::new(
                        -second_viewbox.adjusted.min.x,
                        -second_viewbox.adjusted.min.y,
                    ));
                    rect.max.y = *height * (size.height() / 1080.0);
                    let resolved_obj = ResolvedObject::Rect {
                        color: *color,
                        rect,
                    };

                    let size = resolved_obj.bounds(second_viewbox.adjusted.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: resolved_obj,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                parser::objects::ObjectType::Text(job, font_size, line_height) => {
                    let factor = second_viewbox.adjusted.size() / second_viewbox.unadjusted.size();
                    let font_size = *font_size * factor.x;
                    let mut buffer = Buffer::new(
                        font_system,
                        Metrics::new(
                            font_size,
                            line_height.map_or(font_size * 1.1, |h| h * font_size),
                        ),
                    );
                    buffer.set_size(
                        font_system,
                        second_viewbox.adjusted.width(),
                        // second_viewbox.adjusted.height(),
                        f32::MAX,
                    );
                    parser::objects::add_job_to_buffer(job, &mut buffer, font_system);
                    buffer.shape_until_scroll(font_system);

                    let resolved_obj = ResolvedObject::Text(Arc::new(RwLock::new(Editor(
                        egui_glyphon::glyphon::Editor::new(buffer),
                    ))));
                    let size = resolved_obj.bounds(second_viewbox.adjusted.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox.adjusted, size).into(),
                        size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved.slide.push(ResolvedSlideObj {
                        object: resolved_obj,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                parser::objects::ObjectType::Image {
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
                                        if decoder.is_apng() {
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
                    // let size = Rect {
                    //     min: size.min,
                    //     max: {
                    //         if second_viewbox.max.x / second_viewbox.max.y > size.max.x / size.max.y
                    //         {
                    //             Pos2::new(
                    //                 second_viewbox.max.y * size.max.x / size.max.y,
                    //                 second_viewbox.max.y,
                    //             )
                    //         } else {
                    //             Pos2::new(
                    //                 second_viewbox.max.x,
                    //                 second_viewbox.max.x * size.max.y / size.max.x,
                    //             )
                    //         }
                    //     },
                    // };
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox.adjusted, first_size).into(),
                        first_size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox.adjusted, second_size)
                            .into(),
                        second_size.size(),
                    );
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
        for action in actions {
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
                                    let ref buffer = buffer.read().0;
                                    let glyph = buffer.buffer().lines.get(locations[0][0]).unwrap();
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
                                                + buffer.buffer().metrics().line_height
                                                    * locations[0][0] as f32,
                                        ),
                                        Vec2::new(0.0, glyph.y_offset),
                                    )
                                },
                                {
                                    let ref buffer = buffer.read().0;
                                    let glyph = buffer.buffer().lines.get(locations[1][0]).unwrap();
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
                                                + buffer.buffer().metrics().line_height
                                                    * locations[1][0] as f32,
                                        ),
                                        Vec2::new(
                                            glyph.w,
                                            glyph.y_offset + buffer.buffer().metrics().line_height,
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
            }
        }
        resolved
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SlideShow {
    pub slide_show: Vec<AstObject>,
    pub viewboxes: HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    pub objects: HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
}

impl SlideShow {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn used_fonts(&self, font_system: &mut FontSystem) -> IndexSet<ID, ahash::RandomState> {
        let mut hashset = IndexSet::default();
        for obj in self.objects.values() {
            if let ObjectType::Text(job, _, _) = &obj.object {
                let mut buffer = Buffer::new(font_system, Metrics::new(12.0, 24.0));
                buffer.set_size(font_system, f32::MAX, f32::MAX);
                parser::objects::add_job_to_buffer(job, &mut buffer, font_system);
                buffer.shape_until_scroll(font_system);

                buffer.layout_runs().for_each(|r| {
                    r.glyphs.iter().for_each(|g| {
                        hashset.insert(g.font_id);
                    })
                });
            }
        }

        hashset
    }
}

impl SlideShow {
    // Creates a slide for exercising the Browser JIT on WASM to avoid jank
    fn exercise_jit() -> SlideShow {
        let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
        let spinner_hash = hasher.hash_one("spinner");
        let halves_hash = hasher.hash_one("halves");
        SlideShow {
            slide_show: vec![AstObject::Slide {
                objects: vec![SlideObj {
                    object: spinner_hash,
                    locations: [
                        (LineUp::CenterTop, ViewboxIn::Custom(halves_hash, 0)),
                        (LineUp::CenterCenter, ViewboxIn::Custom(halves_hash, 0)),
                    ],
                    scaled_time: [0.0, 0.5],
                    state: ObjectState::Entering,
                }],
                actions: vec![],
                bg: (Color::default(), None),
                max_time: 0.5,
                next: false,
            }],
            viewboxes: {
                let mut map = HashMap::default();
                map.insert(
                    halves_hash,
                    UnresolvedLayout {
                        direction: layout::Direction::Vertical,
                        margin: 15.0,
                        constraints: vec![Constraint::Ratio(1.0, 2.0), Constraint::Ratio(1.0, 2.0)],
                        expand_to_fill: true,
                        split_on: ViewboxIn::Size,
                    },
                );
                map
            },
            objects: {
                let mut map = HashMap::default();
                map.insert(
                    spinner_hash,
                    Object {
                        position: None,
                        viewbox: None,
                        object: parser::objects::ObjectType::Spinner,
                    },
                );
                map
            },
        }
    }
}

pub enum SlideShowSource {
    Loaded,
    Http,
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

impl MyEguiApp {
    pub fn init_app(
        self,
        egui_ctx: &egui::Context,
        slide_show_source: SlideShowSource,
        #[cfg(target_arch = "wasm32")] hash: &str,
    ) -> Self {
        egui_extras::install_image_loaders(egui_ctx);
        if !egui_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
            egui_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
        }

        match slide_show_source {
            SlideShowSource::Http => {
                let fetch_ss = Arc::clone(&self.slide_show);
                let fetch_resolved = Arc::clone(&self.resolved);
                let fetch_restart_timer = Arc::clone(&self.restart_timer);
                let fetch_font_system = Arc::clone(&self.font_system);
                #[cfg(not(target_arch = "wasm32"))]
                ehttp::fetch(ehttp::Request::get(&self.file_name), move |response| {
                    let res = response.unwrap();
                    let slide_show: (Vec<Vec<u8>>, SlideShow) =
                        bincode::deserialize(&res.bytes).unwrap();
                    let mut fonts = fetch_font_system.lock();
                    let fonts = fonts.db_mut();

                    slide_show
                        .0
                        .into_iter()
                        .for_each(|d| fonts.load_font_data(d));

                    *fetch_ss.write() = slide_show.1;
                    fetch_restart_timer.store(true, Ordering::Relaxed);
                    fetch_resolved.store(None);
                });

                #[cfg(target_arch = "wasm32")]
                {
                    use egui::FontData;
                    use egui::FontDefinitions;
                    use egui::FontTweak;
                    use egui_glyphon::glyphon::fontdb::Query;
                    use egui_glyphon::glyphon::Family;

                    let fetch_ctx = egui_ctx.clone();
                    ehttp::fetch(ehttp::Request::get(hash), move |response| {
                        let res = response.unwrap();
                        let slide_show: (Vec<Vec<u8>>, SlideShow) =
                            bincode::deserialize(&res.bytes).unwrap();
                        let mut fonts = fetch_font_system.lock();
                        {
                            let fonts = fonts.db_mut();

                            slide_show
                                .0
                                .into_iter()
                                .for_each(|d| fonts.load_font_data(d));
                        }

                        let mut font_defs = FontDefinitions::default();

                        if let Some(sans_serif) = fonts.db().query(&Query {
                            families: &[Family::SansSerif],
                            ..Default::default()
                        }) {
                            let face = unsafe {
                                fonts.db_mut().make_shared_face_data(sans_serif).unwrap()
                            };

                            let index = face.1;
                            let face = Arc::into_raw(face.0);

                            font_defs.families.insert(
                                egui::FontFamily::Proportional,
                                vec!["sans-serif".to_string()],
                            );

                            font_defs.font_data.insert(
                                "sans-serif".to_string(),
                                FontData {
                                    font: std::borrow::Cow::Borrowed(unsafe { (&*face).as_ref() }),
                                    index,
                                    tweak: FontTweak::default(),
                                },
                            );
                        }

                        if let Some(monospace) = fonts.db().query(&Query {
                            families: &[Family::Monospace],
                            ..Default::default()
                        }) {
                            let face =
                                unsafe { fonts.db_mut().make_shared_face_data(monospace).unwrap() };

                            let index = face.1;
                            let face = Arc::into_raw(face.0);

                            font_defs
                                .families
                                .insert(egui::FontFamily::Monospace, vec!["monospace".to_string()]);

                            font_defs.font_data.insert(
                                "monospace".to_string(),
                                FontData {
                                    font: std::borrow::Cow::Borrowed(unsafe { (&*face).as_ref() }),
                                    index,
                                    tweak: FontTweak::default(),
                                },
                            );
                        }

                        fetch_ctx.set_fonts(font_defs);
                        *fetch_ss.write() = slide_show.1;
                        fetch_restart_timer.store(true, Ordering::Relaxed);
                        fetch_resolved.store(None);
                    });
                }
            }
            SlideShowSource::Loaded => {}
        }

        MyEguiApp {
            #[cfg(not(target_arch = "wasm32"))]
            helix_cell: None,
            ..self
        }
    }
    pub fn new(
        #[cfg(not(target_arch = "wasm32"))] lsp: bool,
        #[cfg(not(target_arch = "wasm32"))] presentation: Option<String>,
        font_system: Arc<Mutex<FontSystem>>,
    ) -> (Self, SlideShowSource) {
        {
            let mut font_system = font_system.lock();
            font_system.db_mut().set_sans_serif_family("Ubuntu");
            font_system.db_mut().set_monospace_family("Fira Code");
        }
        #[cfg(not(target_arch = "wasm32"))]
        let slide_show_file = Arc::new(Mutex::new(Rope::new()));
        let new_file = Arc::new(AtomicBool::new(true));
        #[cfg(not(target_arch = "wasm32"))]
        let tree_info: Arc<Mutex<Option<Tree>>> = Arc::new(Mutex::new(None));

        #[cfg(not(target_arch = "wasm32"))]
        let mut helix_cell = None;

        #[cfg(not(target_arch = "wasm32"))]
        let mut parser = {
            let mut parser = helix_core::tree_sitter::Parser::new();
            parser.set_language(tree_sitter_grz::language()).unwrap();
            parser
        };
        #[cfg(not(target_arch = "wasm32"))]
        let slide_show: (SlideShow, SlideShowSource) = {
            let viewboxes = HashMap::default();
            let objects = HashMap::default();
            if presentation
                .as_deref()
                .unwrap_or_default()
                .ends_with("slideshow")
            {
                if presentation
                    .as_deref()
                    .unwrap_or_default()
                    .starts_with("http")
                {
                    (SlideShow::exercise_jit(), SlideShowSource::Http)
                } else {
                    let file = std::fs::read(presentation.as_ref().unwrap()).unwrap();
                    let slideshow: (Vec<Vec<u8>>, SlideShow) = bincode::deserialize(&file).unwrap();

                    let mut fonts = font_system.lock();
                    let fonts = fonts.db_mut();

                    slideshow
                        .0
                        .into_iter()
                        .for_each(|d| fonts.load_font_data(d));

                    (slideshow.1, SlideShowSource::Loaded)
                }
            } else {
                if lsp {
                    (SlideShow::default(), SlideShowSource::Loaded)
                } else {
                    let mut slide_show = SlideShow {
                        slide_show: Vec::new(),
                        viewboxes,
                        objects,
                    };
                    let mut tree_info = tree_info.lock();
                    let file = Rope::from_reader(
                        std::fs::File::open(presentation.as_ref().unwrap()).unwrap(),
                    )
                    .unwrap();

                    let tree = parser
                        .parse_with(
                            &mut |byte, _| {
                                if byte <= file.len_bytes() {
                                    let (chunk, start_byte, _, _) = file.chunk_at_byte(byte);
                                    &chunk.as_bytes()[byte - start_byte..]
                                } else {
                                    // out of range
                                    &[]
                                }
                            },
                            None,
                        )
                        .unwrap();
                    let ast = {
                        let ctx = eframe::egui::Context::default();
                        egui_extras::install_image_loaders(&ctx);
                        if !ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                            ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
                        }
                        parser::parse_file(
                            &tree,
                            None,
                            &file,
                            &mut helix_cell,
                            &mut slide_show,
                            &ctx,
                            &std::fs::canonicalize(presentation.as_ref().unwrap()).unwrap(),
                        )
                    };
                    match ast {
                        Ok(_) => {
                            *tree_info = Some(tree);
                            *slide_show_file.lock() = file;
                            (slide_show, SlideShowSource::Loaded)
                        }
                        Err(errors) => {
                            for error in errors {
                                eprintln!(
                                    "{:?}",
                                    parser::ErrWithSource {
                                        error,
                                        source_code: file.to_string()
                                    }
                                );
                            }
                            std::process::exit(1);
                        }
                    }
                }
            }
        };

        #[cfg(target_arch = "wasm32")]
        let slide_show = (SlideShow::exercise_jit(), SlideShowSource::Http);

        new_file.store(false, Ordering::Relaxed);

        (
            Self {
                slide_show: Arc::new(RwLock::new(slide_show.0)),
                next: Arc::new(false.into()),
                export: false,
                restart_timer: Arc::new(false.into()),
                #[cfg(not(target_arch = "wasm32"))]
                slide_show_file,
                #[cfg(not(target_arch = "wasm32"))]
                tree_info,
                #[cfg(not(target_arch = "wasm32"))]
                file_name: presentation.unwrap_or_default().into(),
                #[cfg(not(target_arch = "wasm32"))]
                vb_dbg: Arc::new(0.into()),
                #[cfg(not(target_arch = "wasm32"))]
                obj_dbg: Arc::new(0.into()),
                index: Arc::new(0.into()),
                time: 0.0,
                #[cfg(not(target_arch = "wasm32"))]
                helix_cell,
                resolved: Arc::new(ArcSwapOption::new(None)),
                #[cfg(not(target_arch = "wasm32"))]
                speaker_view: Arc::new(SpeakerView {
                    next_resolved: ArcSwapOption::new(None),
                    current_resolved: ArcSwapOption::new(None),
                    events: SegQueue::new(),
                    max_rect: ArcSwap::new(Arc::new(Rect::ZERO)),
                    line: [
                        AtomicF32::new(0.0),
                        AtomicF32::new(0.0),
                        AtomicF32::new(0.0),
                    ],
                    visible: false.into(),
                }),
                #[cfg(not(target_arch = "wasm32"))]
                lsp,
                #[cfg(not(target_arch = "wasm32"))]
                parser: Arc::new(Mutex::new(parser)),
                clear_color: Color::default().into(),
                font_system,
                resolved_images: Arc::new(Mutex::new(HashMap::default())),
                // frame_history: FrameHistory::default(),
            },
            slide_show.1,
        )
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        // if let Soce(&mut ref mut frame) = frame {
        //     self.frame_history
        //         .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        // }
        self.time += ctx.input(|i| i.stable_dt);
        let speaker_viewport = ViewportId::from_hash_of("speaker_view");
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|input| input.key_down(egui::Key::Q) || input.key_down(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        let slide_show_cloned = Arc::clone(&self.slide_show);
        let slide_show = slide_show_cloned.read();
        let mut index = self.index.load(Ordering::Relaxed);
        if index >= slide_show.slide_show.len() {
            index = slide_show.slide_show.len() - 1;
            self.index
                .store(slide_show.slide_show.len() - 1, Ordering::Relaxed);
            self.next.store(false, Ordering::Relaxed);
        }

        #[cfg(target_arch = "wasm32")]
        {
            egui::TopBottomPanel::bottom("controls")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.add_enabled(self.index.load(Ordering::Relaxed) != 0, egui::Button::new("<")).clicked() {
                        self.index.fetch_sub(1, Ordering::Relaxed);
                        index -= 1;
                        self.next.store(false, Ordering::Relaxed);
                        self.resolved.store(None);
                        self.time = 1000.0;
                    } else if ui.add_enabled(self.index.load(Ordering::Relaxed) != slide_show.slide_show.len() - 1, egui::Button::new(">")).clicked() {
                        self.index.fetch_add(1, Ordering::Relaxed);
                        index += 1;
                        self.resolved.store(None);
                        self.next.store(true, Ordering::Relaxed);
                        self.time = 0.0;
                    }
                    // ui.label(format!(
                    //     "FPS: {:.1}",
                    //     self.frame_history.fps()
                    // ));
                    ui.label("This presentation was made using Grezi, created by Isaac Mills, the guy who made this portfolio!");
                    ui.hyperlink_to("Check out the source code!", "https://github.com/StratusFearMe21/grezi-next");
                })
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.lsp {
            egui::TopBottomPanel::bottom("controls")
                .exact_height(32.0)
                .show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("{}", index + 1));
                    })
                });
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.clear_color))
            .show(ctx, |ui| {
                let window_size = ui.max_rect();
                if self.restart_timer.load(Ordering::Relaxed) {
                    self.time = 0.0;
                    self.restart_timer.store(false, Ordering::Relaxed);
                }

                let resolved = if let Some(resolved) = self.resolved.load_full().and_then(|r| {
                    if r.window_size != window_size {
                        None
                    } else {
                        Some(r)
                    }
                }) {
                    resolved
                } else {
                    if let Some(slide) = slide_show.slide_show.get(index) {
                        ctx.request_repaint_of(speaker_viewport);
                        match slide {
                            AstObject::Slide {
                                objects: slide,
                                actions,
                                bg,
                                ..
                            } => {
                                self.clear_color = bg.0.into();
                                let mut font_system = self.font_system.lock();
                                let resolved = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    window_size,
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&self.resolved_images),
                                    self.export,
                                ));
                                self.resolved.store(Some(Arc::clone(&resolved)));
                                resolved
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                            } => {
                                let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide {
                                        objects: slide, bg, ..
                                    } => {
                                        self.clear_color = bg.0.into();
                                        let mut font_system = self.font_system.lock();
                                        let resolved = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            window_size,
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&self.resolved_images),
                                            self.export,
                                        ));
                                        self.resolved.store(Some(Arc::clone(&resolved)));
                                        resolved
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    } else {
                        Arc::new(Resolved::slideshow_end(window_size))
                    }
                };
                let mut buffers = Vec::new();
                if let Some(slide) = slide_show.slide_show.get(index) {
                    match slide {
                        AstObject::Slide {
                            max_time,
                            next,
                            bg: (bg, b),
                            ..
                        } => {
                            if let Some(b) = b {
                                let color: Color32 =
                                    bg.interpolate(b.1, self.time, b.0.as_secs_f32()).into();
                                if self.clear_color != color {
                                    ctx.request_repaint();
                                }
                                self.clear_color = color;
                            }
                            let mut font_system = self.font_system.lock();
                            resolved.draw_slide(
                                ui,
                                self.time,
                                &mut buffers,
                                font_system.deref_mut(),
                            );
                            resolved.draw_actions(ui, self.time);

                            if self.time < *max_time {
                                ctx.request_repaint();
                            } else if *next && self.next.load(Ordering::Relaxed) {
                                self.index.fetch_add(1, Ordering::Relaxed);
                                self.resolved.store(None);
                                self.time = 0.0;
                                #[cfg(not(target_arch = "wasm32"))]
                                self.speaker_view.clear_resolved();
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
                                ctx.request_repaint();
                            }
                        }
                        AstObject::Action { slide_in_ast, .. } => {
                            let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                            match slide {
                                AstObject::Slide { max_time, .. } => {
                                    let mut font_system = self.font_system.lock();
                                    resolved.draw_slide(
                                        ui,
                                        *max_time,
                                        &mut buffers,
                                        font_system.deref_mut(),
                                    );
                                }
                                _ => todo!(),
                            }
                            resolved.draw_actions(ui, self.time);

                            if self.time < 0.5 {
                                ctx.request_repaint();
                            }
                        }
                    }
                    if !self.export {
                        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                            window_size,
                            GlyphonRendererCallback { buffers },
                        ));
                    } else {
                        ui.painter().add(PaintCallback {
                            rect: window_size,
                            callback: Arc::new(GlyphonRendererCallback { buffers }),
                        });
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let vb_dbg = self.vb_dbg.load(Ordering::Relaxed);
                        if vb_dbg > 0 {
                            if let Some(vb) = resolved.viewboxes.get(&vb_dbg) {
                                for rect in vb.adjusted.iter() {
                                    ctx.debug_painter().rect_stroke(
                                        *rect,
                                        Rounding::ZERO,
                                        Stroke::new(2.0, Color32::RED),
                                    );
                                }
                            }
                        }
                    }
                }
            });

        ctx.input(|input| {
            for event in {
                let iter = input
                    .events
                    .iter()
                    .filter(|e| matches!(e, egui::Event::Key { .. }))
                    .cloned();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    iter.chain(std::iter::from_fn(|| self.speaker_view.events.pop()))
                }
                #[cfg(target_arch = "wasm32")]
                {
                    iter
                }
            } {
                match event {
                    egui::Event::Key {
                        key: egui::Key::ArrowRight | egui::Key::Space,
                        pressed: true,
                        ..
                    } => {
                        let _ = self.index.fetch_update(
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                            |index| {
                                if index != slide_show.slide_show.len() - 1 {
                                    self.resolved.store(None);
                                    self.time = 0.0;
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.speaker_view.clear_resolved();
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.vb_dbg.store(0, Ordering::Relaxed);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.obj_dbg.store(0, Ordering::Relaxed);
                                    self.next.store(true, Ordering::Relaxed);
                                    Some(index + 1)
                                } else {
                                    None
                                }
                            },
                        );
                    }
                    egui::Event::Key {
                        key: egui::Key::ArrowLeft,
                        pressed: true,
                        ..
                    } => {
                        let _ = self.index.fetch_update(
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                            |index| {
                                if index != 0 {
                                    self.resolved.store(None);
                                    self.time = 1000.0;
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.speaker_view.clear_resolved();
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.vb_dbg.store(0, Ordering::Relaxed);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.obj_dbg.store(0, Ordering::Relaxed);
                                    self.next.store(false, Ordering::Relaxed);
                                    return Some(index - 1);
                                }
                                None
                            },
                        );
                    }
                    egui::Event::Key {
                        key: egui::Key::R,
                        pressed: true,
                        ..
                    } => {
                        self.time = 0.0;
                        #[cfg(not(target_arch = "wasm32"))]
                        self.vb_dbg.store(0, Ordering::Relaxed);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.obj_dbg.store(0, Ordering::Relaxed);
                        self.next.store(true, Ordering::Relaxed);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    egui::Event::Key {
                        key: egui::Key::S,
                        pressed: true,
                        ..
                    } => {
                        self.speaker_view.visible.store(true, Ordering::Relaxed);
                    }
                    egui::Event::Key {
                        key: egui::Key::B,
                        pressed: true,
                        ..
                    } => {
                        self.index.store(0, Ordering::Relaxed);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.vb_dbg.store(0, Ordering::Relaxed);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.obj_dbg.store(0, Ordering::Relaxed);
                        self.next.store(false, Ordering::Relaxed);
                        self.resolved.store(None);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.speaker_view.clear_resolved();
                    }
                    _ => {}
                }
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        if self.speaker_view.visible.load(Ordering::Relaxed) {
            let index = Arc::clone(&self.index);
            let slide_show = Arc::clone(&self.slide_show);
            let speaker_notes = self
                .resolved
                .load()
                .as_ref()
                .and_then(|r| r.speaker_notes.clone());
            let speaker_view = Arc::clone(&self.speaker_view);
            let font_system = Arc::clone(&self.font_system);
            let resolved_images = Arc::clone(&self.resolved_images);
            ctx.show_viewport_deferred(
                speaker_viewport,
                ViewportBuilder::default(),
                move |ctx, _| {
                    speaker_view.ui(
                        ctx,
                        index.load(Ordering::Relaxed),
                        &*slide_show.read(),
                        speaker_notes.clone(),
                        Arc::clone(&font_system),
                        Arc::clone(&resolved_images),
                    );

                    if ctx.input(|i| i.viewport().close_requested()) {
                        speaker_view.visible.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);
    }
}
