use cairo::freetype;
use cairo::FontFace;
use cairo::ImageSurface;
use cairo::TextClusterFlags;
use ecolor::Color32;
use ecolor::Rgba;
use eframe::egui;
use eframe::egui::Modifiers;
use eframe::egui::Pos2;
use eframe::egui::Rect;
use eframe::egui::Vec2;
use eframe::epaint::mutex::Mutex;
use eframe::epaint::ColorMode;
use eframe::epaint::TextureId;
use egui_glyphon::glyphon::fontdb::ID;
use egui_glyphon::glyphon::Cursor;
use egui_glyphon::glyphon::FontSystem;
use egui_glyphon::glyphon::LayoutGlyph;
use egui_glyphon::glyphon::LayoutRun;
use egui_glyphon::glyphon::LayoutRunIter;
use egui_glyphon::GlyphonRendererCallback;
use gtk4::prelude::*;
use gtk4::EventControllerKey;
use indexmap::IndexSet;
use lsp_server::IoThreads;
use rangemap::RangeMap;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

use crate::args::Args;
use crate::parser::objects::serde_suck::CursorSerde;
use crate::MyEguiApp;

pub fn fonts_to_ft(
    font_system: Arc<Mutex<FontSystem>>,
    used_fonts: &IndexSet<ID, ahash::RandomState>,
    ft: &freetype::Library,
) -> HashMap<ID, (freetype::Face, cairo::FontFace)> {
    let mut font_system = font_system.lock();
    used_fonts
        .iter()
        .copied()
        .map(|f| {
            let data = unsafe { font_system.db_mut().make_shared_face_data(f) }.unwrap();
            let face = ft
                .new_memory_face(Rc::new(data.0.deref().as_ref().to_owned()), data.1 as isize)
                .unwrap();
            let cairo_face = FontFace::create_from_ft(&face).unwrap();
            (f, (face, cairo_face))
        })
        .collect()
}

pub fn cairo_draw(
    output: egui::FullOutput,
    textures: &mut HashMap<TextureId, (ImageSurface, bool)>,
    ctx: &cairo::Context,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    for (id, tex) in output.textures_delta.set {
        let surface = match tex.image {
            egui::ImageData::Color(c) => ImageSurface::create_for_data(
                c.pixels
                    .iter()
                    .flat_map(|c| {
                        let c = c.to_array();
                        [c[2], c[1], c[0], c[3]]
                    })
                    .collect::<Vec<_>>(),
                cairo::Format::ARgb32,
                c.width() as i32,
                c.height() as i32,
                cairo::Format::ARgb32
                    .stride_for_width(c.width() as u32)
                    .unwrap(),
            )
            .unwrap(),
            _ => continue,
        };

        if let Some(pos) = tex.pos {
            let texture = textures.get_mut(&id).unwrap();

            let ctx = cairo::Context::new(&mut texture.0).unwrap();

            ctx.set_source_surface(&surface, 0.0, 0.0).unwrap();
            ctx.rectangle(
                pos[0] as f64,
                pos[1] as f64,
                surface.width() as f64,
                surface.height() as f64,
            );
            ctx.fill().unwrap();
        } else {
            textures.insert(id, (surface, false));
        }
    }

    for shape in output.shapes {
        ctx.reset_clip();

        ctx.rectangle(
            shape.clip_rect.min.x as f64,
            shape.clip_rect.min.y as f64,
            shape.clip_rect.width() as f64,
            shape.clip_rect.height() as f64,
        );

        ctx.clip();

        cairo_draw_shape(
            ctx,
            shape.shape,
            textures,
            ft,
            Arc::clone(&font_system),
            fonts,
        );
    }

    for id in output.textures_delta.free {
        textures.remove(&id);
    }
}

fn convert_color(color: Color32) -> palette::Srgba<f64> {
    let color: palette::LinSrgba<f32> = palette::cast::from_array(Rgba::from(color).to_array());
    let color = palette::Srgba {
        alpha: color.alpha,
        color: palette::Srgb::new(
            ecolor::gamma_from_linear(color.red),
            ecolor::gamma_from_linear(color.green),
            ecolor::gamma_from_linear(color.blue),
        ),
    };
    color.into_format()
}

