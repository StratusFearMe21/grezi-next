#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    io::{BufWriter, Cursor},
    mem::ManuallyDrop,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use eframe::{
    egui::{self, FontData, FontDefinitions, Image, PointerButton, Rect, SizeHint, Ui},
    epaint::{
        mutex::{Mutex, RwLock},
        text::LayoutJob,
        Color32, FontFamily, FontId, Pos2, Rounding, Stroke, Vec2,
    },
};
use egui_anim::Anim;
#[cfg(not(target_arch = "wasm32"))]
use font_loader::system_fonts::FontPropertyBuilder;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::ropey::Rope;
#[cfg(not(target_arch = "wasm32"))]
use helix_core::tree_sitter::Tree;
use image::codecs::{png::PngDecoder, webp::WebPDecoder};
use keyframe::functions::{EaseOutCubic, EaseOutQuint, Linear};
use layout::{Constraint, UnresolvedLayout};
#[cfg(not(target_arch = "wasm32"))]
use notify::{event::ModifyKind, Watcher};
use parser::{
    actions::{Actions, ResolvedActions},
    color::Color,
    objects::{Object, ObjectState},
    slides::{ResolvedSlideObj, SlideObj},
    viewboxes::{LineUp, ViewboxIn},
    AstObject, PassThroughHasher,
};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::parser::highlighting::HelixCell;
use crate::parser::objects::ResolvedObject;

mod layout;
#[cfg(not(target_arch = "wasm32"))]
mod lsp;
mod parser;

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser)]
struct Args {
    presentation: Option<String>,
    #[clap(short, long)]
    export: bool,
    #[clap(long)]
    lsp: bool,
    #[clap(long)]
    fmt: bool,
    #[clap(short, long)]
    dont_close: bool,
    #[clap(short, long)]
    thumbnail: bool,
    #[clap(short, long)]
    output: Option<String>,
    #[clap(short, long)]
    size: Option<f32>,
    /// Automatically advance to the next page after the given number of seconds
    #[clap(short, long)]
    auto: Option<u64>,
    /// Specifies the expected run time of the presentation
    #[clap(long)]
    duration: Option<humantime::Duration>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyEguiApp {
    slide_show: Arc<RwLock<SlideShow>>,
    clear_resolved: Arc<AtomicBool>,
    next: Arc<AtomicBool>,
    restart_timer: Arc<AtomicBool>,
    #[cfg(not(target_arch = "wasm32"))]
    slide_show_file: Arc<Mutex<Rope>>,
    #[cfg(not(target_arch = "wasm32"))]
    tree_info: Arc<Mutex<Option<Tree>>>,
    #[cfg(not(target_arch = "wasm32"))]
    file_name: String,
    #[cfg(not(target_arch = "wasm32"))]
    dont_exit: bool,
    #[cfg(not(target_arch = "wasm32"))]
    vb_dbg: Arc<AtomicU64>,
    #[cfg(not(target_arch = "wasm32"))]
    obj_dbg: Arc<AtomicU64>,
    index: Arc<AtomicUsize>,
    #[cfg(not(target_arch = "wasm32"))]
    helix_cell: Option<HelixCell>,
    window_size: Vec2,
    // Safe, I think, IDK
    resolved_viewboxes: HashMap<u64, Vec<Rect>, BuildHasherDefault<PassThroughHasher>>,
    resolved_actions: Option<Vec<ResolvedActions>>,
    resolved_slide: Option<Vec<ResolvedSlideObj>>,
    resolved_images: HashMap<u64, ResolvedObject, BuildHasherDefault<PassThroughHasher>>,
    time: f32,
    #[cfg(not(target_arch = "wasm32"))]
    lsp: bool,
    #[cfg(not(target_arch = "wasm32"))]
    parser: Arc<Mutex<helix_core::tree_sitter::Parser>>,
    clear_color: Color32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SlideShow {
    slide_show: Vec<AstObject>,
    viewboxes: HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    objects: HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
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
                        },
                    },
                );
                map
            },
        }
    }
}

