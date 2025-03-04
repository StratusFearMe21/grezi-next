// #![cfg(target_os = "android")]

use std::{sync::Arc, time::Instant};

use arc_swap::ArcSwapOption;
use color_eyre::{
    config::Theme,
    eyre::{self},
};
use eframe::{
    egui::{mutex::Mutex, Rect},
    NativeOptions,
};
use egui_glyphon::{glyphon::FontSystem, GlyphonRenderer};
use grezi_font_serde::FontSystemDeserializer;
use grezi_parser::GrzRoot;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) -> eyre::Result<()> {
    color_eyre::config::HookBuilder::new()
        .theme(Theme::default())
        .install()?;

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tracing_panic::panic_hook(panic_info);
        prev_hook(panic_info);
    }));

    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(tracing_android::layer("com.grezi").unwrap())
        .init();

    // let mut font_system;
    // let font_definitions;
    // let slideshow = {
    //     let deserialize_instant = Instant::now();
    //     let mut buffer = vec![0; SLIDESHOW_FILE.len()];
    //     let result: (FontSystemDeserializer, GrzRoot) =
    //         postcard::from_bytes(SLIDESHOW_FILE).unwrap();
    //     tracing::warn!(time = ?deserialize_instant.elapsed(), "Deserializing finished");
    //     font_system = result.0 .0;
    //     font_definitions = result.0 .1;
    //     result.1
    // };

    // font_system.db_mut().set_sans_serif_family("Ubuntu");
    // font_system.db_mut().set_monospace_family("Fira Code");
    // font_system.db_mut().set_serif_family("DejaVu Serif");

    eframe::run_native(
        "Grezi V3",
        NativeOptions {
            android_app: Some(app.clone()),
            ..Default::default()
        },
        Box::new(move |cc| {
            let font_system = Arc::new(Mutex::new(FontSystem::new()));
            if let Some(ref render_state) = cc.wgpu_render_state {
                GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
            }
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::App::Connection {
                ip: String::new(),
                font_system,
            }))
        }),
    )
    .unwrap();

    Ok(())
}