pub fn cairo_draw_shape(
    ctx: &cairo::Context,
    shape: eframe::epaint::Shape,
    textures: &mut HashMap<TextureId, (ImageSurface, bool)>,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    use cairo::{SurfacePattern, TextCluster};

    match shape {
        egui::Shape::Noop | egui::Shape::Text(_) => {}
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                cairo_draw_shape(ctx, shape, textures, ft, Arc::clone(&font_system), fonts);
            }
        }
        egui::Shape::LineSegment { points, stroke } => {
            let ColorMode::Solid(stroke_color) = stroke.color else {
                panic!("Stroke color was UV?")
            };
            let color = convert_color(stroke_color);
            ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);
            ctx.set_line_width(stroke.width as f64);

            ctx.move_to(points[0].x as f64, points[0].y as f64);
            ctx.line_to(points[1].x as f64, points[1].y as f64);
            ctx.stroke().unwrap();
        }
        egui::Shape::Rect(rect) => {
            let color = convert_color(rect.fill);

            let texture = textures.get_mut(&rect.fill_texture_id);
            if let Some(texture) = texture {
                if !texture.1 {
                    let ctx = cairo::Context::new(&texture.0).unwrap();
                    ctx.set_operator(cairo::Operator::Multiply);
                    ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);
                    ctx.mask_surface(&texture.0, 0.0, 0.0).unwrap();
                    texture.1 = true;
                }
                ctx.save().unwrap();
                ctx.translate(rect.rect.min.x as f64, rect.rect.min.y as f64);
                let ratio = rect.rect.width() as f64 / texture.0.width() as f64;
                ctx.scale(ratio, ratio);
                ctx.set_source(&SurfacePattern::create(&texture.0)).unwrap();
                ctx.paint_with_alpha(rect.uv.min.x as f64).unwrap();
                ctx.restore().unwrap();
            } else {
                ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

                ctx.set_fill_rule(cairo::FillRule::EvenOdd);
                ctx.rectangle(
                    rect.rect.min.x as f64,
                    rect.rect.min.y as f64,
                    rect.rect.width() as f64,
                    rect.rect.height() as f64,
                );
                ctx.fill().unwrap();
                ctx.set_fill_rule(cairo::FillRule::Winding);
            }

            ctx.set_line_width(rect.stroke.width as f64);
            if rect.stroke.width > 0.0 {
                let color = convert_color(rect.stroke.color);
                ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

                ctx.rectangle(
                    rect.rect.min.x as f64,
                    rect.rect.min.y as f64,
                    rect.rect.width() as f64,
                    rect.rect.height() as f64,
                );
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Callback(glyphon_callback) => {
            let callback = glyphon_callback
                .callback
                .downcast_ref::<GlyphonRendererCallback<Option<Arc<RangeMap<CursorSerde, String>>>>>()
                .unwrap();

            struct RunIter<'a> {
                run: Rc<LayoutRun<'a>>,
                glyphs: std::slice::Iter<'a, LayoutGlyph>,
                iter: LayoutRunIter<'a>,
            }

            impl<'a> Iterator for RunIter<'a> {
                type Item = (Rc<LayoutRun<'a>>, &'a LayoutGlyph);

                fn next(&mut self) -> Option<Self::Item> {
                    let next = self.glyphs.next().or_else(|| {
                        self.run = Rc::new(self.iter.next()?);
                        self.glyphs = self.run.glyphs.iter();
                        self.glyphs.next()
                    })?;

                    Some((Rc::clone(&self.run), next))
                }
            }

            impl<'a> RunIter<'a> {
                fn new(mut iter: LayoutRunIter<'a>) -> RunIter<'a> {
                    let run = Rc::new(iter.next().unwrap());
                    let glyphs = run.glyphs.iter();
                    RunIter { run, glyphs, iter }
                }
            }

            for buffer in callback.buffers.iter() {
                let buffer_read = buffer.buffer.read();
                ctx.set_font_size(buffer_read.metrics().font_size as f64);
                let mut glyphs = RunIter::new(buffer_read.layout_runs()).peekable();
                let mut link_tags = 0;

                if let Some(urls) = buffer.associated_data.clone() {
                    for url in urls.iter() {
                        link_tags += 1;

                        let start: Cursor = url.0.start.into();
                        let mut end: Cursor = url.0.end.into();

                        end.index -= 1;

                        let rects = crate::parser::objects::cosmic_jotdown::link_area(
                            buffer_read.deref(),
                            start,
                            end,
                        );

                        if rects.is_empty() {
                            continue;
                        }

                        let mut tag = String::from("rect=[");
                        // let mut red = 1.0;

                        for mut rect in rects {
                            rect = rect.translate(buffer.rect.min.to_vec2());
                            let size = rect.size();

                            // ctx.set_source_rgba(red, 1.0, 1.0, 1.0);
                            // ctx.rectangle(
                            //     rect.min.x as f64,
                            //     rect.min.y as f64,
                            //     size.x as f64,
                            //     size.y as f64,
                            // );
                            // ctx.fill().unwrap();

                            // red -= 0.1;

                            std::fmt::Write::write_fmt(
                                &mut tag,
                                format_args!(
                                    "{} {} {} {} ",
                                    rect.min.x, rect.min.y, size.x, size.y
                                ),
                            )
                            .unwrap();
                        }

                        tag.pop();
                        std::fmt::Write::write_fmt(&mut tag, format_args!("] uri='{}'", url.1))
                            .unwrap();

                        ctx.tag_begin("Link", &tag);
                    }
                }

                while glyphs.peek().is_some() {
                    let color;
                    let orig_color;
                    let font_id;
                    let rtl;
                    if let Some(glyph) = glyphs.peek() {
                        orig_color = glyph.1.color_opt.unwrap_or(buffer.default_color);
                        color = egui_glyphon::glyphon::Color::rgba(
                            (orig_color.r() as f32 * buffer.opacity + 0.5) as u8,
                            (orig_color.g() as f32 * buffer.opacity + 0.5) as u8,
                            (orig_color.b() as f32 * buffer.opacity + 0.5) as u8,
                            (orig_color.a() as f32 * buffer.opacity + 0.5) as u8,
                        );
                        font_id = glyph.1.font_id;
                        rtl = glyph.0.rtl;
                    } else {
                        glyphs.next();
                        continue;
                    }
                    let color_rgba: palette::Srgba<u8> = palette::cast::from_array(color.as_rgba());
                    let color_rgba: palette::Srgba<f64> = color_rgba.into_format();
                    let font = fonts.entry(font_id).or_insert_with(|| {
                        let mut font_system = font_system.lock();
                        let data =
                            unsafe { font_system.db_mut().make_shared_face_data(font_id) }.unwrap();
                        let face = ft
                            .new_memory_face(
                                Rc::new(data.0.deref().as_ref().to_owned()),
                                data.1 as isize,
                            )
                            .unwrap();
                        let cairo_face = FontFace::create_from_ft(&face).unwrap();
                        (face, cairo_face)
                    });

                    ctx.set_source_rgba(
                        color_rgba.red,
                        color_rgba.green,
                        color_rgba.blue,
                        color_rgba.alpha,
                    );
                    ctx.set_font_face(&font.1);

                    let mut new_glyphs = Vec::new();
                    let mut text = String::new();
                    let mut clusters = Vec::new();
                    while let Some(g) = glyphs.peek() {
                        if g.1.color_opt.unwrap_or(buffer.default_color) != orig_color
                            || g.1.font_id != font_id
                            || g.0.rtl != rtl
                        {
                            break;
                        }

                        let t = &g.0.text[g.1.start..g.1.end];
                        text.push_str(&t);
                        clusters.push(TextCluster::new(t.len() as i32, 1));
                        let glyph = cairo::Glyph::new(
                            g.1.glyph_id as _,
                            buffer.rect.left() as f64 + g.1.x as f64,
                            buffer.rect.top() as f64 + g.0.line_y as f64 + g.1.y as f64,
                        );
                        new_glyphs.push(glyph);
                        glyphs.next();
                    }

                    ctx.show_text_glyphs(
                        &text,
                        &new_glyphs,
                        &clusters,
                        if rtl {
                            TextClusterFlags::Backward
                        } else {
                            TextClusterFlags::None
                        },
                    )
                    .unwrap();
                }

                for _ in 0..link_tags {
                    ctx.tag_end("Link");
                }
            }
        }
        egui::Shape::Circle(circle) => {
            let color = convert_color(circle.fill);
            ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

            ctx.arc(
                circle.center.x as f64,
                circle.center.y as f64,
                circle.radius as f64,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            ctx.fill().unwrap();

            ctx.set_line_width(circle.stroke.width as f64);
            if circle.stroke.width > 0.0 {
                let color = convert_color(circle.stroke.color);
                ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

                ctx.arc(
                    circle.center.x as f64,
                    circle.center.y as f64,
                    circle.radius as f64,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Ellipse(ellipse) => {
            let color = convert_color(ellipse.fill);
            ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);
            let matrix = ctx.matrix();
            ctx.translate(ellipse.center.x as f64, ellipse.center.y as f64);
            ctx.scale(ellipse.radius.x as f64, ellipse.radius.y as f64);
            ctx.translate(-ellipse.center.x as f64, -ellipse.center.y as f64);

            ctx.arc(
                ellipse.center.x as f64,
                ellipse.center.y as f64,
                1.0,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            ctx.set_matrix(matrix);
            ctx.fill().unwrap();

            ctx.set_line_width(ellipse.stroke.width as f64);
            if ellipse.stroke.width > 0.0 {
                let color = convert_color(ellipse.stroke.color);
                ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);
                let matrix = ctx.matrix();
                ctx.translate(ellipse.center.x as f64, ellipse.center.y as f64);
                ctx.scale(ellipse.radius.x as f64, ellipse.radius.y as f64);
                ctx.translate(-ellipse.center.x as f64, -ellipse.center.y as f64);

                ctx.arc(
                    ellipse.center.x as f64,
                    ellipse.center.y as f64,
                    1.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                ctx.set_matrix(matrix);
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Path(_path) => {}
        egui::Shape::Mesh(_mesh) => {}
        egui::Shape::QuadraticBezier(_qb) => {}
        egui::Shape::CubicBezier(_cb) => {}
    }
}

use gtk::glib;
use gtk4 as gtk;

pub fn run_gtk() -> gtk4::glib::ExitCode {
    std::env::set_var("GSK_RENDERER", "cairo");
    let application = gtk::Application::builder()
        .application_id("com.github.grezi")
        .build();

    application.connect_activate(build_ui);
    application.run_with_args::<String>(&[])
}

fn build_ui(application: &gtk::Application) {
    let font_system = Arc::new(Mutex::new(FontSystem::new()));
    let args: Args = clap::Parser::parse();

    let app = MyEguiApp::new(args.lsp, args.presentation, Arc::clone(&font_system));
    let egui_ctx = egui::Context::default();

    let mut init_app = app.0.clone().init_app(&egui_ctx, app.1);
    init_app.export = true;
    let used_fonts = init_app
        .slide_show
        .load()
        .read()
        .used_fonts(&mut font_system.lock());
    let ft = cairo::freetype::Library::init().unwrap();

    let mut font_defs_ft = fonts_to_ft(Arc::clone(&font_system), &used_fonts, &ft);

    let io_threads: Rc<OnceCell<(IoThreads, _)>> = Rc::new(OnceCell::new());

    let io_threads_run = Rc::clone(&io_threads);
    if args.lsp {
        let lsp_egui_ctx = egui_ctx.clone();
        egui_extras::install_image_loaders(&lsp_egui_ctx);
        if !lsp_egui_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
            lsp_egui_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
        }
        // Create the transport. Includes the stdio (stdin and stdout) versions but this could
        // also be implemented to use sockets or HTTP.
        let (connection, iot) = lsp_server::Connection::stdio();
        let _ = io_threads_run.set((iot, connection.sender.clone()));
        std::thread::spawn(move || crate::lsp::start_lsp(app.0, lsp_egui_ctx, connection));
    }

    // if let Some(iot) = Rc::get_mut(&mut io_threads).unwrap().take() {
    //     iot.1
    //         .send(lsp_server::Message::Notification(
    //             lsp_server::Notification {
    //                 method: lsp_types::notification::Exit::METHOD.to_owned(),
    //                 params: serde_json::to_value(()).unwrap(),
    //             },
    //         ))
    //         .unwrap();
    //     iot.0.join().unwrap();
    // }

    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Grezi"));

    let draw_area = gtk::DrawingArea::builder().hexpand(true).build();

    window.set_child(Some(&draw_area));

    let mut textures: HashMap<TextureId, (ImageSurface, bool)> = HashMap::default();
    let beginning = Instant::now();

    let mut input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::ZERO)),
        //pixels_per_point: Some(2.0),
        ..Default::default()
    };

    let event_controller = EventControllerKey::new();

    pub struct UnsafeSendSync<T>(pub T);
    unsafe impl<T> Send for UnsafeSendSync<T> {}
    unsafe impl<T> Sync for UnsafeSendSync<T> {}

    impl<T> AsRef<T> for UnsafeSendSync<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    let redraw = Arc::new(AtomicBool::new(false));
    let r = Arc::clone(&redraw);
    let da = UnsafeSendSync(draw_area.clone());
    egui_ctx.set_request_repaint_callback(move |_| {
        r.store(true, std::sync::atomic::Ordering::Relaxed);
        da.as_ref().queue_draw();
    });

    let (key_sender, key_receiver) = std::sync::mpsc::channel();

    draw_area.set_draw_func(move |area, ctx, width, height| {
        let redraw = Arc::clone(&redraw);
        input.events = key_receiver.try_iter().collect();
        input.screen_rect = Some(Rect::from_min_size(
            Pos2::ZERO,
            Vec2::new(width as f32, height as f32),
        ));
        input.time = Some(beginning.elapsed().as_secs_f64());

        let output = egui_ctx.run(input.clone(), |ctx| init_app.update(ctx));

        cairo_draw(
            output,
            &mut textures,
            &ctx,
            &ft,
            Arc::clone(&font_system),
            &mut font_defs_ft,
        );

        let da = UnsafeSendSync(area.clone());
        glib::source::idle_add(move || {
            if redraw.swap(false, std::sync::atomic::Ordering::Relaxed) {
                da.as_ref().queue_draw();
            }

            glib::ControlFlow::Break
        });
    });

    event_controller.connect_key_pressed(move |_ev_key, key, _key_code, _modifier| {
        use egui::Key as EguiKey;
        use gtk::gdk::Key;
        let key = match key {
            Key::BackSpace => EguiKey::Backspace,
            Key::Down => EguiKey::ArrowDown,
            Key::Up => EguiKey::ArrowUp,
            Key::Left => EguiKey::ArrowLeft,
            Key::Right => EguiKey::ArrowRight,
            Key::KP_Enter | Key::ISO_Enter => EguiKey::Enter,
            Key::space | Key::KP_Space => EguiKey::Space,
            Key::Page_Up => EguiKey::PageUp,
            Key::Page_Down => EguiKey::PageDown,
            Key::colon => EguiKey::Colon,
            Key::comma => EguiKey::Comma,
            Key::backslash => EguiKey::Backslash,
            Key::slash => EguiKey::Slash,
            Key::vertbar => EguiKey::Pipe,
            Key::question => EguiKey::Questionmark,
            Key::bracketleft => EguiKey::OpenBracket,
            Key::braceright => EguiKey::CloseBracket,
            Key::grave => EguiKey::Backtick,
            Key::minus => EguiKey::Minus,
            Key::period => EguiKey::Period,
            Key::plus => EguiKey::Plus,
            Key::equal => EguiKey::Equals,
            Key::semicolon => EguiKey::Semicolon,
            Key::singlelowquotemark => EguiKey::Quote,
            Key::_0 | Key::KP_0 => EguiKey::Num0,
            Key::_1 | Key::KP_1 => EguiKey::Num1,
            Key::_2 | Key::KP_2 => EguiKey::Num2,
            Key::_3 | Key::KP_3 => EguiKey::Num3,
            Key::_4 | Key::KP_4 => EguiKey::Num4,
            Key::_5 | Key::KP_5 => EguiKey::Num5,
            Key::_6 | Key::KP_6 => EguiKey::Num6,
            Key::_7 | Key::KP_7 => EguiKey::Num7,
            Key::_8 | Key::KP_8 => EguiKey::Num8,
            Key::_9 | Key::KP_9 => EguiKey::Num9,
            _ => {
                if let Some(k) = key.name().and_then(|name| egui::Key::from_name(&name)) {
                    k
                } else {
                    return gtk4::glib::Propagation::Proceed;
                }
            }
        };

        key_sender
            .send(egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            })
            .unwrap();
        draw_area.queue_draw();

        gtk4::glib::Propagation::Proceed
    });

    window.add_controller(event_controller);

    window.present();
}
