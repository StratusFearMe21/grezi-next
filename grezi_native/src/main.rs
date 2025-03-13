use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use args::{FitParser, RangeParser};
use clap::Parser;
use color_eyre::{
    config::Theme,
    eyre::{self, bail, Context},
};
use eframe::{
    egui::{mutex::Mutex, FontDefinitions, Vec2},
    NativeOptions,
};
use egui_glyphon::{glyphon::FontSystem, GlyphonRenderer};
use grezi_export::GrzExporter;
use grezi_file_owner::{AppHandle, FileOwnerMessage};
use grezi_font_serde::FontSystemDeserializer;
use grezi_parser::{parse::GrzFile, GrzRoot};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod args;
#[cfg(feature = "remote")]
mod remote;

#[derive(Parser)]
struct Args {
    input: Option<String>,
    #[clap(short, long)]
    export: bool,
    #[clap(short, long)]
    output: Option<PathBuf>,
    #[clap(short, long)]
    lsp: bool,
    #[clap(short, long)]
    remote: bool,
    #[clap(short, long, value_parser = RangeParser)]
    pub index: Option<std::ops::Range<usize>>,
    #[clap(short, long, value_parser = FitParser, default_value = "1920x1080")]
    pub size: Vec2,
}

impl Args {
    fn open_grz_file(&self) -> eyre::Result<(FontSystem, FontDefinitions, GrzFile)> {
        let mut font_system;
        let font_definitions;
        let slideshow;
        if self.lsp {
            font_system = FontSystem::new();
            font_definitions = FontDefinitions::default();
            slideshow = GrzFile::empty()?;
        } else {
            if let Some(input) = self.input.as_ref() {
                let file = BufReader::new(File::open(input).wrap_err("Failed to open GRZ file")?);

                let input_extension = Path::new(input).extension().and_then(|e| e.to_str());
                slideshow = if input_extension == Some("slideshow") {
                    let deserialize_instant = Instant::now();
                    let mut buffer = vec![
                        0;
                        file.get_ref()
                            .metadata()
                            .wrap_err("Failed to get input file metadata")?
                            .len() as usize
                    ];
                    let result: (FontSystemDeserializer, GrzRoot) =
                        postcard::from_io((file, &mut buffer)).unwrap().0;
                    tracing::warn!(time = ?deserialize_instant.elapsed(), "Deserializing finished");
                    font_system = result.0 .0;
                    font_definitions = result.0 .1;
                    GrzFile::wrap_root(input.clone(), result.1)
                        .wrap_err("Failed to wrap slideshow root in GrzFile")?
                } else {
                    font_system = FontSystem::new();
                    font_definitions = FontDefinitions::default();
                    let mut file = grezi_parser::parse::GrzFile::new(input.clone(), file)?;
                    let parse_result = file.parse(&[])?;
                    eprint!("{:?}", parse_result);

                    if parse_result.has_errors() {
                        bail!("Grz file contained errors");
                    }

                    file
                };
            } else {
                bail!("No input path provided, and LSP not enabled");
            }
        }

        font_system.db_mut().set_sans_serif_family("Ubuntu");
        font_system.db_mut().set_monospace_family("Fira Code");
        font_system.db_mut().set_serif_family("DejaVu Serif");

        Ok((font_system, font_definitions, slideshow))
    }
}

fn main() -> eyre::Result<()> {
    let mut args = Args::parse();
    let color = supports_color::on(supports_color::Stream::Stderr)
        .map(|c| c.has_basic)
        .unwrap_or_default();
    if !color {
        color_eyre::config::HookBuilder::new()
            .theme(Theme::default())
            .install()?;
    } else {
        color_eyre::install()?;
    }

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            #[cfg(debug_assertions)]
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    if args.lsp {
                        EnvFilter::try_new("warn")
                    } else {
                        EnvFilter::try_new("info")
                    }
                })
                .unwrap(),
            #[cfg(not(debug_assertions))]
            EnvFilter::from_default_env(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(color)
                .with_writer(io::stderr),
        )
        .init();

    let (font_system, font_definitions, slideshow) = args.open_grz_file()?;

    let font_system = Arc::new(Mutex::new(font_system));
    if args.remote && args.output.is_none() {
        args.output = args.input.and_then(|i| {
            Some(Path::new(&format!("{}.slideshow", i.rsplit_once('.')?.0)).to_path_buf())
        });
    }
    if let Some(ref output_slideshow_path) = args.output {
        GrzExporter::new(&slideshow.slideshow, Arc::clone(&font_system))
            .export(
                output_slideshow_path,
                args.size,
                args.index
                    .unwrap_or_else(|| 0..slideshow.slideshow.slides.len()),
            )
            .unwrap();
        if !args.remote {
            return Ok(());
        }
    }

    eframe::run_native(
        "Grezi V3",
        NativeOptions::default(),
        Box::new(move |cc| {
            if let Some(ref render_state) = cc.wgpu_render_state {
                GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
            }
            let (keys_tx, keys_rx) = std::sync::mpsc::channel();
            cc.egui_ctx.set_fonts(font_definitions);
            let (owner_tx, owner_rx) = crossbeam_channel::unbounded();
            owner_tx
                .send(FileOwnerMessage::Index {
                    index: 0,
                    reset_time: true,
                })
                .unwrap();
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app_shared_data = AppHandle::new(
                keys_tx,
                owner_tx,
                cc.egui_ctx.clone(),
                Arc::clone(&font_system),
            );
            #[cfg(feature = "remote")]
            if args.lsp {
                #[cfg(feature = "lsp")]
                {
                    let app_shared_data = app_shared_data.clone();
                    std::thread::spawn(move || {
                        grezi_lsp::GrzLsp::new(app_shared_data, owner_rx).run()
                    });
                }
                #[cfg(not(feature = "lsp"))]
                {
                    color_eyre::eyre::bail!("LSP feature is not enabled");
                }
            } else {
                let t_app_shared_data = app_shared_data.clone();
                std::thread::spawn(move || {
                    remote::Remote {
                        app_handle: t_app_shared_data,
                        cached_slideshow_file: args
                            .output
                            .as_ref()
                            .map(|p| p.as_path())
                            .unwrap_or_else(|| Path::new(""))
                            .into(),
                    }
                    .run()
                });
                let t_app_shared_data = app_shared_data.clone();
                std::thread::spawn(move || {
                    grezi_file_owner::DefaultOwner {
                        root: slideshow,
                        message_receiver: owner_rx,
                        shared_data: t_app_shared_data,
                        slide_index: 0,
                    }
                    .run()
                });
            }
            Ok(Box::new(app::App {
                time: 0.0,
                clip: false,
                first_pointer_pos: None,
                custom_key_events: keys_rx,
                shared_data: app_shared_data,
            }))
        }),
    )
    .unwrap();

    Ok(())
}
