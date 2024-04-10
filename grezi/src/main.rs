use grezi::MyEguiApp;
#[cfg(not(target_arch = "wasm32"))]
use std::str::FromStr;

#[cfg(not(target_arch = "wasm32"))]
use clap::builder::{StringValueParser, TypedValueParser};

use eframe::epaint::mutex::Mutex;
use egui_glyphon::glyphon::FontSystem;
use egui_glyphon::GlyphonRenderer;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct Range(std::ops::Range<usize>);

#[derive(Clone)]
#[cfg(not(target_arch = "wasm32"))]
struct RangeParser;

#[cfg(not(target_arch = "wasm32"))]
impl TypedValueParser for RangeParser {
    type Value = Range;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let parsed = StringValueParser::new().parse_ref(cmd, arg, value)?;

        Range::from_str(&parsed).map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("..");
        let start: usize = split.next().unwrap().parse().or(Err(()))?;
        let end;
        if let Some(s) = split.next() {
            end = s.parse().or(Err(()))?;
        } else {
            end = start;
        }

        Ok(Range(std::ops::Range {
            start: start.saturating_sub(1),
            end,
        }))
    }
}

#[derive(Clone, Copy)]
#[cfg(not(target_arch = "wasm32"))]
pub struct Fit(eframe::egui::Vec2);

#[derive(Clone)]
#[cfg(not(target_arch = "wasm32"))]
struct FitParser;

#[cfg(not(target_arch = "wasm32"))]
impl TypedValueParser for FitParser {
    type Value = Fit;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let parsed = StringValueParser::new().parse_ref(cmd, arg, value)?;

        Fit::from_str(&parsed).map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FromStr for Fit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('x');
        let start: f32 = split.next().unwrap().parse().or(Err(()))?;
        let fit = if let Some(s) = split.next() {
            let fit: f32 = s.parse().or(Err(()))?;

            eframe::egui::vec2(start, fit)
        } else {
            eframe::egui::vec2(start, start)
        };

        Ok(Fit(fit))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser)]
