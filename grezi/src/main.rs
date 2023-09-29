#![warn(clippy::all, rust_2018_idioms)]
#![allow(unreachable_patterns)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    cell::RefCell,
    collections::HashMap,
    hash::BuildHasherDefault,
    mem::ManuallyDrop,
    ops::Range,
    path::Path,
    rc::Rc,
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
    epaint::{mutex::Mutex, Color32, FontFamily, Pos2, Rounding, Vec2},
};
#[cfg(not(target_arch = "wasm32"))]
use imara_diff::{
    intern::{InternedInput, TokenSource},
    sources::ByteLines,
    Algorithm,
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
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use tree_sitter::Tree;

#[cfg(not(target_arch = "wasm32"))]
use crate::parser::highlighting::HelixCell;
use crate::parser::objects::ResolvedObject;

mod layout;
mod parser;

#[allow(dead_code)]
struct MyEguiApp {
    slide_show: SlideShow,
    #[cfg(not(target_arch = "wasm32"))]
    new_file: Arc<AtomicBool>,
    #[cfg(not(target_arch = "wasm32"))]
    slide_show_file: Arc<Mutex<String>>,
    #[cfg(not(target_arch = "wasm32"))]
    tree_info: Arc<Mutex<Option<(Tree, String)>>>,
    #[cfg(not(target_arch = "wasm32"))]
    file_name: String,
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
}

#[derive(Serialize, Deserialize)]
struct SlideShow {
    slide_show: Vec<AstObject>,
    viewboxes: HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
    objects: HashMap<u64, Object, BuildHasherDefault<PassThroughHasher>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
struct LinesWVec<'a> {
    vec: Rc<RefCell<Vec<Range<usize>>>>,
    iter: ByteLines<'a, true>,
    current_byte: usize,
}

#[cfg(target_arch = "wasm32")]
const PRESENTATION: &[u8] = include_bytes!(env!("PRESENTATION"));

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Iterator for LinesWVec<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            self.vec
                .borrow_mut()
                .push(self.current_byte..self.current_byte + next.len());
            self.current_byte += next.len();
            Some(next)
        } else {
            None
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> TokenSource for LinesWVec<'a> {
    type Token = &'a [u8];

    type Tokenizer = LinesWVec<'a>;

    fn tokenize(&self) -> Self::Tokenizer {
        self.clone()
    }

    fn estimate_tokens(&self) -> u32 {
        self.iter.estimate_tokens()
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glo::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        #[cfg(not(target_arch = "wasm32"))]
        let mut args = std::env::args().skip(1);
        #[cfg(not(target_arch = "wasm32"))]
        let file_name = args.next().unwrap();
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "Fira Code".to_owned(),
            FontData::from_static(include_bytes!("/usr/share/fonts/TTF/FiraCode-Regular.ttf")),
        );

        fonts.families.insert(
            FontFamily::Name("Fira Code".into()),
            vec!["Fira Code".to_owned()],
        );

        cc.egui_ctx.set_fonts(fonts);

        #[cfg(not(target_arch = "wasm32"))]
        let slide_show_file = Arc::new(Mutex::new(String::new()));
        #[cfg(not(target_arch = "wasm32"))]
        let new_file = Arc::new(AtomicBool::new(true));
        #[cfg(not(target_arch = "wasm32"))]
        let tree_info: Arc<Mutex<Option<(Tree, String)>>> = Arc::new(Mutex::new(None));
        #[cfg(not(target_arch = "wasm32"))]
        let watcher_tree_info = Arc::clone(&tree_info);
        #[cfg(not(target_arch = "wasm32"))]
        let watcher_context = cc.egui_ctx.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let watcher_file_name = file_name.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let watcher_slide_show_file = Arc::clone(&slide_show_file);
        #[cfg(not(target_arch = "wasm32"))]
        let watcher_new_file = Arc::clone(&new_file);
        #[cfg(not(target_arch = "wasm32"))]
        let mut instant = Instant::now();
        #[cfg(not(target_arch = "wasm32"))]
        let mut watcher = ManuallyDrop::new(
            notify::recommended_watcher(
                move |res: Result<notify::Event, notify::Error>| match res {
                    Ok(event) => match event.kind {
                        notify::EventKind::Modify(ModifyKind::Data(_)) => {
                            if Instant::now().duration_since(instant) > Duration::from_millis(250) {
                                std::thread::sleep(Duration::from_millis(250));
                                instant = Instant::now();
                                let new_file = std::fs::read_to_string(&watcher_file_name).unwrap();
                                let slide_show_file = watcher_slide_show_file.lock();
                                let slide_show = slide_show_file.as_str();
                                let mut tree_info = watcher_tree_info.lock();
                                if let Some(info) = tree_info.as_mut() {
                                    edit_tree(&slide_show, &new_file, &mut info.0);
                                    info.1 = new_file;
                                }

                                watcher_new_file.store(true, Ordering::Relaxed);
                                watcher_context.request_repaint();
                            }
                        }
                        _ => {}
                    },
                    Err(_) => {}
                },
            )
            .unwrap(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        watcher
            .watch(Path::new(&file_name), notify::RecursiveMode::NonRecursive)
            .unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        let mut helix_cell = None;

        #[cfg(not(target_arch = "wasm32"))]
        let slide_show: SlideShow = {
            let mut viewboxes = HashMap::default();
            let mut objects = HashMap::default();
            if file_name.ends_with(".slideshow") {
                let file = std::fs::read(&file_name).unwrap();
                postcard::from_bytes(&file).unwrap()
            } else {
                let mut tree_info = tree_info.lock();
                let file = std::fs::read_to_string(&file_name).unwrap();
                let ast = parser::parse_file(
                    &file_name,
                    file,
                    None,
                    &mut helix_cell,
                    &mut viewboxes,
                    &mut objects,
                );
                *tree_info = Some((ast.0, String::new()));
                match ast.1 {
                    Ok(ast) => {
                        *slide_show_file.lock() = ast.0;
                        SlideShow {
                            slide_show: ast.1,
                            viewboxes,
                            objects,
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e.get());
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
        if args.find(|f| f == "--export").is_some() {
            postcard::to_io(&slide_show, std::fs::File::create("out.slideshow").unwrap()).unwrap();
            std::process::exit(0);
        }

        Self {
            slide_show,
            #[cfg(not(target_arch = "wasm32"))]
            new_file,
            #[cfg(not(target_arch = "wasm32"))]
            slide_show_file,
            #[cfg(not(target_arch = "wasm32"))]
            tree_info,
            #[cfg(not(target_arch = "wasm32"))]
            file_name,
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
        }
    }

    fn draw_slide(&self, slide: &[ResolvedSlideObj], ui: &mut Ui, time: f32) {
        for obj in slide {
            let time = if obj.scaled_time[0] < time {
                time - obj.scaled_time[0]
            } else {
                0.0
            };
            let obj_pos;
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
                                    locations[0][0]
                                } else {
                                    keyframe::ease_with_scaled_time(
                                        Linear,
                                        locations[0][0],
                                        locations[1][0],
                                        time,
                                        0.5,
                                    )
                                },
                                locations[0][1],
                            ),
                            max: Pos2::new(
                                keyframe::ease_with_scaled_time(
                                    EaseOutQuint,
                                    locations[0][0],
                                    locations[1][0],
                                    time,
                                    0.5,
                                ),
                                locations[1][1],
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
    fn resolve_layout(&mut self, hash: u64, index: usize, size: Rect) -> Rect {
        match self.resolved_viewboxes.get(&hash) {
            None => {
                let split = match self.slide_show.viewboxes.get(&hash).unwrap().split_on {
                    ViewboxIn::Size => size,
                    ViewboxIn::Custom(hash, index) => self.resolve_layout(hash, index, size),
                };
                let unresolved_layout = self.slide_show.viewboxes.get(&hash).unwrap();
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
    ) -> Vec<ResolvedSlideObj> {
        let mut resolved_slides = Vec::new();
        let size = Rect::from_min_size(Pos2::ZERO, size);
        for object in slide {
            let first_viewbox = match object.locations[0].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => self.resolve_layout(hash, index, size),
            };
            let second_viewbox = match object.locations[1].1 {
                ViewboxIn::Size => size.shrink(15.0),
                ViewboxIn::Custom(hash, index) => self.resolve_layout(hash, index, size),
            };
            let obj = self.slide_show.objects.get(&object.object).unwrap();
            match &obj.object {
                parser::objects::ObjectType::Text { layout_job } => {
                    let mut layout_job = layout_job.clone();
                    layout_job.wrap.max_width = second_viewbox.width();
                    for row in layout_job.sections.iter_mut() {
                        row.format.font_id.size *= size.max.x / 1920.0;
                    }
                    let galley = ctx.fonts(|f| f.layout_job(layout_job));
                    let resolved_obj = ResolvedObject::Text(galley);
                    let size = resolved_obj.bounds();
                    use parser::viewboxes::LineUp;
                    let first_pos = get_pos!(object.locations[0].0, first_viewbox, size);
                    let second_pos = get_pos!(object.locations[1].0, second_viewbox, size);
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
                        locations: [
                            [locations.min.x, locations.min.y],
                            [locations.max.x, locations.max.y],
                        ],
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
        let mut button_hit = false;
        egui::TopBottomPanel::bottom("controls")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("⬅").clicked() {
                        if self.index != 0 {
                            self.index -= 1;
                            self.resolved_actions = None;
                            self.resolved_slide = None;
                        }
                        button_hit = true;
                        self.time = 1000.0;
                    } else if ui.button("➡").clicked() {
                        if self.index != self.slide_show.slide_show.len() {
                            self.index += 1;
                            self.resolved_actions = None;
                            self.resolved_slide = None;
                        }
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
            if self.new_file.load(Ordering::Relaxed) {
                let ast = {
                    let mut tree_info = self.tree_info.lock();
                    let mut info = tree_info.take();
                    let file = info.as_mut().map_or_else(
                        || std::fs::read_to_string(&self.file_name).unwrap(),
                        |i| {
                            if !i.1.is_empty() {
                                let mut new_string = String::new();
                                core::mem::swap(&mut new_string, &mut i.1);
                                new_string
                            } else {
                                std::fs::read_to_string(&self.file_name).unwrap()
                            }
                        },
                    );
                    self.slide_show.viewboxes.clear();
                    self.slide_show.objects.clear();
                    let ast = parser::parse_file(
                        &self.file_name,
                        file,
                        info.as_ref().map(|t| &t.0),
                        &mut self.helix_cell,
                        &mut self.slide_show.viewboxes,
                        &mut self.slide_show.objects,
                    );
                    *tree_info = Some((ast.0, String::new()));
                    match ast.1 {
                        Ok(ast) => {
                            self.delta = Instant::now();
                            self.time = 0.0;
                            *self.slide_show_file.lock() = ast.0;
                            ast.1
                        }
                        Err(e) => {
                            println!("{:?}", e.get());
                            std::process::exit(1);
                        }
                    }
                };

                self.resolved_actions = None;
                self.resolved_slide = None;
                self.resolved_viewboxes.clear();
                self.slide_show.slide_show = ast;
                self.new_file.store(false, Ordering::Relaxed);
            }

            let slide = self.slide_show.slide_show.get(self.index).unwrap() as *const AstObject;
            // This is safe because the resolution functions do not touch self.slide_show.slide_show
            let resolved_slide = match &self.resolved_slide {
                None => match unsafe { &*slide } {
                    AstObject::Slide {
                        objects: slide,
                        actions,
                        ..
                    } => {
                        let resolved_slide = self.resolve_slide(
                            slide,
                            ctx,
                            Vec2::new(self.window_size[0] as f32, self.window_size[1] as f32),
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
                        let slide = self.slide_show.slide_show.get(*slide_in_ast).unwrap()
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
            };
            let slide = self.slide_show.slide_show.get(self.index).unwrap();

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
                    let slide = self.slide_show.slide_show.get(*slide_in_ast).unwrap();
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
                            self.index += 1;
                            self.resolved_actions = None;
                            self.resolved_slide = None;
                            if self.index == self.slide_show.slide_show.len() {
                                #[cfg(not(target_arch = "wasm32"))]
                                frame.close();
                                #[cfg(target_arch = "wasm32")]
                                {
                                    self.index -= 1;
                                }
                            }
                            self.time = 0.0;
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
                            }
                            self.time = 1000.0;
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
    eframe::run_native(
        "Grezi",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
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
                Box::new(|cc| Box::new(MyEguiApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

// Benchmarks show that diffing and editing the tree is faster than
// Parsing it all over again
#[cfg(not(target_arch = "wasm32"))]
fn edit_tree(before: &str, after: &str, tree: &mut Tree) {
    let before_vec = Rc::new(RefCell::new(Vec::new()));
    let after_vec = Rc::new(RefCell::new(Vec::new()));
    let input = InternedInput::new(
        LinesWVec {
            vec: Rc::clone(&before_vec),
            iter: imara_diff::sources::byte_lines_with_terminator(before.as_bytes()),
            current_byte: 0,
        },
        LinesWVec {
            vec: Rc::clone(&after_vec),
            iter: imara_diff::sources::byte_lines_with_terminator(after.as_bytes()),
            current_byte: 0,
        },
    );
    let mut line_offset: isize = 0;
    let mut byte_offset: isize = 0;
    let sink = |before: Range<u32>, after: Range<u32>| {
        let before_vec = before_vec.borrow();
        let after_vec = after_vec.borrow();
        let input_edit = tree_sitter::InputEdit {
            start_byte: (before_vec[(before.start as isize) as usize].start as isize + byte_offset)
                as usize,
            old_end_byte: (before_vec[((before.end - 1) as isize) as usize].end as isize
                + byte_offset) as usize,
            new_end_byte: after_vec[after.end as usize - 1].end,
            start_position: tree_sitter::Point {
                row: (before.start as isize + line_offset) as usize,
                column: 0,
            },
            old_end_position: tree_sitter::Point {
                row: ((before.end - 1) as isize + line_offset) as usize,
                column: input.interner
                    [input.before[((before.end - 1) as isize + line_offset) as usize]]
                    .len(),
            },
            new_end_position: tree_sitter::Point {
                row: after.end as usize - 1,
                column: input.interner[input.after[after.end as usize - 1]].len(),
            },
        };
        if after.start == after.end {
            let iter = before.into_iter();
            line_offset -= iter.len() as isize;

            for i in iter {
                byte_offset -= before_vec[i as usize].len() as isize;
            }
        } else if before.start == before.end {
            let iter = after.into_iter();
            line_offset += iter.len() as isize;

            for i in iter {
                byte_offset += after_vec[i as usize].len() as isize;
            }
        }
        tree.edit(&input_edit);
    };
    imara_diff::diff(Algorithm::Histogram, &input, sink);
}
