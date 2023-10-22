#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    mem::ManuallyDrop,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use eframe::{
    egui::{self, FontData, FontDefinitions, PointerButton, Rect, Ui},
    epaint::{
        mutex::{Mutex, RwLock},
        Color32, FontFamily, Pos2, Rounding, Vec2,
    },
};
use keyframe::functions::{EaseOutCubic, EaseOutQuint, Linear};
use layout::UnresolvedLayout;
#[cfg(not(target_arch = "wasm32"))]
use notify::{event::ModifyKind, Watcher};
use parser::{
    actions::{Actions, ResolvedActions},
    objects::{Object, ObjectState},
    slides::{ResolvedSlideObj, SlideObj},
    viewboxes::ViewboxIn,
    AstObject, PassThroughHasher,
};
#[cfg(not(target_arch = "wasm32"))]
use ropey::Rope;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use tree_sitter::Tree;

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
    presentation: Option<PathBuf>,
    #[clap(short, long)]
    export: bool,
    #[clap(short, long)]
    dont_close: bool,
    #[clap(long)]
    lsp: bool,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyEguiApp {
    slide_show: Arc<RwLock<SlideShow>>,
    #[cfg(not(target_arch = "wasm32"))]
    clear_resolved: Arc<AtomicBool>,
    #[cfg(not(target_arch = "wasm32"))]
    slide_show_file: Arc<Mutex<Rope>>,
    #[cfg(not(target_arch = "wasm32"))]
    tree_info: Arc<Mutex<Option<(Tree, Rope)>>>,
    #[cfg(not(target_arch = "wasm32"))]
    file_name: String,
    #[cfg(not(target_arch = "wasm32"))]
    dont_exit: bool,
    index: usize,
    #[cfg(not(target_arch = "wasm32"))]
    delta: Instant,
    #[cfg(not(target_arch = "wasm32"))]
    helix_cell: Option<HelixCell>,
    window_size: [u32; 2],
    // Safe, I think, IDK
    resolved_viewboxes: HashMap<u64, Vec<Rect>, BuildHasherDefault<PassThroughHasher>>,
    resolved_actions: Option<Vec<ResolvedActions>>,
    resolved_slide: Option<Vec<ResolvedSlideObj>>,
    time: f32,
    #[cfg(not(target_arch = "wasm32"))]
    lsp: bool,
    #[cfg(not(target_arch = "wasm32"))]
    parser: Arc<Mutex<tree_sitter::Parser>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SlideShow {
    slide_show: Vec<AstObject>,
    viewboxes: HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    objects: HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
}

#[cfg(target_arch = "wasm32")]
const PRESENTATION: &[u8] = include_bytes!(env!("PRESENTATION"));

impl MyEguiApp {
    fn init_app(mut self, cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "Fira Code".to_owned(),
            FontData::from_static(include_bytes!("../fira/ttf/FiraCode-Regular.ttf")),
        );

        fonts.families.insert(
            FontFamily::Name("Fira Code".into()),
            vec!["Fira Code".to_owned()],
        );

        cc.egui_ctx.set_fonts(fonts);

        #[cfg(not(target_arch = "wasm32"))]
        if !self.lsp {
            let watcher_tree_info = Arc::clone(&self.tree_info);
            let watcher_context = cc.egui_ctx.clone();
            let watcher_file_name = self.file_name.clone();
            let watcher_slide_show_file = Arc::clone(&self.slide_show_file);
            let watcher_new_file = Arc::clone(&self.clear_resolved);
            let watcher_parser = Arc::clone(&self.parser);
            let watcher_slide_show = Arc::clone(&self.slide_show);
            let mut instant = Instant::now();
            let mut w = ManuallyDrop::new(
                notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                    match res {
                        Ok(event) => match event.kind {
                            notify::EventKind::Modify(ModifyKind::Data(_)) => {
                                if Instant::now().duration_since(instant)
                                    > Duration::from_millis(250)
                                {
                                    std::thread::sleep(Duration::from_millis(250));
                                    instant = Instant::now();
                                    let new_file = Rope::from_reader(
                                        std::fs::File::open(&watcher_file_name).unwrap(),
                                    )
                                    .unwrap();
                                    let mut slide_show_file = watcher_slide_show_file.lock();
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
                                            info.0.edit(change);
                                        }

                                        info.1 = new_file;
                                        info.0 = watcher_parser
                                            .lock()
                                            .parse_with(
                                                &mut |byte, _| {
                                                    if byte <= info.1.len_bytes() {
                                                        let (chunk, start_byte, _, _) =
                                                            info.1.chunk_at_byte(byte);
                                                        &chunk.as_bytes()[byte - start_byte..]
                                                    } else {
                                                        // out of range
                                                        &[]
                                                    }
                                                },
                                                Some(&info.0),
                                            )
                                            .unwrap();

                                        let mut slide_show = watcher_slide_show.write();

                                        slide_show.viewboxes.clear();
                                        slide_show.objects.clear();
                                        let ast = parser::parse_file(
                                            &info.0,
                                            &info.1,
                                            &mut self.helix_cell,
                                            &mut slide_show,
                                        );
                                        match ast {
                                            Ok(ast) => {
                                                *slide_show_file = info.1.clone();
                                                slide_show.slide_show = ast;
                                                watcher_new_file.store(true, Ordering::Relaxed);
                                            }
                                            Err(errors) => {
                                                for error in errors {
                                                    eprintln!(
                                                        "{:?}",
                                                        parser::ErrWithSource {
                                                            error,
                                                            source_code: info.1.to_string()
                                                        }
                                                    );
                                                }
                                            }
                                        }
                                    }

                                    watcher_context.request_repaint();
                                }
                            }
                            _ => {}
                        },
                        Err(_) => {}
                    }
                })
                .unwrap(),
            );

            w.watch(self.file_name.as_ref(), notify::RecursiveMode::NonRecursive)
                .unwrap();
        }
        MyEguiApp {
            #[cfg(not(target_arch = "wasm32"))]
            helix_cell: None,
            ..self
        }
    }
    fn new(#[cfg(not(target_arch = "wasm32"))] args: &Args) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glo::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        #[cfg(not(target_arch = "wasm32"))]
        let slide_show_file = Arc::new(Mutex::new(Rope::new()));
        #[cfg(not(target_arch = "wasm32"))]
        let new_file = Arc::new(AtomicBool::new(true));
        #[cfg(not(target_arch = "wasm32"))]
        let tree_info: Arc<Mutex<Option<(Tree, Rope)>>> = Arc::new(Mutex::new(None));

        #[cfg(not(target_arch = "wasm32"))]
        let mut helix_cell = None;

        #[cfg(not(target_arch = "wasm32"))]
        let mut parser = {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(tree_sitter_grz::language()).unwrap();
            parser
        };
        #[cfg(not(target_arch = "wasm32"))]
        let slide_show: SlideShow = {
            let viewboxes = HashMap::default();
            let objects = HashMap::default();
            if args.lsp {
                SlideShow::default()
            } else if args
                .presentation
                .as_ref()
                .unwrap()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                == "slideshow"
            {
                let file = std::fs::read(args.presentation.as_ref().unwrap()).unwrap();
                postcard::from_bytes(&file).unwrap()
            } else {
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
                let ast = parser::parse_file(&tree, &file, &mut helix_cell, &mut slide_show);
                match ast {
                    Ok(ast) => {
                        *tree_info = Some((tree, Rope::new()));
                        *slide_show_file.lock() = file;
                        slide_show.slide_show = ast;
                        slide_show
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
        let slide_show: SlideShow = { postcard::from_bytes(PRESENTATION).unwrap() };

        #[cfg(not(target_arch = "wasm32"))]
        new_file.store(false, Ordering::Relaxed);

        #[cfg(not(target_arch = "wasm32"))]
        if args.export {
            postcard::to_io(&slide_show, std::fs::File::create("out.slideshow").unwrap()).unwrap();
            std::process::exit(0);
        }

        Self {
            slide_show: Arc::new(RwLock::new(slide_show)),
            #[cfg(not(target_arch = "wasm32"))]
            clear_resolved: new_file,
            #[cfg(not(target_arch = "wasm32"))]
            slide_show_file,
            #[cfg(not(target_arch = "wasm32"))]
            tree_info,
            #[cfg(not(target_arch = "wasm32"))]
            file_name: args
                .presentation
                .clone()
                .unwrap_or_default()
                .display()
                .to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            dont_exit: args.dont_close,
            index: 0,
            #[cfg(not(target_arch = "wasm32"))]
            delta: Instant::now(),
            time: 0.0,
            #[cfg(not(target_arch = "wasm32"))]
            helix_cell,
            resolved_viewboxes: HashMap::default(),
            resolved_actions: None,
            resolved_slide: None,
            #[cfg(not(target_arch = "wasm32"))]
            window_size: [0, 0],
            #[cfg(target_arch = "wasm32")]
            window_size: [1920, 1080],
            #[cfg(not(target_arch = "wasm32"))]
            lsp: args.lsp,
            #[cfg(not(target_arch = "wasm32"))]
            parser: Arc::new(Mutex::new(parser)),
        }
    }

    fn draw_slide(&self, slide: &[ResolvedSlideObj], ui: &mut Ui, time: f32) {
        for obj in slide {
            let time = if obj.scaled_time[0] < time {
                time - obj.scaled_time[0]
            } else {
                0.0
            };
            let mut obj_pos;
            match &obj.object {
                parser::objects::ResolvedObject::Text(galley) => {
                    obj_pos = Pos2::from(keyframe::ease_with_scaled_time(
                        EaseOutCubic,
                        obj.locations[0],
                        obj.locations[1],
                        time,
                        obj.scaled_time[1],
                    ));
                    ui.painter().galley(obj_pos, Arc::clone(galley));
                    obj_pos.x += galley.rect.min.x;
                    // ui.painter()
                    // .circle(obj_pos, 5.0, Color32::RED, Stroke::NONE);
                }
            }

            match obj.state {
                ObjectState::Entering => {
                    let fade_color =
                        Color32::from_gray(27).gamma_multiply(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            1.0,
                            0.0,
                            time,
                            obj.scaled_time[1],
                        ));
                    ui.painter().rect_filled(
                        obj.object.bounds().translate(obj_pos.to_vec2()),
                        Rounding::none(),
                        fade_color,
                    );
                }
                ObjectState::Exiting => {
                    let fade_color =
                        Color32::from_gray(27).gamma_multiply(keyframe::ease_with_scaled_time(
                            EaseOutCubic,
                            0.0,
                            1.0,
                            time,
                            obj.scaled_time[1],
                        ));
                    ui.painter().rect_filled(
                        obj.object.bounds().translate(obj_pos.to_vec2()),
                        Rounding::none(),
                        fade_color,
                    );
                }
                ObjectState::OnScreen => {}
            }
        }
    }

    fn draw_actions(&self, actions: &[ResolvedActions], ui: &mut Ui, time: f32) {
        for action in actions {
            match action {
                parser::actions::ResolvedActions::Highlight { locations, persist } => {
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
                        },
                        Rounding::none(),
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
        ctx: &egui::Context,
        size: Vec2,
        slide_show: &SlideShow,
    ) -> Vec<ResolvedSlideObj> {
        let mut resolved_slides = Vec::new();
        let size = Rect::from_min_size(Pos2::ZERO, size);
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    self.resolve_layout(hash, index, size, &*slide_show)
                }
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => {
                    self.resolve_layout(hash, index, size, &*slide_show)
                }
            };
            let obj = slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                parser::objects::ObjectType::Text { layout_job } => {
                    let mut layout_job = layout_job.clone();
                    layout_job.wrap.max_width = second_viewbox.width();
                    for row in layout_job.sections.iter_mut() {
                        row.format.font_id.size *= size.max.x / 1920.0;
                    }
                    let galley = ctx.fonts(|f| f.layout_job(layout_job));
                    let galley_x = -galley.rect.min.x;
                    let resolved_obj = ResolvedObject::Text(galley);
                    let size = resolved_obj.bounds();
                    use parser::viewboxes::LineUp;
                    let mut first_pos = get_pos!(object.locations[0].0, first_viewbox, size);
                    let mut second_pos = get_pos!(object.locations[1].0, second_viewbox, size);
                    first_pos[0] += galley_x;
                    second_pos[0] += galley_x;
                    resolved_slides.push(ResolvedSlideObj {
                        object: resolved_obj,
                        locations: [first_pos, second_pos],
                        scaled_time: object.scaled_time,
                        state: object.state,
                    });
                }
            }
        }
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
                        from_rect
                            .union(to_rect)
                            .translate(Vec2::from(text_object.locations[1]))
                    } else {
                        match &text_object.object {
                            ResolvedObject::Text(galley) => {
                                galley.rect.translate(Vec2::from(text_object.locations[1]))
                            }
                            _ => todo!(),
                        }
                    };
                    resolved_actions.push(ResolvedActions::Highlight {
                        locations,
                        persist: *persist,
                    });
                }
            }
        }
        resolved_actions
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let delta = self.delta;
            self.delta = Instant::now();
            let delta = self.delta.duration_since(delta);
            self.time += delta.as_secs_f32();
            if self.window_size == [0, 0] {
                return;
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.time += 0.01666667;
        }
        #[cfg(not(target_arch = "wasm32"))]
        ctx.input(|input| {
            if input.key_down(egui::Key::Q) || input.key_down(egui::Key::Escape) {
                frame.close();
            }
        });
        #[cfg(target_arch = "wasm32")]
        let slide_show_cloned = Arc::clone(&self.slide_show);
        #[cfg(target_arch = "wasm32")]
        let slide_show = slide_show_cloned.read();
        let mut button_hit = false;
        #[cfg(target_arch = "wasm32")]
        egui::TopBottomPanel::bottom("controls")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.add_enabled(self.index != 0, egui::Button::new("⬅")).clicked() {
                        self.index -= 1;
                        self.resolved_actions = None;
                        self.resolved_slide = None;
                        button_hit = true;
                        self.time = 1000.0;
                    } else if ui.add_enabled(self.index != slide_show.slide_show.len() - 1, egui::Button::new("➡")).clicked() {
                        self.index += 1;
                        self.resolved_actions = None;
                        self.resolved_slide = None;
                        button_hit = true;
                        self.time = 0.0;
                    }
                    ui.label("This presentation was made using Grezi, created by Isaac Mills, the guy who made this portfolio!");
                    ui.hyperlink_to("Check out the source code!", "https://github.com/StratusFearMe21/grezi-next");
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            #[cfg(target_arch = "wasm32")]
            {
                let available = ui.available_size();
                let window_size = [available.x as u32, available.y as u32];
                if self.window_size != window_size {
                    self.time = 0.0;
                    self.window_size = window_size;
                    self.resolved_viewboxes.clear();
                    self.resolved_actions = None;
                    self.resolved_slide = None;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            if self.clear_resolved.load(Ordering::Relaxed) {
                self.resolved_actions = None;
                self.resolved_slide = None;
                self.resolved_viewboxes.clear();
                self.delta = Instant::now();
                self.time = 0.0;
                self.clear_resolved.store(false, Ordering::Relaxed);
            }

            #[cfg(not(target_arch = "wasm32"))]
            let slide_show_cloned = Arc::clone(&self.slide_show);
            #[cfg(not(target_arch = "wasm32"))]
            let slide_show = slide_show_cloned.read();

            // This is safe because the resolution functions do not touch self.slide_show.slide_show
            let resolved_slide = {
                if let Some(slide) = slide_show
                    .slide_show
                    .get(self.index)
                    .or_else(|| slide_show.slide_show.last())
                {
                    let slide = slide as *const AstObject;
                    match &self.resolved_slide {
                        None => match unsafe { &*slide } {
                            AstObject::Slide {
                                objects: slide,
                                actions,
                                ..
                            } => {
                                let resolved_slide = self.resolve_slide(
                                    slide,
                                    ctx,
                                    Vec2::new(
                                        self.window_size[0] as f32,
                                        self.window_size[1] as f32,
                                    ),
                                    &*slide_show,
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
                                let slide = slide_show.slide_show.get(*slide_in_ast).unwrap()
                                    as *const AstObject;
                                match unsafe { &*slide } {
                                    AstObject::Slide { objects, .. } => {
                                        let resolved_slide = self.resolve_slide(
                                            objects,
                                            ctx,
                                            Vec2::new(
                                                self.window_size[0] as f32,
                                                self.window_size[1] as f32,
                                            ),
                                            &*slide_show,
                                        );
                                        self.resolved_actions =
                                            Some(self.resolve_actions(actions, &resolved_slide));
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
            if let Some(slide) = slide_show
                .slide_show
                .get(self.index)
                .or_else(|| slide_show.slide_show.last())
            {
                let resolved_actions = match &self.resolved_actions {
                    None => unreachable!(),
                    Some(resolved) => resolved,
                };

                match slide {
                    AstObject::Slide { max_time, .. } => {
                        self.draw_slide(resolved_slide, ui, self.time);
                        self.draw_actions(resolved_actions, ui, self.time);

                        if self.time < *max_time {
                            ctx.request_repaint();
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
                                if self.index == slide_show.slide_show.len() - 1 {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if !self.dont_exit {
                                        frame.close();
                                    }
                                } else {
                                    self.index += 1;
                                    self.resolved_actions = None;
                                    self.resolved_slide = None;
                                    self.time = 0.0;
                                }
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
                                if self.index != 0 {
                                    self.index -= 1;
                                    self.resolved_actions = None;
                                    self.resolved_slide = None;
                                    self.time = 1000.0;
                                }
                            }
                            egui::Event::Key {
                                key: egui::Key::R,
                                pressed: true,
                                ..
                            } => {
                                self.time = 0.0;
                            }
                            egui::Event::Key {
                                key: egui::Key::B,
                                pressed: true,
                                ..
                            } => {
                                self.index = 0;
                                self.resolved_actions = None;
                                self.resolved_slide = None;
                            }
                            _ => {}
                        }
                    }
                })
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn post_rendering(&mut self, window_size_px: [u32; 2], _frame: &eframe::Frame) {
        if self.window_size != window_size_px {
            self.time = 0.0;
            self.window_size = window_size_px;
            self.resolved_viewboxes.clear();
            self.resolved_actions = None;
            self.resolved_slide = None;
        }
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        fullscreen: true,
        resizable: true,
        vsync: true,
        ..Default::default()
    };
    let args: Args = clap::Parser::parse();
    let app = MyEguiApp::new(&args);
    let init_app = app.clone();

    eframe::run_native(
        "Grezi",
        native_options,
        Box::new(move |cc| {
            if args.lsp {
                let lsp_egui_ctx = cc.egui_ctx.clone();
                let current_thread = std::thread::current();
                std::thread::spawn(move || lsp::start_lsp(app, current_thread, lsp_egui_ctx));
                std::thread::park();
            }
            Box::new(init_app.init_app(cc))
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
                Box::new(|cc| Box::new(MyEguiApp::new().init_app(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