enum SlideShowSource {
    Loaded,
    Http,
}

impl MyEguiApp {
    fn init_app(
        mut self,
        egui_ctx: &egui::Context,
        fonts: FontDefinitions,
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
                let fetch_clear_resolved = Arc::clone(&self.clear_resolved);
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
                    fetch_clear_resolved.store(true, Ordering::Relaxed);
                });

                #[cfg(target_arch = "wasm32")]
                ehttp::fetch(ehttp::Request::get(hash), move |response| {
                    let res = response.unwrap();
                    let slide_show: (FontDefinitions, SlideShow) =
                        bincode::deserialize(&res.bytes).unwrap();
                    fetch_ctx.set_fonts(slide_show.0);
                    *fetch_ss.write() = slide_show.1;
                    fetch_restart_timer.store(true, Ordering::Relaxed);
                    fetch_clear_resolved.store(true, Ordering::Relaxed);
                });
            }
            SlideShowSource::Loaded => {
                #[cfg(not(target_arch = "wasm32"))]
                if !self.lsp {
                    use std::time::Instant;

                    let watcher_tree_info = Arc::clone(&self.tree_info);
                    let watcher_context = egui_ctx.clone();
                    let watcher_file_name = self.file_name.clone();
                    let watcher_slide_show_file = Arc::clone(&self.slide_show_file);
                    let watcher_new_file = Arc::clone(&self.clear_resolved);
                    let watcher_restart_timer = Arc::clone(&self.restart_timer);
                    let watcher_parser = Arc::clone(&self.parser);
                    let watcher_slide_show = Arc::clone(&self.slide_show);
                    let mut instant = Instant::now();
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
                                                std::fs::File::open(&watcher_file_name).unwrap(),
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
                                                        Some(&info),
                                                    )
                                                    .unwrap();

                                                let mut slide_show = watcher_slide_show.write();

                                                let ast = parser::parse_file(
                                                    &tree,
                                                    Some(&info),
                                                    &new_file,
                                                    &mut self.helix_cell,
                                                    &mut slide_show,
                                                    &watcher_context,
                                                    std::path::Path::new(&watcher_file_name),
                                                );
                                                *info = tree;
                                                match ast {
                                                    Ok(_) => {
                                                        *slide_show_file = new_file.clone();
                                                        watcher_new_file
                                                            .store(true, Ordering::Relaxed);
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

                    w.watch(self.file_name.as_ref(), notify::RecursiveMode::NonRecursive)
                        .unwrap();
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        egui_ctx.set_fonts(fonts);

        MyEguiApp {
            #[cfg(not(target_arch = "wasm32"))]
            helix_cell: None,
            ..self
        }
    }
    fn new(
        #[cfg(not(target_arch = "wasm32"))] args: &Args,
    ) -> (Self, FontDefinitions, SlideShowSource) {
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
        let mut parser = {
            let mut parser = helix_core::tree_sitter::Parser::new();
            parser.set_language(tree_sitter_grz::language()).unwrap();
            parser
        };
        #[cfg(not(target_arch = "wasm32"))]
        let slide_show: (SlideShow, SlideShowSource) = {
            let viewboxes = HashMap::default();
            let objects = HashMap::default();
            if args.lsp {
                (SlideShow::default(), SlideShowSource::Loaded)
            } else if args.presentation.as_ref().unwrap().ends_with("slideshow") {
                if args.presentation.as_ref().unwrap().starts_with("http") {
                    (SlideShow::exercise_jit(), SlideShowSource::Http)
                } else {
                    let file = std::fs::read(args.presentation.as_ref().unwrap()).unwrap();
                    let slideshow: (FontDefinitions, SlideShow) =
                        bincode::deserialize(&file).unwrap();

                    fonts = slideshow.0;

                    (slideshow.1, SlideShowSource::Loaded)
                }
            } else {
                let fira_code_prop = FontPropertyBuilder::new().family("Fira Code").build();
                if let Some(font) = font_loader::system_fonts::get(&fira_code_prop) {
                    fonts
                        .font_data
                        .insert("Fira Code".to_owned(), FontData::from_owned(font.0));

                    fonts
                        .families
                        .get_mut(&FontFamily::Monospace)
                        .unwrap()
                        .insert(0, "Fira Code".to_owned());
                }
                let mut slide_show = SlideShow {
                    slide_show: Vec::new(),
                    viewboxes,
                    objects,
                };
                let mut tree_info = tree_info.lock();
                let file = Rope::from_reader(
                    std::fs::File::open(args.presentation.as_ref().unwrap()).unwrap(),
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
                        &std::fs::canonicalize(args.presentation.as_ref().unwrap()).unwrap(),
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
        };

        #[cfg(target_arch = "wasm32")]
        let slide_show = (SlideShow::exercise_jit(), SlideShowSource::Http);

        new_file.store(false, Ordering::Relaxed);

        #[cfg(not(target_arch = "wasm32"))]
        if args.export {
            bincode::serialize_into(
                BufWriter::new(std::fs::File::create("out.slideshow").unwrap()),
                &(fonts, slide_show.0),
            )
            .unwrap();
            std::process::exit(0);
        }

        (
            Self {
                slide_show: Arc::new(RwLock::new(slide_show.0)),
                clear_resolved: new_file,
                next: Arc::new(false.into()),
                restart_timer: Arc::new(false.into()),
                #[cfg(not(target_arch = "wasm32"))]
                slide_show_file,
                #[cfg(not(target_arch = "wasm32"))]
                tree_info,
                #[cfg(not(target_arch = "wasm32"))]
                file_name: args.presentation.clone().unwrap_or_default(),
                #[cfg(not(target_arch = "wasm32"))]
                dont_exit: args.dont_close,
                #[cfg(not(target_arch = "wasm32"))]
                vb_dbg: Arc::new(0.into()),
                #[cfg(not(target_arch = "wasm32"))]
                obj_dbg: Arc::new(0.into()),
                index: Arc::new(0.into()),
                time: 0.0,
                #[cfg(not(target_arch = "wasm32"))]
                helix_cell,
                resolved_viewboxes: HashMap::default(),
                resolved_actions: None,
                resolved_slide: None,
                resolved_images: HashMap::default(),
                window_size: Vec2::ZERO,
                #[cfg(not(target_arch = "wasm32"))]
                lsp: args.lsp,
                #[cfg(not(target_arch = "wasm32"))]
                parser: Arc::new(Mutex::new(parser)),
                clear_color: Color::default().into(),
            },
            fonts,
            slide_show.1,
        )
    }

    fn draw_slide(&self, slide: &[ResolvedSlideObj], ui: &mut Ui, time: f32) {
        for obj in slide {
            let time = if obj.scaled_time[0] < time {
                (time - obj.scaled_time[0]).clamp(0.0, obj.scaled_time[1])
            } else {
                0.0
            };
            let mut obj_pos = Rect::from([
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
                    obj_pos = obj_pos.translate(egui::vec2(-galley.rect.min.x, 0.0));
                    // ui.painter()
                    // .circle(obj_pos, 5.0, Color32::RED, Stroke::NONE);

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
                    ui.painter()
                        .galley_with_gamma(obj_pos.min, Arc::clone(galley), gamma_multiply);
                }
                ResolvedObject::Image {
                    image,
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
                    image
                        .clone()
                        .fit_to_exact_size(scale.unwrap_or_else(|| obj_pos.size()))
                        .tint(tint)
                        .paint_at(ui, obj_pos)
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
                    Image::from_uri(anim.find_img(ui.ctx()))
                        .fit_to_exact_size(scale.unwrap_or_else(|| obj_pos.size()))
                        .tint(tint)
                        .paint_at(ui, obj_pos)
                }
                ResolvedObject::Spinner => egui::Spinner::new().paint_at(&ui, obj_pos),
            }
        }
    }

    fn draw_actions(&self, actions: &[ResolvedActions], ui: &mut Ui, time: f32) {
        for action in actions {
            match action {
                parser::actions::ResolvedActions::Highlight {
                    locations,
                    persist,
                    locations_of_object,
                    scaled_time,
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
                                        0.5,
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
                                    0.5,
                                ),
                                locations.max.y,
                            ),
                        }
                        .translate(obj_pos),
                        Rounding::ZERO,
                        Color32::LIGHT_YELLOW.gamma_multiply(0.5),
                    );
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
    ) -> Rect {
        match self.resolved_viewboxes.get(&hash) {
            None => {
                let split = match slide_show.viewboxes.get(&hash).unwrap().split_on {
                    ViewboxIn::Size => size,
                    ViewboxIn::Custom(hash, index) => {
                        self.resolve_layout(hash, index, size, slide_show)
                    }
                };

                let unresolved_layout = slide_show.viewboxes.get(&hash).unwrap();
                let mut constraints = unresolved_layout.constraints.clone();
                constraints.iter_mut().for_each(|c| match c {
                    layout::Constraint::Length(length) => *length *= size.max.x / 1920.0,
                    layout::Constraint::Min(min) => *min *= size.max.x / 1920.0,
                    layout::Constraint::Max(max) => *max *= size.max.x / 1920.0,
                    _ => {}
                });
                let layout = layout::Layout::default()
                    .direction(unresolved_layout.direction)
                    .constraints(&unresolved_layout.constraints)
                    .split(split);
                let rect = layout[index];
                self.resolved_viewboxes.insert(hash, layout);
                rect
            }
            Some(viewboxes) => viewboxes.get(index).copied().unwrap(),
        }
    }

    fn resolve_slide(
        &mut self,
        slide: &[SlideObj],
        ui: &mut Ui,
        size: Vec2,
        slide_show: &SlideShow,
    ) -> Vec<ResolvedSlideObj> {
        let mut resolved_slides = Vec::new();
        let size = Rect::from_min_size(Pos2::ZERO, size);
        let mut images = Vec::with_capacity(3);
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    self.resolve_layout(hash, index, size, slide_show)
                }
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    self.resolve_layout(hash, index, size, slide_show)
                }
            };
            let obj = slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                parser::objects::ObjectType::Spinner => {
                    let size = ResolvedObject::Spinner.bounds(second_viewbox.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox, size).into(),
                        size.size().into(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, size).into(),
                        size.size().into(),
                    );
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved_slides.push(ResolvedSlideObj {
                        object: ResolvedObject::Spinner,
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
                parser::objects::ObjectType::Text { layout_job } => {
                    let mut layout_job = layout_job.clone();
                    layout_job.wrap.max_width = second_viewbox.width();
                    for row in layout_job.sections.iter_mut() {
                        row.format.font_id.size *= size.max.x / 1920.0;
                    }
                    let galley = ui.ctx().fonts(|f| f.layout_job(layout_job));
                    let resolved_obj = ResolvedObject::Text(galley);
                    let size = resolved_obj.bounds(second_viewbox.size(), ui);
                    let first_pos = Rect::from_min_size(
                        get_pos!(object.locations[0].0, first_viewbox, size).into(),
                        size.size().into(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, size).into(),
                        size.size().into(),
                    );
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved_slides.push(ResolvedSlideObj {
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
                    let resolved_obj = self
                        .resolved_images
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
                            ResolvedObject::Image {
                                image: Image::from_bytes(uri.clone(), Arc::clone(bytes)),
                                tint: *tint,
                                scale: *scale,
                            }
                        });
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
                        first_size.size().into(),
                    );
                    let first_pos = [first_pos.min, first_pos.max];
                    let second_pos = Rect::from_min_size(
                        get_pos!(object.locations[1].0, second_viewbox, second_size).into(),
                        second_size.size().into(),
                    );
                    let second_pos = [second_pos.min, second_pos.max];
                    resolved_slides.push(ResolvedSlideObj {
                        object: resolved_obj.clone(),
                        locations: [first_pos.map(|f| f.into()), second_pos.map(|f| f.into())],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
            }
        }
        self.resolved_images
            .retain(|k, _| images.binary_search(k).is_ok());
        resolved_slides
    }

    fn resolve_actions(
        &self,
        actions: &[Actions],
        slide: &[ResolvedSlideObj],
    ) -> Vec<ResolvedActions> {
        let mut resolved_actions = Vec::new();
        for action in actions {
            match action {
                Actions::Highlight {
                    locations,
                    index,
                    persist,
                } => {
                    let text_object = slide.get(*index).unwrap();
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
                        match &text_object.object {
                            ResolvedObject::Text(_) => {
                                let to_rect = Rect::from([
                                    Pos2::from(text_object.locations[1][0]),
                                    Pos2::from(text_object.locations[1][1]),
                                ]);
                                Rect::from_min_size(Pos2::new(0.0, 0.0), to_rect.size())
                            }
                            _ => todo!(),
                        }
                    };
                    resolved_actions.push(ResolvedActions::Highlight {
                        locations,
                        persist: *persist,
                        locations_of_object: [
                            text_object.locations[0][0],
                            text_object.locations[1][0],
                        ],
                        scaled_time: text_object.scaled_time,
                    });
                }
            }
        }
        resolved_actions
    }

    fn update(&mut self, ctx: &egui::Context, frame: Option<&mut eframe::Frame>) {
        self.time += ctx.input(|i| i.stable_dt);
        #[cfg(not(target_arch = "wasm32"))]
        ctx.input(|input| {
            if input.key_down(egui::Key::Q) || input.key_down(egui::Key::Escape) {
                if let Some(&mut ref mut frame) = frame {
                    frame.close();
                }
            }
        });
        let slide_show_cloned = Arc::clone(&self.slide_show);
        let slide_show = slide_show_cloned.read();
        #[cfg(target_arch = "wasm32")]
        let mut button_hit = false;
        #[cfg(not(target_arch = "wasm32"))]
        let button_hit = false;
        let mut index = self.index.load(Ordering::Relaxed);
        if index >= slide_show.slide_show.len() {
            index = slide_show.slide_show.len() - 1;
            self.index
                .store(slide_show.slide_show.len() - 1, Ordering::Relaxed);
            self.next.store(false, Ordering::Relaxed);
        }
        if let Some(slide) = slide_show.slide_show.get(index) {
            match slide {
                AstObject::Slide { bg, .. } => {
                    if let Some(b) = bg.1 {
                        let color: Color32 =
                            bg.0.interpolate(b.1, self.time, b.0.as_secs_f32()).into();
                        self.clear_color = color;
                    }
                }
                _ => {}
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.clear_color))
            .show(ctx, |ui| {
                let window_size = ui.available_size();
                if self.window_size != window_size {
                    self.window_size = window_size;
                    self.resolved_viewboxes.clear();
                    self.resolved_actions = None;
                    self.resolved_slide = None;
                }
                if self.clear_resolved.load(Ordering::Relaxed) {
                    self.resolved_actions = None;
                    self.resolved_slide = None;
                    self.resolved_viewboxes.clear();
                    self.clear_resolved.store(false, Ordering::Relaxed);
                }
                if self.restart_timer.load(Ordering::Relaxed) {
                    self.time = 0.0;
                    self.restart_timer.store(false, Ordering::Relaxed);
                }

                // This is safe because the resolution functions do not touch self.slide_show.slide_show
                let resolved_slide = {
                    if let Some(slide) = slide_show.slide_show.get(index) {
                        match &self.resolved_slide {
                            None => match slide {
                                AstObject::Slide {
                                    objects: slide,
                                    actions,
                                    ..
                                } => {
                                    let resolved_slide = self.resolve_slide(
                                        slide,
                                        ui,
                                        self.window_size,
                                        &slide_show,
                                    );
                                    self.resolved_actions =
                                        Some(self.resolve_actions(actions, &resolved_slide));
                                    self.resolved_slide = Some(resolved_slide);
                                    self.resolved_slide.as_ref().unwrap()
                                }
                                AstObject::Action {
                                    actions,
                                    slide_in_ast,
                                } => {
                                    let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                                    match slide {
                                        AstObject::Slide { objects, .. } => {
                                            let resolved_slide = self.resolve_slide(
                                                objects,
                                                ui,
                                                self.window_size,
                                                &slide_show,
                                            );
                                            self.resolved_actions = Some(
                                                self.resolve_actions(actions, &resolved_slide),
                                            );
                                            self.resolved_slide = Some(resolved_slide);
                                            self.resolved_slide.as_ref().unwrap()
                                        }
                                        _ => todo!(),
                                    }
                                }
                            },
                            Some(resolved) => resolved,
                        }
                    } else {
                        return;
                    }
                };
                if let Some(slide) = slide_show.slide_show.get(index) {
                    let resolved_actions = match &self.resolved_actions {
                        None => unreachable!(),
                        Some(resolved) => resolved,
                    };

                    match slide {
                        AstObject::Slide { max_time, next, .. } => {
                            self.draw_slide(resolved_slide, ui, self.time);
                            self.draw_actions(resolved_actions, ui, self.time);

                            if self.time < *max_time {
                                ctx.request_repaint();
                            } else if *next && self.next.load(Ordering::Relaxed) {
                                self.index.fetch_add(1, Ordering::Relaxed);
                                self.resolved_actions = None;
                                self.resolved_slide = None;
                                self.time = 0.0;
                                #[cfg(not(target_arch = "wasm32"))]
                                self.vb_dbg.store(0, Ordering::Relaxed);
                                #[cfg(not(target_arch = "wasm32"))]
                                self.obj_dbg.store(0, Ordering::Relaxed);
                            }
                        }
                        AstObject::Action { slide_in_ast, .. } => {
                            let slide = slide_show.slide_show.get(*slide_in_ast).unwrap();
                            match slide {
                                AstObject::Slide { max_time, .. } => {
                                    self.draw_slide(resolved_slide, ui, *max_time);
                                }
                                _ => todo!(),
                            }
                            self.draw_actions(resolved_actions, ui, self.time);

                            if self.time < 0.5 {
                                ctx.request_repaint();
                            }
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let vb_dbg = self.vb_dbg.load(Ordering::Relaxed);
                        if vb_dbg > 0 {
                            if let Some(vb) = self.resolved_viewboxes.get(&vb_dbg) {
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
        #[cfg(target_arch = "wasm32")]
        egui::TopBottomPanel::bottom("controls")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.add_enabled(self.index.load(Ordering::Relaxed) != 0, egui::Button::new("⬅")).clicked() {
                        self.index.fetch_sub(1, Ordering::Relaxed);
                        self.resolved_actions = None;
                        self.next.store(false, Ordering::Relaxed);
                        self.resolved_slide = None;
                        button_hit = true;
                        self.time = 1000.0;
                    } else if ui.add_enabled(self.index.load(Ordering::Relaxed) != slide_show.slide_show.len() - 1, egui::Button::new("➡")).clicked() {
                        self.index.fetch_add(1, Ordering::Relaxed);
                        self.resolved_actions = None;
                        self.next.store(true, Ordering::Relaxed);
                        self.resolved_slide = None;
                        button_hit = true;
                        self.time = 0.0;
                    }
                    ui.label("This presentation was made using Grezi, created by Isaac Mills, the guy who made this portfolio!");
                    ui.hyperlink_to("Check out the source code!", "https://github.com/StratusFearMe21/grezi-next");
                })
            });
        ctx.input(|input| {
            for event in input.events.iter() {
                match event {
                    egui::Event::Key {
                        key: egui::Key::ArrowRight | egui::Key::Space,
                        pressed: true,
                        ..
                    }
                    | egui::Event::PointerButton {
                        button: PointerButton::Primary,
                        pressed: false,
                        ..
                    } if !button_hit => {
                        let _ = self.index.fetch_update(
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                            |index| {
                                if index == slide_show.slide_show.len() - 1 {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if !self.dont_exit {
                                        if let Some(&mut ref mut frame) = frame {
                                            frame.close();
                                        }
                                    }
                                    None
                                } else {
                                    self.resolved_actions = None;
                                    self.resolved_slide = None;
                                    self.time = 0.0;
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.vb_dbg.store(0, Ordering::Relaxed);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    self.obj_dbg.store(0, Ordering::Relaxed);
                                    self.next.store(true, Ordering::Relaxed);
                                    Some(index + 1)
                                }
                            },
                        );
                    }
                    egui::Event::Key {
                        key: egui::Key::ArrowLeft,
                        pressed: true,
                        ..
                    }
                    | egui::Event::PointerButton {
                        button: PointerButton::Secondary,
                        pressed: false,
                        ..
                    } if !button_hit => {
                        let _ = self.index.fetch_update(
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                            |index| {
                                if index != 0 {
                                    self.resolved_actions = None;
                                    self.resolved_slide = None;
                                    self.time = 1000.0;
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
                        self.resolved_actions = None;
                        self.resolved_slide = None;
                    }
                    _ => {}
                }
            }
        })
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx, Some(frame))
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::egui::ImageSize;
    use rand::{Rng, SeedableRng};

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let args: Args = clap::Parser::parse();
    let native_options = eframe::NativeOptions {
        fullscreen: !args.lsp,
        resizable: true,
        vsync: true,
        ..Default::default()
    };
    let mut app = MyEguiApp::new(&args);
    let init_app = app.0.clone();

    if args.fmt {
        let mut file = app.0.slide_show_file.lock();
        match lsp::format_code(&app.0, &file) {
            Ok(edits) => {
                let transaction = helix_lsp::util::generate_transaction_from_edits(
                    &file,
                    edits,
                    helix_lsp::OffsetEncoding::Utf16,
                );

                if transaction.apply(&mut file) {
                    println!("{}", *file);
                } else {
                    panic!("Transaction could not be applied");
                }
            }
            Err(error) => {
                eprintln!(
                    "{:?}",
                    parser::ErrWithSource {
                        error,
                        source_code: file.to_string()
                    }
                );
            }
        }
        return Ok(());
    } else if args.thumbnail {
        let slide_num = {
            let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
            rand_xoshiro::Xoshiro128PlusPlus::seed_from_u64(
                hasher.hash_one(&*app.0.slide_show_file.lock()),
            )
            .gen_range(0..app.0.slide_show.read().slide_show.len())
        };
        app.0.index.store(slide_num, Ordering::SeqCst);
        let size = Vec2::new(args.size.unwrap_or(128.0), args.size.unwrap_or(128.0));
        let fit = ImageSize {
            max_size: size,
            ..Default::default()
        }
        .calc_size(size, Vec2::new(1920.0, 1080.0));
        let pixmap = egui_rasterizer::rasterize((fit.x, fit.y), |ctx| {
            app.0.time = f32::MAX;
            app.0.init_app(ctx, app.1, app.2).update(ctx, None)
        });

        image::save_buffer(
            args.output.unwrap(),
            pixmap.data(),
            pixmap.width(),
            pixmap.height(),
            image::ColorType::Rgba8,
        )
        .unwrap();

        return Ok(());
    }

    eframe::run_native(
        "Grezi",
        native_options,
        Box::new(move |cc| {
            if args.lsp {
                let lsp_egui_ctx = cc.egui_ctx.clone();
                egui_extras::install_image_loaders(&lsp_egui_ctx);
                if !lsp_egui_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                    lsp_egui_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
                }
                let current_thread = std::thread::current();
                std::thread::spawn(move || lsp::start_lsp(app.0, current_thread, lsp_egui_ctx));
                std::thread::park();
            }
            Box::new(init_app.init_app(&cc.egui_ctx, app.1, app.2))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions {
        follow_system_theme: false,
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    Box::new({
                        let app = MyEguiApp::new();
                        app.0.init_app(
                            &cc.egui_ctx,
                            app.1,
                            app.2,
                            &cc.integration_info.web_info.location.hash[1..],
                        )
                    })
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
