use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
    sync::Arc,
    time::Instant,
};

use app::AppHandle;
use clap::Parser;
use color_eyre::{
    config::Theme,
    eyre::{self, Context, OptionExt},
};
use eframe::{
    egui::{self, ahash, mutex::Mutex, FontDefinitions, Rect},
    NativeOptions,
};
use egui_glyphon::{glyphon::FontSystem, GlyphonRenderer};
use grezi_egui::GrzResolvedSlide;
use grezi_font_serde::{FontRef, FontSystemDeserializer, IndexSliceSerializer};
use grezi_parser::{parse::GrzFile, slide::BASE_SIZE, GrzRoot};
use indexmap::IndexSet;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
#[cfg(feature = "remote")]
mod remote;

#[derive(Parser)]
struct Args {
    input: PathBuf,
    #[clap(short, long)]
    export: bool,
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
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
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
            #[cfg(not(debug_assertions))]
            EnvFilter::from_default_env(),
        )
        .with(tracing_subscriber::fmt::layer().with_ansi(color))
        .init();

    let file =
        BufReader::new(File::open(args.input.as_path()).wrap_err("Failed to open GRZ file")?);

    let input_extension = args.input.extension().and_then(|e| e.to_str());
    let mut font_system;
    let font_definitions;
    let slideshow = if input_extension == Some("slideshow") {
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
        GrzFile::wrap_root(args.input.clone(), result.1)
            .wrap_err("Failed to wrap slideshow root in GrzFile")?
    } else {
        font_system = FontSystem::new();
        font_definitions = FontDefinitions::default();
        let mut file = grezi_parser::parse::GrzFile::new(args.input.clone(), file)?;
        let parse_result = file.parse()?;
        eprint!("{:?}", parse_result);

        if !parse_result.has_errors() {
            dbg!(&file.slideshow);
        } else {
            return Ok(());
        }

        file
    };

    font_system.db_mut().set_sans_serif_family("Ubuntu");
    font_system.db_mut().set_monospace_family("Fira Code");
    font_system.db_mut().set_serif_family("DejaVu Serif");

    let output_slideshow_path = args
        .output
        .clone()
        .unwrap_or_else(|| args.input.parent().unwrap().join("__cache.slideshow"));
    if args.output.is_some() || input_extension != Some("slideshow") {
        let mut output_file = BufWriter::new(
            File::create(output_slideshow_path.as_path()).wrap_err("Failed to open output file")?,
        );
        let egui_ctx = egui::Context::default();
        egui_extras::install_image_loaders(&egui_ctx);
        let mut all_fonts_used: IndexSet<FontRef, ahash::RandomState> = IndexSet::default();
        for i in 0..slideshow.slideshow.slides.len() {
            let slide = GrzResolvedSlide::resolve_slide(
                &slideshow.slideshow,
                BASE_SIZE,
                &mut font_system,
                &egui_ctx,
                i,
            )
            .ok_or_eyre("Failed to resolve slide during serialization")?;

            let fonts = slide.fonts_used();

            for font in fonts {
                all_fonts_used.insert(FontRef(unsafe {
                    font_system.db_mut().make_shared_face_data(font).unwrap().0
                }));
            }
        }
        postcard::to_io(
            &(
                IndexSliceSerializer(all_fonts_used.as_slice()),
                &slideshow.slideshow,
            ),
            &mut output_file,
        )
        .wrap_err("Failed to write slideshow in binary format to file")?;
        output_file
            .flush()
            .wrap_err("Failed to flush output file")?;
        if args.output.is_some() {
            return Ok(());
        }
    } else if input_extension == Some("slideshow") {
        std::fs::hard_link(args.input.as_path(), output_slideshow_path.as_path())
            .wrap_err("Failed to link slideshow to cache path")?;
    }

    eframe::run_native(
        "Grezi V3",
        NativeOptions::default(),
        Box::new(move |cc| {
            let font_system = Arc::new(Mutex::new(font_system));
            if let Some(ref render_state) = cc.wgpu_render_state {
                GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
            }
            let (keys_tx, keys_rx) = std::sync::mpsc::channel();
            cc.egui_ctx.set_fonts(font_definitions);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app_shared_data =
                AppHandle::new(keys_tx, cc.egui_ctx.clone(), Arc::clone(&font_system));
            #[cfg(feature = "remote")]
            {
                let app_shared_data = app_shared_data.clone();
                std::thread::spawn(move || {
                    remote::Remote {
                        app_handle: app_shared_data,
                        cached_slideshow_file: output_slideshow_path.into(),
                    }
                    .run()
                });
            }
            Ok(Box::new(app::App {
                time: 0.0,
                max_rect: Rect::ZERO,
                first_pointer_pos: None,
                custom_key_events: keys_rx,
                shared_data: app_shared_data,
            }))
        }),
    )
    .wrap_err("Failed to run eframe application")
}
