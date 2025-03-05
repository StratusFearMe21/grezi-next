use color_eyre::{
    config::Theme,
    eyre::{self, Context, OptionExt},
};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() -> eyre::Result<()> {
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

    let path = std::env::args()
        .nth(1)
        .ok_or_eyre("First argument was not passed")?;
    let file = std::fs::File::open(&path).wrap_err("Failed to open GRZ file")?;

    let mut file = grezi_parser::parse::GrzFile::new(path, file)?;
    let parse_result = file.parse(Vec::new())?;
    eprint!("{:?}", parse_result);

    if !parse_result.has_errors() {
        dbg!(&file.slideshow);
    }

    Ok(())
}
