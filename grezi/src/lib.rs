#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
    io::Cursor,
    mem::ManuallyDrop,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use arc_swap::{ArcSwap, ArcSwapOption};
use atomic_float::AtomicF32;
#[cfg(not(target_arch = "wasm32"))]
use crossbeam_queue::SegQueue;
use eframe::{
    egui::{
        self, FontData, FontDefinitions, Image, Rect, SizeHint, Ui, ViewportBuilder, ViewportId,
    },
    epaint::{
        mutex::{Mutex, RwLock},
        text::LayoutJob,
        Color32, FontFamily, FontId, Pos2, Rounding, Stroke, TextShape, Vec2,
    },
};
use egui_anim::Anim;
#[cfg(not(target_arch = "wasm32"))]
use fontdb::{Family, Query};
// use frame_history::FrameHistory;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::ropey::Rope;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::tree_sitter::Tree;
use image::codecs::{png::PngDecoder, webp::WebPDecoder};
use keyframe::functions::{EaseOutCubic, EaseOutQuint, Linear};
use layout::{Constraint, Direction, UnresolvedLayout};
#[cfg(not(target_arch = "wasm32"))]
use notify::{event::ModifyKind, Watcher};
use parser::{
    actions::{Actions, ResolvedActions, HIGHLIGHT_COLOR_DEFAULT},
    color::Color,
    objects::{Object, ObjectState, ObjectType},
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
mod label;
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
    pub time: f32,
    #[cfg(not(target_arch = "wasm32"))]
    pub lsp: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub parser: Arc<Mutex<helix_core::tree_sitter::Parser>>,
    pub export: bool,
    pub clear_color: Color32,
    pub fonts: FontDefinitions,
    // pub frame_history: FrameHistory,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct SpeakerView {
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
            });
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
            });

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
                                let res = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    layout[2],
                                    &slide_show,
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
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            layout[2],
                                            &slide_show,
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
                        self.current_resolved
                            .store(Some(Arc::clone(&next_resolved)));
                    }
                    if let Some(slide) = slide_show.slide_show.get(c_index) {
                        ctx.request_repaint();
                        match slide {
                            AstObject::Slide {
                                objects: slide,
                                actions,
                                ..
                            } => {
                                let res = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    layout[0],
                                    &slide_show,
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
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            layout[0],
                                            &slide_show,
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
            next_resolved.draw_actions(ui, f32::MAX);
            next_resolved.draw_slide(ui, f32::MAX);
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
            current_resolved.draw_slide(ui, f32::MAX);
            current_resolved.draw_actions(ui, f32::MAX);
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
    pub viewboxes: HashMap<u64, Vec<Rect>, BuildHasherDefault<PassThroughHasher>>,
    pub actions: Vec<ResolvedActions>,
    pub slide: Vec<ResolvedSlideObj>,
    pub images: HashMap<u64, ResolvedObject, BuildHasherDefault<PassThroughHasher>>,
    pub speaker_notes: Option<Arc<str>>,
    pub window_size: Rect,
}

impl Default for Resolved {
    fn default() -> Self {
        Self {
            viewboxes: Default::default(),
            actions: Default::default(),
            slide: Default::default(),
            images: Default::default(),
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
    fn draw_slide(&self, ui: &mut Ui, time: f32) {
        for obj in &self.slide {
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
                ResolvedObject::Text(galley) => {
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
                        ui.add(label::Label::new(
                            Arc::clone(galley),
                            obj_pos,
                            gamma_multiply,
                        ));
                    }
                }
                ResolvedObject::Image {
                    image,
                    mut tint,
                    source,
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

                        if let Some(galley) = source.clone() {
                            ui.painter().add(
                                TextShape::new(
                                    Pos2::new(
                                        obj_pos.max.x - galley.rect.max.x,
                                        obj_pos.min.y - galley.rect.max.y,
                                    ),
                                    galley,
                                    Color32::TRANSPARENT,
                                )
                                .with_opacity_factor(gamma),
                            );
                        }
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
                    ui.painter().rect_filled(
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
    ) -> Rect {
        match self.viewboxes.get(&hash) {
            None => {
                let split = match slide_show.viewboxes.get(&hash).unwrap().split_on {
                    ViewboxIn::Size => size,
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
                let rect = layout[index];
                self.viewboxes.insert(hash, layout);
                rect
            }
            Some(viewboxes) => viewboxes.get(index).copied().unwrap(),
        }
    }

    fn resolve(
        slide: &[SlideObj],
        actions: &[Actions],
        ui: &mut Ui,
        size: Rect,
        slide_show: &SlideShow,
        export: bool,
    ) -> Self {
        let mut resolved = Resolved::default();
        resolved.window_size = size;
        let mut images = Vec::with_capacity(3);
        let mut source_offset = 0.0;
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    resolved.resolve_layout(hash, index, size, slide_show)
                }
                ViewboxIn::Inherit(_) => unreachable!(),
            };

            let obj = slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                parser::objects::ObjectType::Spinner => {
                    let size = ResolvedObject::Spinner.bounds(second_viewbox.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox, size).into(),
                        size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, size).into(),
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
                    let mut rect = second_viewbox
                        .translate(Vec2::new(-second_viewbox.min.x, -second_viewbox.min.y));
                    rect.max.y = *height * (size.height() / 1080.0);
                    let resolved_obj = ResolvedObject::Rect {
                        color: *color,
                        rect,
                    };

                    let size = resolved_obj.bounds(second_viewbox.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox, size).into(),
                        size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, size).into(),
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
                parser::objects::ObjectType::Text { layout_job, source } => {
                    let mut layout_job = layout_job.clone();
                    layout_job.wrap.max_width = second_viewbox.width();
                    for row in layout_job.sections.iter_mut() {
                        row.format.font_id.size *=
                            (size.width() + size.height()) / (1920.0 + 1080.0);
                        row.format.strikethrough.width *=
                            (size.width() + size.height()) / (1920.0 + 1080.0);
                        row.format.underline.width *=
                            (size.width() + size.height()) / (1920.0 + 1080.0);
                        if let Some(lh) = row.format.line_height.as_mut() {
                            *lh *= row.format.font_id.size
                        }
                    }
                    let galley = ui.ctx().fonts(|f| f.layout_job(layout_job));
                    let resolved_obj = ResolvedObject::Text(galley);
                    let size = resolved_obj.bounds(second_viewbox.size(), ui);
                    let mut first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox, size).into(),
                        size.size(),
                    );
                    let mut second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, size).into(),
                        size.size(),
                    );
                    if *source {
                        second_pos = second_pos.translate(Vec2::new(
                            if object.state == ObjectState::Exiting {
                                size.width()
                                    - match &resolved_obj {
                                        ResolvedObject::Text(galley) => galley
                                            .pos_from_pcursor(
                                                eframe::epaint::text::cursor::PCursor {
                                                    paragraph: 0,
                                                    offset: 2,
                                                    prefer_next_row: false,
                                                },
                                            )
                                            .width(),
                                        _ => unreachable!(),
                                    }
                            } else {
                                0.0
                            },
                            -source_offset,
                        ));

                        first_pos = first_pos.translate(Vec2::new(
                            if object.state == ObjectState::Entering {
                                size.width()
                                    - match &resolved_obj {
                                        ResolvedObject::Text(galley) => galley
                                            .pos_from_pcursor(
                                                eframe::epaint::text::cursor::PCursor {
                                                    paragraph: 0,
                                                    offset: 2,
                                                    prefer_next_row: false,
                                                },
                                            )
                                            .width(),
                                        _ => unreachable!(),
                                    }
                            } else {
                                0.0
                            },
                            -source_offset,
                        ));
                        if object.state != ObjectState::Exiting {
                            source_offset += size.max.y + 5.0;
                        }
                    }
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
                    source,
                    scale,
                } => {
                    match images.binary_search(&object.object) {
                        Err(index) | Ok(index) => images.insert(index, object.object),
                    }
                    let resolved_obj = resolved
                        .images
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
                                source: None,
                            }
                        });

                    match resolved_obj {
                        ResolvedObject::Image { source: s, .. } => {
                            *s = source.clone().map(|mut layout_job| {
                                layout_job.wrap.max_width = second_viewbox.width();
                                for row in layout_job.sections.iter_mut() {
                                    row.format.font_id.size *=
                                        (size.width() + size.height()) / (1920.0 + 1080.0);
                                    row.format.strikethrough.width *=
                                        (size.width() + size.height()) / (1920.0 + 1080.0);
                                    row.format.underline.width *=
                                        (size.width() + size.height()) / (1920.0 + 1080.0);
                                    if let Some(lh) = row.format.line_height.as_mut() {
                                        *lh *= row.format.font_id.size
                                    }
                                }
                                ui.ctx().fonts(|f| f.layout_job(layout_job))
                            });
                        }
                        ResolvedObject::Anim { .. } => {}
                        _ => unreachable!(),
                    }

                    let first_size =
                        resolved_obj.bounds(scale.unwrap_or_else(|| first_viewbox.size()), ui);
                    let second_size =
                        resolved_obj.bounds(scale.unwrap_or_else(|| second_viewbox.size()), ui);
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
                        get_pos!(object.locations[0].0, first_viewbox, first_size).into(),
                        first_size.size(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, second_size).into(),
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
        resolved
            .images
            .retain(|k, _| images.binary_search(k).is_ok());
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
                            ResolvedObject::Text(galley) => (
                                galley.pos_from_pcursor(locations[0]),
                                galley.pos_from_pcursor(locations[1]),
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
    pub fn used_fonts(&self, defs: &FontDefinitions) -> HashSet<String, ahash::RandomState> {
        let mut hashset = HashSet::default();
        for obj in self.objects.values() {
            if let ObjectType::Text { layout_job, .. } = &obj.object {
                for section in &layout_job.sections {
                    hashset.insert(
                        defs.families.get(&section.format.font_id.family).unwrap()[0].clone(),
                    );
                }
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
        let loading_hash = hasher.hash_one("loading");
        let halves_hash = hasher.hash_one("halves");
        SlideShow {
            slide_show: vec![AstObject::Slide {
                objects: vec![
                    SlideObj {
                        object: spinner_hash,
                        locations: [
                            (LineUp::CenterTop, ViewboxIn::Custom(halves_hash, 0)),
                            (LineUp::CenterCenter, ViewboxIn::Custom(halves_hash, 0)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: ObjectState::Entering,
                    },
                    SlideObj {
                        object: loading_hash,
                        locations: [
                            (LineUp::CenterBottom, ViewboxIn::Custom(halves_hash, 1)),
                            (LineUp::CenterCenter, ViewboxIn::Custom(halves_hash, 1)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: ObjectState::Entering,
                    },
                ],
                actions: vec![Actions::Highlight {
                    locations: None,
                    index: 1,
                    persist: true,
                    color: HIGHLIGHT_COLOR_DEFAULT,
                }],
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
                        source_obj: None,
                    },
                );
                map.insert(
                    loading_hash,
                    Object {
                        position: None,
                        viewbox: None,
                        object: parser::objects::ObjectType::Text {
                            layout_job: {
                                let mut job = LayoutJob::default();
                                job.append(
                                    "Loading",
                                    0.0,
                                    egui::TextFormat {
                                        font_id: FontId::proportional(48.0),
                                        color: Color32::WHITE,
                                        background: Color32::TRANSPARENT,
                                        ..Default::default()
                                    },
                                );
                                job
                            },
                            source: false,
                        },
                        source_obj: None,
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

fn resolve_layout_raw(
    size: Rect,
    direction: Direction,
    mut constraints: Vec<Constraint>,
    split: Rect,
    margin: f32,
) -> Vec<Rect> {
    constraints.iter_mut().for_each(|c| match c {
        layout::Constraint::Length(length) => {
            *length *= (size.width() + size.height()) / (1920.0 + 1080.0)
        }
        layout::Constraint::Min(min) => {
            *min *= (size.width() + size.height()) / (1920.0 + 1080.0);
        }
        layout::Constraint::Max(max) => {
            *max *= (size.width() + size.height()) / (1920.0 + 1080.0);
        }
        _ => {}
    });
    layout::Layout::default()
        .direction(direction)
        .margin(margin)
        .constraints(&constraints)
        .split(split)
        .unwrap()
}

impl MyEguiApp {
    pub fn init_app(
        mut self,
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
                let fetch_ctx = egui_ctx.clone();
                #[cfg(not(target_arch = "wasm32"))]
                ehttp::fetch(ehttp::Request::get(&self.file_name), move |response| {
                    let res = response.unwrap();
                    let slide_show: (FontDefinitions, SlideShow) =
                        bincode::deserialize(&res.bytes).unwrap();
                    fetch_ctx.set_fonts(slide_show.0);
                    *fetch_ss.write() = slide_show.1;
                    fetch_restart_timer.store(true, Ordering::Relaxed);
                    fetch_resolved.store(None);
                });

                #[cfg(target_arch = "wasm32")]
                ehttp::fetch(ehttp::Request::get(hash), move |response| {
                    let res = response.unwrap();
                    let slide_show: (FontDefinitions, SlideShow) =
                        bincode::deserialize(&res.bytes).unwrap();
                    fetch_ctx.set_fonts(slide_show.0);
                    *fetch_ss.write() = slide_show.1;
                    fetch_restart_timer.store(true, Ordering::Relaxed);
                    fetch_resolved.store(None);
                });
            }
            SlideShowSource::Loaded => {
                #[cfg(not(target_arch = "wasm32"))]
                if !self.lsp && !self.export {
                    use std::time::Instant;

                    let watcher_tree_info = Arc::clone(&self.tree_info);
                    let watcher_context = egui_ctx.clone();
                    let watcher_file_name = Arc::clone(&self.file_name);
                    let watcher_slide_show_file = Arc::clone(&self.slide_show_file);
                    let watcher_resolved = Arc::clone(&self.resolved);
                    let watcher_restart_timer = Arc::clone(&self.restart_timer);
                    let watcher_parser = Arc::clone(&self.parser);
                    let watcher_slide_show = Arc::clone(&self.slide_show);
                    let mut fonts = self.fonts.clone();
                    let mut instant = Instant::now();
                    let mut font_db = fontdb::Database::new();
                    font_db.load_system_fonts();
                    let mut w = ManuallyDrop::new(
                        notify::recommended_watcher(
                            move |res: Result<notify::Event, notify::Error>| {
                                if let Ok(event) = res {
                                    if let notify::EventKind::Modify(ModifyKind::Data(_)) =
                                        event.kind
                                    {
                                        if Instant::now().duration_since(instant)
                                            > Duration::from_millis(250)
                                        {
                                            std::thread::sleep(Duration::from_millis(250));
                                            instant = Instant::now();
                                            let new_file = Rope::from_reader(
                                                std::fs::File::open(watcher_file_name.as_ref())
                                                    .unwrap(),
                                            )
                                            .unwrap();
                                            let mut slide_show_file =
                                                watcher_slide_show_file.lock();
                                            let mut tree_info = watcher_tree_info.lock();
                                            if let Some(info) = tree_info.as_mut() {
                                                let transaction = helix_core::diff::compare_ropes(
                                                    &slide_show_file,
                                                    &new_file,
                                                );
                                                let edits = lsp::generate_edits(
                                                    slide_show_file.slice(..),
                                                    transaction.changes(),
                                                );
                                                for change in edits.iter().rev() {
                                                    info.edit(change);
                                                }

                                                let tree = watcher_parser
                                                    .lock()
                                                    .parse_with(
                                                        &mut |byte, _| {
                                                            if byte <= new_file.len_bytes() {
                                                                let (chunk, start_byte, _, _) =
                                                                    new_file.chunk_at_byte(byte);
                                                                &chunk.as_bytes()
                                                                    [byte - start_byte..]
                                                            } else {
                                                                // out of range
                                                                &[]
                                                            }
                                                        },
                                                        Some(info),
                                                    )
                                                    .unwrap();

                                                let mut slide_show = watcher_slide_show.write();

                                                let ast = parser::parse_file(
                                                    &tree,
                                                    Some(info),
                                                    &new_file,
                                                    &mut self.helix_cell,
                                                    &mut slide_show,
                                                    &mut font_db,
                                                    &mut Default::default(),
                                                    &mut fonts,
                                                    &watcher_context,
                                                    std::path::Path::new(
                                                        watcher_file_name.as_ref(),
                                                    ),
                                                );
                                                *info = tree;
                                                match ast {
                                                    Ok(_) => {
                                                        *slide_show_file = new_file.clone();
                                                        watcher_resolved.store(None);
                                                        watcher_restart_timer
                                                            .store(true, Ordering::Relaxed);
                                                    }
                                                    Err(errors) => {
                                                        for error in errors {
                                                            eprintln!(
                                                                "{:?}",
                                                                parser::ErrWithSource {
                                                                    error,
                                                                    source_code: new_file
                                                                        .to_string()
                                                                }
                                                            );
                                                        }
                                                    }
                                                }
                                            }

                                            watcher_context.request_repaint();
                                        }
                                    }
                                }
                            },
                        )
                        .unwrap(),
                    );

                    w.watch(
                        std::path::Path::new(self.file_name.as_ref()),
                        notify::RecursiveMode::NonRecursive,
                    )
                    .unwrap();

                    egui_ctx.set_fonts(self.fonts.clone());
                }
            }
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
    ) -> (Self, SlideShowSource) {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glo::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut fonts = FontDefinitions::default();

        #[cfg(not(target_arch = "wasm32"))]
        let slide_show_file = Arc::new(Mutex::new(Rope::new()));
        let new_file = Arc::new(AtomicBool::new(true));
        #[cfg(not(target_arch = "wasm32"))]
        let tree_info: Arc<Mutex<Option<Tree>>> = Arc::new(Mutex::new(None));

        #[cfg(not(target_arch = "wasm32"))]
        let mut helix_cell = None;
        #[cfg(not(target_arch = "wasm32"))]
        let mut sources = indexmap::IndexSet::default();

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
                    let slideshow: (FontDefinitions, SlideShow) =
                        bincode::deserialize(&file).unwrap();

                    fonts = slideshow.0;

                    (slideshow.1, SlideShowSource::Loaded)
                }
            } else {
                let mut font_db = fontdb::Database::new();
                font_db.load_system_fonts();
                let mut fira_code_prop = Query::default();
                fira_code_prop.families = &[Family::Name("Fira Code")];
                if let Some(font) = font_db.query(&fira_code_prop) {
                    // Leaking the font makes it cheaper to clone the font definitions elsewhere
                    let (src, index) = unsafe { font_db.make_shared_face_data(font).unwrap() };
                    let data: &'static [u8] = unsafe { &*Arc::into_raw(src) }.as_ref();
                    fonts.font_data.insert("Fira Code".to_owned(), {
                        let mut font = FontData::from_static(data);
                        font.index = index;
                        font
                    });

                    fonts
                        .families
                        .get_mut(&FontFamily::Monospace)
                        .unwrap()
                        .insert(0, "Fira Code".to_owned());
                }
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
                            &mut font_db,
                            &mut sources,
                            &mut fonts,
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
                }),
                #[cfg(not(target_arch = "wasm32"))]
                lsp,
                #[cfg(not(target_arch = "wasm32"))]
                parser: Arc::new(Mutex::new(parser)),
                clear_color: Color::default().into(),
                fonts,
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
        if ctx.input(|input| {
            let r = input.key_down(egui::Key::Q) || input.key_down(egui::Key::Escape);
            if r {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            r || input.viewport().close_requested()
        }) {
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
        egui::TopBottomPanel::bottom("controls")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.add_enabled(self.index.load(Ordering::Relaxed) != 0, egui::Button::new("<")).clicked() {
                        self.index.fetch_sub(1, Ordering::Relaxed);
                        self.next.store(false, Ordering::Relaxed);
                        self.resolved.store(None);
                        self.time = 1000.0;
                    } else if ui.add_enabled(self.index.load(Ordering::Relaxed) != slide_show.slide_show.len() - 1, egui::Button::new(">")).clicked() {
                        self.index.fetch_add(1, Ordering::Relaxed);
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
                                let resolved = Arc::new(Resolved::resolve(
                                    slide,
                                    actions,
                                    ui,
                                    window_size,
                                    &slide_show,
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
                                        let resolved = Arc::new(Resolved::resolve(
                                            slide,
                                            actions,
                                            ui,
                                            window_size,
                                            &slide_show,
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
                            resolved.draw_slide(ui, self.time);
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
                                    resolved.draw_slide(ui, *max_time);
                                }
                                _ => todo!(),
                            }
                            resolved.draw_actions(ui, self.time);

                            if self.time < 0.5 {
                                ctx.request_repaint();
                            }
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let vb_dbg = self.vb_dbg.load(Ordering::Relaxed);
                        if vb_dbg > 0 {
                            if let Some(vb) = resolved.viewboxes.get(&vb_dbg) {
                                for rect in vb {
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
        if !self.lsp && !self.export {
            let index = Arc::clone(&self.index);
            let slide_show = Arc::clone(&self.slide_show);
            let speaker_notes = self
                .resolved
                .load()
                .as_ref()
                .and_then(|r| r.speaker_notes.clone());
            let speaker_view = Arc::clone(&self.speaker_view);
            ctx.show_viewport_deferred(
                speaker_viewport,
                ViewportBuilder::default(),
                move |ctx, _| {
                    speaker_view.ui(
                        ctx,
                        index.load(Ordering::Relaxed),
                        &*slide_show.read(),
                        speaker_notes.clone(),
                    )
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
