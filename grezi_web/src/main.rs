use std::sync::Arc;

use arc_swap::ArcSwapOption;
use color_eyre::{config::Theme, eyre};
use eframe::{egui::mutex::Mutex, wasm_bindgen::JsCast};
use egui_glyphon::{glyphon::FontSystem, GlyphonRenderer};
use grezi_font_serde::FontSystemDeserializer;
use grezi_parser::GrzRoot;
// use tracing_error::ErrorLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;

#[derive(Clone)]
pub struct AppHandle {
    pub slideshow: Arc<ArcSwapOption<GrzRoot>>,
    pub font_system: Arc<Mutex<FontSystem>>,
    pub egui_ctx: eframe::egui::Context,
}

fn main() -> eyre::Result<()> {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions {
        // follow_system_theme: false,
        ..Default::default()
    };

    color_eyre::config::HookBuilder::new()
        .theme(Theme::default())
        .install()?;

    // tracing_subscriber::registry()
    //     .with(ErrorLayer::default())
    //     .with(
    //         #[cfg(debug_assertions)]
    //         EnvFilter::try_from_default_env()
    //             .or_else(|_| EnvFilter::try_new("warn"))
    //             .unwrap(),
    //         #[cfg(not(debug_assertions))]
    //         EnvFilter::from_default_env(),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let font_system = FontSystem::new();
    let font_system = Arc::new(Mutex::new(font_system));

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("the_canvas_id")
                    .unwrap()
                    .unchecked_into(), // hardcode it
                web_options,
                Box::new(move |cc: &eframe::CreationContext| {
                    let app_shared_data = AppHandle {
                        slideshow: Arc::new(ArcSwapOption::empty()),
                        font_system: Arc::clone(&font_system),
                        egui_ctx: cc.egui_ctx.clone(),
                    };
                    let fetch_shared_data = app_shared_data.clone();
                    ehttp::fetch(
                        ehttp::Request::get(&cc.integration_info.web_info.location.hash[1..]),
                        move |response| {
                            let res = response.unwrap();

                            let result: (FontSystemDeserializer, GrzRoot) =
                                postcard::from_bytes(&res.bytes).unwrap();
                            let mut font_system = fetch_shared_data.font_system.lock();
                            *font_system = result.0 .0;
                            fetch_shared_data.egui_ctx.set_fonts(result.0 .1);
                            fetch_shared_data.slideshow.store(Some(Arc::new(result.1)));

                            font_system.db_mut().set_sans_serif_family("Ubuntu");
                            font_system.db_mut().set_monospace_family("Fira Code");
                            font_system.db_mut().set_serif_family("DejaVu Serif");
                        },
                    );
                    if let Some(ref render_state) = cc.wgpu_render_state {
                        GlyphonRenderer::insert(render_state, Arc::clone(&font_system));
                    }
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Ok(Box::new(app::App {
                        time: 0.0,
                        slide_index: 0,
                        resolved_slide: None,
                        shared_data: app_shared_data,
                    }))
                }),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}