pub struct Args {
    pub presentation: Option<String>,
    #[clap(short, long)]
    pub export: bool,
    #[clap(long)]
    pub lsp: bool,
    #[clap(long)]
    pub fmt: bool,
    #[clap(short, long)]
    pub output: Option<String>,
    #[clap(short, long, value_parser = RangeParser)]
    pub index: Option<Range>,
    #[clap(short, long, value_parser = FitParser)]
    pub size: Option<Fit>,
    /// Automatically advance to the next page after the given number of seconds
    #[clap(short, long)]
    pub auto: Option<u64>,
    /// Specifies the expected run time of the presentation
    #[clap(long)]
    pub duration: Option<humantime::Duration>,
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> miette::Result<()> {
    use std::hash::Hash;
    use std::io::BufWriter;
    use std::ops::Deref;

    use eframe::egui::ViewportBuilder;
    use indexmap::IndexSet;
    use miette::{Context, IntoDiagnostic};

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let args: Args = clap::Parser::parse();
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            fullscreen: Some(!args.lsp),
            ..Default::default()
        },
        vsync: true,
        persist_window: false,
        ..Default::default()
    };
    let font_system = Arc::new(Mutex::new(FontSystem::new()));
    #[allow(unused_mut)]
    let mut app = MyEguiApp::new(args.lsp, args.presentation, Arc::clone(&font_system));
    let init_app = app.0.clone();

    if args.fmt {
        let mut file = app.0.slide_show_file.lock();
        match grezi::lsp::formatter::format_code(&app.0, &file) {
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
                    grezi::parser::ErrWithSource {
                        error,
                        source_code: file.to_string()
                    }
                );
            }
        }
        return Ok(());
    } else if args.export {
        let output = args.output.unwrap_or_else(|| "out.slideshow".to_owned());
        let used_fonts = app.0.slide_show.read().used_fonts(&mut font_system.lock());
        if output.ends_with("slideshow") {
            let mut font_system = font_system.lock();
            let mut fonts = IndexSet::new();

            struct FontRef(Arc<dyn AsRef<[u8]> + Send + Sync>);

            impl Hash for FontRef {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.0.deref().as_ref().hash(state);
                }
            }

            impl PartialEq for FontRef {
                fn eq(&self, other: &Self) -> bool {
                    self.0.deref().as_ref().eq(other.0.deref().as_ref())
                }
            }

            impl Eq for FontRef {}

            for font in used_fonts.into_iter() {
                unsafe {
                    let font = font_system.db_mut().make_shared_face_data(font).unwrap();
                    fonts.insert(FontRef(font.0));
                }
            }

            let fonts: Vec<&[u8]> = fonts.iter().map(|f| f.0.deref().as_ref()).collect();

            bincode::serialize_into(
                BufWriter::new(
                    std::fs::File::create(&output)
                        .into_diagnostic()
                        .with_context(|| format!("Error with creating {}", output))?,
                ),
                &(&fonts, &*app.0.slide_show.read()),
            )
            .into_diagnostic()
            .with_context(|| format!("Error seriaizing {}", &*init_app.slide_show_file.lock()))?;

            return Ok(());
        }

        #[cfg(feature = "cairo")]
        {
            use cairo::{ImageSurface, PdfSurface, PsSurface, SvgSurface};
            use eframe::egui;
            use eframe::epaint::{Pos2, Rect, TextureId};
            use eframe::{egui::ImageSize, epaint::Vec2};
            use std::collections::HashMap;
            use std::sync::atomic::Ordering;

            let mut multi_page = false;
            let mut image_data = Vec::new();

            let size = args.size.unwrap_or_else(|| Fit(Vec2::new(1920.0, 1080.0)));
            let fit = ImageSize {
                max_size: size.0,
                ..Default::default()
            }
            .calc_size(size.0, Vec2::new(16.0, 9.0));

            let len = init_app.slide_show.read().slide_show.len();
            let range = args.index.map(|r| r.0).unwrap_or(0..len);
            let mut path;
            let mut ctx = if output.ends_with("pdf") {
                multi_page = true;
                let surface = PdfSurface::new(fit.x as f64, fit.y as f64, &output)
                    .into_diagnostic()
                    .with_context(|| {
                        format!(
                            "Error creating pdf surface with size {:?} at {}",
                            fit, &output
                        )
                    })?;
                cairo::Context::new(&surface)
            } else if output.ends_with("ps") {
                multi_page = true;
                let surface = PsSurface::new(fit.x as f64, fit.y as f64, &output)
                    .into_diagnostic()
                    .with_context(|| {
                        format!(
                            "Error creating ps surface with size {:?} at {}",
                            fit, &output
                        )
                    })?;
                cairo::Context::new(&surface)
            } else if output.ends_with("svg") {
                let surface = SvgSurface::new(fit.x as f64, fit.y as f64, None::<&str>)
                    .into_diagnostic()
                    .with_context(|| format!("Error creating svg surface with size {:?}", fit))?;
                cairo::Context::new(&surface)
            } else {
                let stride = cairo::Format::ARgb32
                    .stride_for_width(fit.x as u32)
                    .into_diagnostic()
                    .with_context(|| {
                        format!("Error calculating ARGB stride for width: {}", fit.x as u32)
                    })?;
                image_data = vec![0; stride as usize * fit.y as usize];
                let surface = unsafe {
                    ImageSurface::create_for_data_unsafe(
                        image_data.as_mut_ptr(),
                        cairo::Format::ARgb32,
                        fit.x as i32,
                        fit.y as i32,
                        stride,
                    )
                    .into_diagnostic()
                    .with_context(|| format!("Error creating image surface with size {:?}", fit))?
                };
                cairo::Context::new(&surface)
            }
            .into_diagnostic()
            .with_context(|| "Error creating cairo surface")?;

            let egui_ctx = egui::Context::default();

            app.0.time_offset = f32::MIN;
            app.0.export = true;
            let mut init_app = app.0.init_app(&egui_ctx, app.1);
            let ft = cairo::freetype::Library::init().unwrap();

            let font_defs_ft = grezi::cairo::fonts_to_ft(Arc::clone(&font_system), &used_fonts, ft);
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, fit)),
                //pixels_per_point: Some(2.0),
                ..Default::default()
            };
            let mut textures: HashMap<TextureId, ImageSurface> = HashMap::default();
            if multi_page {
                for i in range {
                    init_app.index.store(i, Ordering::SeqCst);

                    let output = egui_ctx.run(input.clone(), |ctx| init_app.update(ctx));

                    init_app.resolved.store(None);

                    grezi::cairo::cairo_draw(output, &mut textures, &ctx, &font_defs_ft);

                    ctx.show_page().unwrap();
                }

                ctx.target().finish();
            } else {
                for i in range.clone() {
                    init_app.index.store(i, Ordering::SeqCst);
                    if image_data.is_empty() {
                        if range.len() <= 1 {
                            let surface =
                                SvgSurface::new(fit.x as f64, fit.y as f64, Some(&output))
                                    .into_diagnostic()
                                    .with_context(|| {
                                        format!(
                                            "Error creating svg surface with size {:?} at {}",
                                            fit, &output
                                        )
                                    })?;
                            ctx = cairo::Context::new(&surface).unwrap();
                        } else {
                            let o = output.rsplit_once('.').unwrap();
                            path = format!("{}_{}.{}", o.0, i + 1, o.1);
                            ctx = cairo::Context::new(
                                &SvgSurface::new(fit.x as f64, fit.y as f64, Some(&path)).unwrap(),
                            )
                            .unwrap();
                        }
                    }

                    let e_output = egui_ctx.run(input.clone(), |ctx| init_app.update(ctx));

                    grezi::cairo::cairo_draw(e_output, &mut textures, &ctx, &font_defs_ft);

                    ctx.target().finish();

                    if !image_data.is_empty() {
                        image_data.chunks_mut(4).for_each(|chunk| {
                            chunk.swap(0, 2);
                        });
                        let o = output.rsplit_once('.').unwrap();
                        let p: &str = if range.len() <= 1 {
                            &output
                        } else {
                            path = format!("{}_{}.{}", o.0, i + 1, o.1);
                            &path
                        };
                        image::save_buffer(
                            p,
                            &image_data,
                            fit.x as u32,
                            fit.y as u32,
                            image::ColorType::Rgba8,
                        )
                        .into_diagnostic()
                        .with_context(|| format!("Error saving image {} with size {:?}", p, fit))?;
                        image_data.iter_mut().for_each(|n| *n = 0);
                        let stride = cairo::Format::ARgb32
                            .stride_for_width(fit.x as u32)
                            .unwrap();
                        ctx = cairo::Context::new(unsafe {
                            &ImageSurface::create_for_data_unsafe(
                                image_data.as_mut_ptr(),
                                cairo::Format::ARgb32,
                                fit.x as i32,
                                fit.y as i32,
                                stride,
                            )
                            .unwrap()
                        })
                        .unwrap();
                    }
                }
            }

            return Ok(());
        }
    }

    eframe::run_native(
        "Grezi",
        native_options,
        Box::new(move |cc| {
            if let Some(ref render_state) = cc.wgpu_render_state {
                GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
            }
            if args.lsp {
                let lsp_egui_ctx = cc.egui_ctx.clone();
                egui_extras::install_image_loaders(&lsp_egui_ctx);
                if !lsp_egui_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                    lsp_egui_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
                }
                let current_thread = std::thread::current();
                std::thread::spawn(move || {
                    grezi::lsp::start_lsp(app.0, current_thread, lsp_egui_ctx)
                });
                std::thread::park();
            }
            Box::new(init_app.init_app(&cc.egui_ctx, app.1))
        }),
    )
    .into_diagnostic()?;

    Ok(())
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

    let font_system = Arc::new(Mutex::new(FontSystem::new()));

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(move |cc| {
                    if let Some(ref render_state) = cc.wgpu_render_state {
                        GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
                    }
                    Box::new({
                        let app = MyEguiApp::new(font_system);
                        app.0.init_app(
                            &cc.egui_ctx,
                            app.1,
                            &cc.integration_info.web_info.location.hash[1..],
                        )
                    })
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
