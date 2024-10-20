#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, AtomicI8, AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
};

use arc_swap::{ArcSwap, ArcSwapOption};
use atomic_float::AtomicF32;
#[cfg(not(target_arch = "wasm32"))]
use crossbeam_queue::SegQueue;
use eframe::{
    egui::{self, Modifiers, Rect, Sense, ViewportBuilder, ViewportId},
    egui_wgpu,
    emath::Align2,
    epaint::{
        mutex::{Mutex, RwLock},
        Color32, PaintCallback, Rounding, Stroke,
    },
};
use egui_glyphon::{
    glyphon::{fontdb::ID, FontSystem, Metrics},
    GlyphonRendererCallback,
};
// use frame_history::FrameHistory;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::ropey::Rope;
use indexmap::IndexMap;
#[cfg(not(target_arch = "wasm32"))]
use indexmap::IndexSet;
use parser::{
    color::Color,
    objects::{Object, ObjectState, ObjectType},
    slides::SlideObj,
    viewboxes::ViewboxIn,
    AstObject, PassThroughHasher,
};
use resolver::{
    layout::{Constraint, Direction, Layout, UnresolvedLayout},
    Resolved,
};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::parser::highlighting::HelixCell;
use crate::parser::objects::ResolvedObject;

#[cfg(all(not(target_arch = "wasm32"), feature = "cairo"))]
pub mod cairo;
// mod frame_history;
#[cfg(not(target_arch = "wasm32"))]
pub mod args;
#[cfg(not(target_arch = "wasm32"))]
pub mod impress_server;
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
pub mod parser;
#[cfg(not(target_arch = "wasm32"))]
pub mod proremote_server;
pub mod resolver;

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyEguiApp {
    pub slide_show: Arc<ArcSwap<RwLock<SlideShow>>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub viewbox_nodes: Arc<ArcSwap<HashMap<u64, u64, BuildHasherDefault<PassThroughHasher>>>>,
    pub next: Arc<AtomicBool>,
    pub restart_timer: Arc<AtomicI8>,
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
    pub time_offset: f32,
    #[cfg(not(target_arch = "wasm32"))]
    pub lsp: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub parser: Arc<Mutex<helix_core::tree_sitter::Parser>>,
    pub export: bool,
    pub clear_color: Color32,
    pub font_system: Arc<Mutex<FontSystem>>,
    pub in_drag: bool, // pub frame_history: FrameHistory,
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
                    let layout = Layout::default()
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
                    if let Some((_, slide)) = slide_show.slide_show.get_index(index) {
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
                                    (actions, None),
                                    ui,
                                    layout[2],
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&resolved_images),
                                ));
                                self.next_resolved.store(Some(Arc::clone(&res)));
                                next_resolved = res;
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                                ..
                            } => {
                                let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide {
                                        objects: slide,
                                        actions: slide_actions,
                                        ..
                                    } => {
                                        let mut font_system = font_system.lock();
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            (slide_actions, Some(actions)),
                                            ui,
                                            layout[2],
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&resolved_images),
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
                    if let Some((_, slide)) = slide_show.slide_show.get_index(c_index) {
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
                                    (actions, None),
                                    ui,
                                    layout[0],
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&resolved_images),
                                ));
                                self.current_resolved.store(Some(Arc::clone(&res)));
                                current_resolved = res;
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                                ..
                            } => {
                                let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide {
                                        objects: slide,
                                        actions: slide_actions,
                                        ..
                                    } => {
                                        let mut font_system = font_system.lock();
                                        let res = Arc::new(Resolved::resolve(
                                            slide,
                                            (slide_actions, Some(actions)),
                                            ui,
                                            layout[0],
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&resolved_images),
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
            if let Some((_, slide)) = slide_show.slide_show.get_index(index) {
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
                        let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
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
                next_resolved.draw_slide(
                    ui,
                    f32::MAX,
                    &mut buffers,
                    font_system.lock().deref_mut(),
                    false,
                );
                next_resolved.draw_actions(ui, f32::MAX, false);
            }
            if let Some((_, slide)) = slide_show.slide_show.get_index(c_index) {
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
                        let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
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
                current_resolved.draw_slide(
                    ui,
                    f32::MAX,
                    &mut buffers,
                    font_system.lock().deref_mut(),
                    false,
                );
                current_resolved.draw_actions(ui, f32::MAX, false);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SlideShow {
    pub slide_show: IndexMap<u64, AstObject, BuildHasherDefault<PassThroughHasher>>,
    pub viewboxes: HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    pub objects: HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
}

// TODO: Handle all buffer[0].buffer cases

impl SlideShow {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn used_fonts(&self, font_system: &mut FontSystem) -> IndexSet<ID, ahash::RandomState> {
        let mut hashset = IndexSet::default();
        for obj in self.objects.values() {
            if let ObjectType::Text { job, .. } = &obj.object {
                for j in job {
                    let buffer =
                        j.make_buffer(font_system, f32::MAX, Metrics::new(18.0, 24.0), None);

                    buffer.layout_runs().for_each(|r| {
                        r.glyphs.iter().for_each(|g| {
                            hashset.insert(g.font_id);
                        })
                    });
                }
            }
        }

        hashset
    }
}

impl SlideShow {
    fn loading() -> SlideShow {
        let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
        let spinner_hash = hasher.hash_one("spinner");
        let halves_hash = hasher.hash_one("halves");
        SlideShow {
            slide_show: {
                let slide = AstObject::Slide {
                    objects: vec![SlideObj {
                        object: spinner_hash,
                        locations: [
                            (Align2::CENTER_TOP, ViewboxIn::Custom(halves_hash, 0)),
                            (Align2::CENTER_CENTER, ViewboxIn::Custom(halves_hash, 0)),
                        ],
                        scaled_time: [0.0, 0.5],
                        state: ObjectState::Entering,
                    }],
                    actions: vec![],
                    bg: (Color::default(), None),
                    max_time: 0.5,
                    next: false,
                };
                let mut map = IndexMap::default();
                map.insert(0, slide);
                map
            },
            viewboxes: {
                let mut map = HashMap::default();
                map.insert(
                    halves_hash,
                    UnresolvedLayout {
                        direction: Direction::Vertical,
                        margin: 15.0,
                        margin_per: 0.0,
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

                    fetch_ss.store(Arc::new(RwLock::new(slide_show.1)));
                    fetch_restart_timer.store(1, Ordering::Relaxed);
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
                        fetch_ss.store(Arc::new(RwLock::new(slide_show.1)));
                        fetch_restart_timer.store(1, Ordering::Relaxed);
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
        let mut helix_cell = None;

        #[cfg(not(target_arch = "wasm32"))]
        let mut parser = {
            let mut parser = helix_core::tree_sitter::Parser::new();
            parser.set_language(&tree_sitter_grz::language()).unwrap();
            parser
        };
        #[cfg(not(target_arch = "wasm32"))]
        let viewbox_nodes = Arc::new(ArcSwap::new(Arc::new(HashMap::default())));
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
                    (SlideShow::loading(), SlideShowSource::Http)
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
                    (SlideShow::loading(), SlideShowSource::Loaded)
                } else {
                    let mut slide_show = SlideShow {
                        slide_show: IndexMap::default(),
                        viewboxes,
                        objects,
                    };
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
                            Arc::clone(&viewbox_nodes),
                        )
                    };
                    match ast {
                        Ok(_) => {
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
        let slide_show = (SlideShow::loading(), SlideShowSource::Http);

        new_file.store(false, Ordering::Relaxed);

        (
            Self {
                slide_show: Arc::new(ArcSwap::new(Arc::new(RwLock::new(slide_show.0)))),
                #[cfg(not(target_arch = "wasm32"))]
                viewbox_nodes,
                next: Arc::new(false.into()),
                export: false,
                restart_timer: Arc::new(1.into()),
                #[cfg(not(target_arch = "wasm32"))]
                file_name: presentation.unwrap_or_default().into(),
                #[cfg(not(target_arch = "wasm32"))]
                vb_dbg: Arc::new(0.into()),
                #[cfg(not(target_arch = "wasm32"))]
                obj_dbg: Arc::new(0.into()),
                index: Arc::new(0.into()),
                time_offset: 0.0,
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
                in_drag: false,
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
        let ui_time = ctx.input(|i| i.time) as f32;
        #[cfg(not(target_arch = "wasm32"))]
        let speaker_viewport = ViewportId::from_hash_of("speaker_view");
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|input| input.key_down(egui::Key::Q) || input.key_down(egui::Key::Escape))
            && !self.lsp
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        let slide_show_cloned = Arc::clone(&self.slide_show);
        let slide_show = slide_show_cloned.load();
        let slide_show = slide_show.write();
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
                        if ui
                            .add_enabled(
                                self.index.load(Ordering::Relaxed) != 0,
                                egui::Button::new("<"),
                            )
                            .clicked()
                        {
                            self.index.fetch_sub(1, Ordering::Relaxed);
                            index -= 1;
                            self.next.store(false, Ordering::Relaxed);
                            self.resolved.store(None);
                            self.time_offset = 0.0;
                        } else if ui
                            .add_enabled(
                                self.index.load(Ordering::Relaxed)
                                    != slide_show.slide_show.len() - 1,
                                egui::Button::new(">"),
                            )
                            .clicked()
                        {
                            self.index.fetch_add(1, Ordering::Relaxed);
                            index += 1;
                            self.resolved.store(None);
                            self.next.store(true, Ordering::Relaxed);
                            self.time_offset = ui_time;
                        }
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
                let response = ui.allocate_rect(window_size, Sense::click_and_drag());

                let drag_delta = response.drag_delta().x;
                if (drag_delta < -20.0 && !self.in_drag) || response.clicked() {
                    self.in_drag = true;
                    let _ = self
                        .index
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |idx| {
                            if idx != slide_show.slide_show.len() - 1 {
                                self.resolved.store(None);
                                self.time_offset = ui_time;
                                index += 1;
                                #[cfg(not(target_arch = "wasm32"))]
                                self.speaker_view.clear_resolved();
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
                                self.next.store(true, Ordering::Relaxed);
                                Some(idx + 1)
                            } else {
                                None
                            }
                        });
                } else if drag_delta > 20.0 && !self.in_drag {
                    self.in_drag = true;
                    let _ = self
                        .index
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |idx| {
                            if idx != 0 {
                                self.resolved.store(None);
                                self.time_offset = 0.0;
                                index -= 1;
                                while matches!(
                                    slide_show.slide_show.get_index(index).map(|o| o.1),
                                    Some(
                                        AstObject::Action { next: true, .. }
                                            | AstObject::Slide { next: true, .. }
                                    )
                                ) {
                                    index -= 1;
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                self.speaker_view.clear_resolved();
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
                                self.next.store(false, Ordering::Relaxed);
                                return Some(idx - 1);
                            }
                            None
                        });
                } else if drag_delta == 0.0 {
                    self.in_drag = false;
                }

                let restart_timer = self.restart_timer.fetch_sub(1, Ordering::Relaxed);

                if !self.export && restart_timer >= 0 {
                    self.time_offset = ui_time;
                } else {
                    self.restart_timer.store(-1, Ordering::Relaxed);
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
                    if let Some((_, slide)) = slide_show.slide_show.get_index(index) {
                        #[cfg(not(target_arch = "wasm32"))]
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
                                    (actions, None),
                                    ui,
                                    window_size,
                                    &slide_show,
                                    font_system.deref_mut(),
                                    Arc::clone(&self.resolved_images),
                                ));
                                self.resolved.store(Some(Arc::clone(&resolved)));
                                resolved
                            }
                            AstObject::Action {
                                actions,
                                slide_in_ast,
                                ..
                            } => {
                                let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
                                match slide {
                                    AstObject::Slide {
                                        objects: slide,
                                        bg,
                                        actions: slide_actions,
                                        ..
                                    } => {
                                        self.clear_color = bg.0.into();
                                        let mut font_system = self.font_system.lock();
                                        let resolved = Arc::new(Resolved::resolve(
                                            slide,
                                            (slide_actions, Some(actions)),
                                            ui,
                                            window_size,
                                            &slide_show,
                                            font_system.deref_mut(),
                                            Arc::clone(&self.resolved_images),
                                        ));
                                        self.resolved.store(Some(Arc::clone(&resolved)));
                                        resolved
                                    }
                                    _ => Arc::new(Resolved::slideshow_end(window_size)),
                                }
                            }
                        }
                    } else {
                        Arc::new(Resolved::slideshow_end(window_size))
                    }
                };

                #[cfg(not(target_arch = "wasm32"))]
                if self.lsp {
                    ui.painter().rect_stroke(
                        resolved.draw_size,
                        Rounding::default(),
                        ui.style().visuals.widgets.noninteractive.bg_stroke,
                    );
                }

                let mut buffers = Vec::new();
                if let Some((_, slide)) = slide_show.slide_show.get_index(index) {
                    let time = ui_time - self.time_offset;

                    match slide {
                        AstObject::Slide {
                            max_time,
                            next,
                            bg: (bg, b),
                            ..
                        } => {
                            if let Some(b) = b {
                                let color: Color32 =
                                    bg.interpolate(b.1, time, b.0.as_secs_f32()).into();
                                if self.clear_color != color {
                                    ctx.request_repaint();
                                }
                                self.clear_color = color;
                            }
                            resolved.draw_slide(
                                ui,
                                time,
                                &mut buffers,
                                self.font_system.lock().deref_mut(),
                                self.export,
                            );
                            resolved.draw_actions(ui, time, self.export);

                            if time < *max_time {
                                ctx.request_repaint();
                            } else if *next && self.next.load(Ordering::Relaxed) {
                                self.index.fetch_add(1, Ordering::Relaxed);
                                self.resolved.store(None);
                                self.time_offset = ui_time;
                                #[cfg(not(target_arch = "wasm32"))]
                                self.speaker_view.clear_resolved();
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
                                ctx.request_repaint();
                            }
                        }
                        AstObject::Action {
                            slide_in_ast, next, ..
                        } => {
                            let slide = slide_show.slide_show.get(slide_in_ast).unwrap();
                            match slide {
                                AstObject::Slide { max_time, .. } => {
                                    resolved.draw_slide(
                                        ui,
                                        *max_time,
                                        &mut buffers,
                                        self.font_system.lock().deref_mut(),
                                        self.export,
                                    );
                                }
                                _ => todo!(),
                            }
                            resolved.draw_actions(ui, time, self.export);

                            if time < 0.5 {
                                ctx.request_repaint();
                            } else if *next && self.next.load(Ordering::Relaxed) {
                                self.index.fetch_add(1, Ordering::Relaxed);
                                self.resolved.store(None);
                                self.time_offset = ui_time;
                                #[cfg(not(target_arch = "wasm32"))]
                                self.speaker_view.clear_resolved();
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
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
                                    self.time_offset = ui_time;
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
                            |mut index| {
                                if index != 0 {
                                    self.resolved.store(None);
                                    self.time_offset = 0.0;
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.speaker_view.clear_resolved();
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.vb_dbg.store(0, Ordering::Relaxed);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.obj_dbg.store(0, Ordering::Relaxed);
                                    self.next.store(false, Ordering::Relaxed);
                                    index -= 1;
                                    while matches!(
                                        slide_show.slide_show.get_index(index).map(|o| o.1),
                                        Some(
                                            AstObject::Action { next: true, .. }
                                                | AstObject::Slide { next: true, .. }
                                        )
                                    ) {
                                        index -= 1;
                                    }
                                    return Some(index);
                                }
                                None
                            },
                        );
                    }
                    egui::Event::Key {
                        key: egui::Key::G,
                        modifiers: Modifiers::SHIFT,
                        pressed: true,
                        ..
                    } => {
                        self.index
                            .store(slide_show.slide_show.len() - 1, Ordering::Relaxed);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.vb_dbg.store(0, Ordering::Relaxed);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.obj_dbg.store(0, Ordering::Relaxed);
                        self.next.store(false, Ordering::Relaxed);
                        self.resolved.store(None);
                        #[cfg(not(target_arch = "wasm32"))]
                        self.speaker_view.clear_resolved();
                    }
                    egui::Event::Key {
                        key: egui::Key::R,
                        pressed: true,
                        ..
                    } => {
                        self.time_offset = ui_time;
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
                        slide_show.load().read().deref(),
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

    fn persist_egui_memory(&self) -> bool {
        false
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::MAX
    }
}
