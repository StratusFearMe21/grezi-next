#![cfg(target_os = "android")]

use std::sync::Arc;

use color_eyre::{
    config::Theme,
    eyre::{self},
};
use eframe::{egui::mutex::Mutex, NativeOptions};
use egui_glyphon::{glyphon::FontSystem, GlyphonRenderer};
#[cfg(debug_assertions)]
use tracing_error::ErrorLayer;
#[cfg(debug_assertions)]
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
