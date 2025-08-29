use std::{ops::Deref, sync::Arc, time::Duration};

use miette::highlighters::{Highlighter, HighlighterState};
use owo_colors::{Style, Styled};
use tracing::instrument;
use tree_house::{
    LanguageLoader, Syntax,
    highlighter::{Highlight, HighlightEvent},
};

use crate::parse::{
    error::base16_terminal_theme::{THEME_COLORS, THEME_NAMES},
    helix_loader::grammar::HelixLanguageLoader,
};

#[derive(Clone)]
pub struct GrzHighlighter {
    pub source: Arc<str>,
    pub language_loader: HelixLanguageLoader,
    pub grz_syntax: Arc<Syntax>,
}

impl GrzHighlighter {
    pub fn new(source: Arc<str>) -> GrzHighlighter {
        let language_loader = HelixLanguageLoader::new(&THEME_NAMES);
        let grz_syntax = Arc::new(
            Syntax::new(
                source.deref().into(),
                language_loader
                    .language_for_marker(tree_house::InjectionLanguageMarker::Name("grz"))
                    .unwrap(),
                Duration::from_secs(5),
                &language_loader,
            )
            .unwrap(),
        );
        GrzHighlighter {
            source,
            language_loader,
            grz_syntax,
        }
    }
}

impl Highlighter for GrzHighlighter {
    fn start_highlighter_state<'h>(
        &'h self,
        source: &dyn miette::SpanContents<'_>,
    ) -> Box<dyn miette::highlighters::HighlighterState + 'h> {
        // Weird lifetimes on this trait prevent me from not doing this
        let mut highlighter = tree_house::highlighter::Highlighter::new(
            &self.grz_syntax,
            self.source.deref().into(),
            &self.language_loader,
            source.span().offset() as u32..(source.span().offset() + source.span().len()) as u32,
        );
        let mut syntax_highlight_stack = Vec::new();
        while highlighter.next_event_offset() < source.span().offset() as u32 {
            let (event, new_highlights) = highlighter.advance();
            if event == HighlightEvent::Refresh {
                syntax_highlight_stack.clear();
            }
            syntax_highlight_stack.extend(new_highlights);
        }
        let beginning_newlines = source
            .data()
            .iter()
            .take_while(|byte| **byte == b'\n' || **byte == b'\r')
            .count();
        Box::new(GrzHighlighterState {
            syntax_highlight_stack,
            initial_pos: (source.span().offset() + beginning_newlines) as u32,
            highlighter,
            // source: Arc::clone(&self.source),
        })
    }
}

pub struct GrzHighlighterState<'h> {
    highlighter: tree_house::highlighter::Highlighter<'h, 'h, HelixLanguageLoader>,
    syntax_highlight_stack: Vec<Highlight>,
    // source: Arc<str>,
    initial_pos: u32,
}

impl<'h> HighlighterState for GrzHighlighterState<'h> {
    // Copied mostly from https://github.com/helix-editor/helix/blob/7e4e556f84cd657dc99e3e0acfa7442170a01a11/helix-term/src/ui/markdown.rs#L31
    #[instrument(skip(self))]
    fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>> {
        let mut out_line = Vec::new();

        let mut pos = self.initial_pos;

        while pos - self.initial_pos < line.len() as u32 {
            if pos == self.highlighter.next_event_offset() {
                let (event, new_highlights) = self.highlighter.advance();
                if event == HighlightEvent::Refresh {
                    self.syntax_highlight_stack.clear();
                }
                self.syntax_highlight_stack.extend(new_highlights);
            }

            let start = pos;
            pos = self.highlighter.next_event_offset();
            if pos == u32::MAX {
                pos = line.len() as u32;
            }
            if pos == start {
                continue;
            }
            // The highlighter should always move forward.
            // If the highlighter malfunctions, bail on syntax highlighting and log an error.
            // tracing::info!(range = ?start..pos, source = ?self.source.get(start as usize..pos as usize));
            debug_assert!(pos > start);

            if pos < start {
                tracing::error!("Failed to highlight error");
                return vec![Style::default().style(line)];
            }

            let style = self
                .syntax_highlight_stack
                .iter()
                .map(|highlight| THEME_COLORS[highlight.idx()])
                .fold(Style::default(), |mut style, highlight| {
                    if let Some(fg) = highlight.0 {
                        style = style.color(fg);
                    }

                    if let Some(bg) = highlight.1 {
                        style = style.on_color(bg);
                    }

                    style.effects(highlight.2)
                });

            let end = (pos - self.initial_pos) as usize;
            let start = (start - self.initial_pos) as usize;
            let text_range = start..end.min(line.len());
            if let Some(text) = line.get(text_range.clone()) {
                // tracing::info!(text);
                out_line.push(style.style(text));
            } else {
                tracing::error!("Invalid text range");
            }

            if end > line.len() {
                // We are at the end of the line, and since `line`
                // doesn't contain the newline character, but our source
                // does, we skip that character by adding one to the
                // next position we start at on the next line.
                self.initial_pos = pos - (end - line.len()) as u32 + 1;
                break;
            }
        }

        out_line
    }
}
