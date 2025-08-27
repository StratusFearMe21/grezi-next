use clap::builder::{StringValueParser, TypedValueParser};
use eframe::egui::{self, Vec2};
use grezi_egui::get_size;

include!(concat!(env!("OUT_DIR"), "/paper_sizes.rs"));

#[derive(Clone)]
pub struct RangeParser;

impl TypedValueParser for RangeParser {
    type Value = std::ops::Range<usize>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let parsed = StringValueParser::new().parse_ref(cmd, arg, value)?;

        let mut split = parsed.split("..");
        let start: usize = split
            .next()
            .unwrap()
            .parse()
            .map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))?;
        let end;
        if let Some(s) = split.next() {
            end = s
                .parse()
                .map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))?;
        } else {
            end = start;
        }

        Ok(std::ops::Range {
            start: start.saturating_sub(1),
            end,
        })
    }
}

#[derive(Clone)]
pub struct FitParser;

impl TypedValueParser for FitParser {
    type Value = egui::Vec2;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let parsed = StringValueParser::new().parse_ref(cmd, arg, value)?;
        let mut parsed = parsed.as_str();

        let keep_size = parsed.ends_with('!');

        if keep_size {
            parsed = &parsed[..parsed.len() - 1];
        }

        let fit = if let Some(paper_size) = PAPER_SIZES.get(parsed) {
            // We pretty much always want landscape.
            //
            // Cairo wants the size passed to it to be in "points"
            // which is what the unit of the paper_size map is
            Vec2::new(paper_size[1], paper_size[0])
        } else {
            let mut split = parsed.split('x');
            let start: f32 = split
                .next()
                .unwrap()
                .parse()
                .map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))?;
            let fit = if let Some(s) = split.next() {
                let fit: f32 = s
                    .parse()
                    .map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidValue))?;

                eframe::egui::vec2(start, fit)
            } else {
                eframe::egui::vec2(start, start)
            };

            if keep_size { fit } else { get_size(fit) }
        };

        Ok(fit)
    }
}
