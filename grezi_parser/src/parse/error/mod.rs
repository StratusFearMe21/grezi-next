use std::{
    fmt::{Debug, Display},
    io,
    ops::DerefMut,
    sync::{Arc, OnceLock},
};

use css_color::ParseColorErrorInner;
use highlight::GrzHighlighter;
use miette::{Diagnostic, GraphicalReportHandler};
use ropey::Rope;
use thiserror::Error;
use tracing_error::{SpanTrace, SpanTraceStatus};
use tree_house_bindings::Tree;
use tree_sitter_grz::NodeKind;

use super::cursor::ErrorInfo;

mod base16_terminal_theme;
mod highlight;

pub type Result<T> = std::result::Result<T, Arc<ErrsWithSource>>;
pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Error, Debug, Clone)]
// Putting the io::Error here makes the
// error output look better
pub struct IoError(#[from] Arc<io::Error>);

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.kind())
    }
}

#[derive(Diagnostic, Error, Debug, Clone)]
pub enum ParseError {
    #[error("A fatal error occurred")]
    IoError(#[source] IoError),
    #[error("Syntax error")]
    Syntax(#[label("{1}")] super::CharRange, &'static str),
    #[error("Failed to parse CSS color")]
    ColorSyntax {
        #[label("{error}{}", if *expected_none {
            " or `none`"
        } else {
            ""
        })]
        range: super::CharRange,
        #[source]
        error: ParseColorErrorInner,
        expected_none: bool,
    },
    #[error("Missing error")]
    Missing(#[label("{1}")] super::CharRange, &'static str),
    #[error("Not found error")]
    NotFound(#[label("{1}")] super::CharRange, &'static str),
    #[error("Viewbox error")]
    Viewbox(#[label("{1}")] super::CharRange, String),
    #[error("Bad node")]
    BadNode(
        #[label("{2}; Found a `{1:?}` here")] super::CharRange,
        NodeKind,
        &'static str,
    ),
    #[error("Long name")]
    #[diagnostic(severity(Warning))]
    #[diagnostic(help("Try making this name under 23 characters so it can be inlined"))]
    LongName(#[label("This name is pretty long")] super::CharRange),
}

impl ParseError {
    pub fn char_range_mut(&mut self) -> Option<&mut super::CharRange> {
        match self {
            Self::IoError(_) => None,
            Self::Syntax(range, _) => Some(range),
            Self::ColorSyntax { range, .. } => Some(range),
            Self::Missing(range, _) => Some(range),
            Self::NotFound(range, _) => Some(range),
            Self::Viewbox(range, _) => Some(range),
            Self::BadNode(range, _, _) => Some(range),
            Self::LongName(range) => Some(range),
        }
    }

    pub fn char_range(&self) -> Option<&super::CharRange> {
        match self {
            Self::IoError(_) => None,
            Self::Syntax(range, _) => Some(range),
            Self::ColorSyntax { range, .. } => Some(range),
            Self::Missing(range, _) => Some(range),
            Self::NotFound(range, _) => Some(range),
            Self::Viewbox(range, _) => Some(range),
            Self::BadNode(range, _, _) => Some(range),
            Self::LongName(range) => Some(range),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::IoError(IoError(Arc::new(value)))
    }
}

#[derive(Default)]
pub struct ErrsWithSource {
    pub errors: boxcar::Vec<(ParseError, SpanTrace)>,
    source_code_and_tree: OnceLock<(Rope, Tree)>,
}

impl ErrsWithSource {
    pub fn append_error(&self, error: ParseError, error_info: ErrorInfo) {
        self.errors.push((error, SpanTrace::capture()));
        let _ = self
            .source_code_and_tree
            .get_or_init(|| (error_info.source.clone(), error_info.tree.clone()));
    }

    pub fn has_errors(&self) -> bool {
        let mut has_errors = false;
        for (_, (error, _)) in &self.errors {
            if error.severity().is_none() || error.severity() == Some(miette::Severity::Error) {
                has_errors = true;
            }
        }
        has_errors
    }
}

impl Debug for ErrsWithSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = supports_color::on(supports_color::Stream::Stderr)
            .map(|c| c.has_basic)
            .unwrap_or_default();
        for (mut error, trace) in self.errors.iter().map(|(_, e)| (e.0.clone(), &e.1)) {
            let mut handler = GraphicalReportHandler::new();
            let mut source_code: Option<Arc<str>> = None;
            if let Some((source, tree)) = self.source_code_and_tree.get() {
                // Gets the single top level element that needs to be highlighted
                // to display the error. Also adjusts the CharRange of the
                // error to map to the truncated source
                let source: Arc<str> = if let Some(range) = error.char_range_mut() {
                    let mut walker = tree.walk();
                    walker.goto_first_child_for_byte(range.byte_range.start as u32);
                    if let Ok(highlight_range) =
                        super::CharRange::new(walker.node().range(), source)
                    {
                        *range = range.clone() - highlight_range.clone();
                        let byte_range = highlight_range.byte_range;
                        source
                            .byte_slice(byte_range.start as usize..byte_range.end as usize)
                            .to_string()
                            .into()
                    } else {
                        source.to_string().into()
                    }
                } else {
                    source.to_string().into()
                };
                source_code = Some(Arc::clone(&source));
                if color {
                    handler =
                        handler.with_syntax_highlighting(GrzHighlighter::new(Arc::clone(&source)));
                }
            }
            let mut report = miette::Report::new(error);
            if let Some(sc) = source_code {
                report = report.with_source_code(sc);
            }
            report.deref_mut();
            handler.render_report(f, report.as_ref())?;
            if matches!(trace.status(), SpanTraceStatus::CAPTURED) {
                write!(f, "\n{}\n\n", color_spantrace::colorize(trace))?;
            }
        }

        Ok(())
    }
}

impl From<io::Error> for ErrsWithSource {
    fn from(value: io::Error) -> Self {
        Self {
            errors: boxcar::vec![(value.into(), SpanTrace::capture())],
            source_code_and_tree: OnceLock::new(),
        }
    }
}
