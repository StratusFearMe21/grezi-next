use std::str::FromStr;

use clap::builder::{StringValueParser, TypedValueParser};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct Range(pub std::ops::Range<usize>);

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
pub struct Fit(pub eframe::egui::Vec2);

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
    /// For compatibility, if your system is wacky
    #[clap(short, long)]
    pub gtk: bool,
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
