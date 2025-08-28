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

pub struct GrzHighlighter {
    pub source: Arc<str>,
    pub language_loader: HelixLanguageLoader,
    pub grz_syntax: Syntax,
}

impl GrzHighlighter {
    pub fn new(source: Arc<str>) -> GrzHighlighter {
        let language_loader = HelixLanguageLoader::new(&THEME_NAMES);
        let grz_syntax = Syntax::new(
            source.deref().into(),
            language_loader
                .language_for_marker(tree_house::InjectionLanguageMarker::Name("grz"))
                .unwrap(),
            Duration::from_secs(5),
            &language_loader,
        )
        .unwrap();
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
        _source: &dyn miette::SpanContents<'_>,
    ) -> Box<dyn miette::highlighters::HighlighterState + 'h> {
        // Weird lifetimes on this trait prevent me from not doing this
        let highlighter = tree_house::highlighter::Highlighter::new(
            &self.grz_syntax,
            self.source.deref().into(),
            &self.language_loader,
            // Incoming source is already truncated
            ..,
        );
        Box::new(GrzHighlighterState {
            highlighter,
            syntax_highlight_stack: Vec::new(),
        })
    }
}

pub struct GrzHighlighterState<'h> {
    highlighter: tree_house::highlighter::Highlighter<'h, 'h, HelixLanguageLoader>,
    syntax_highlight_stack: Vec<Highlight>,
}

impl<'h> HighlighterState for GrzHighlighterState<'h> {
    #[instrument(skip(self))]
    fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>> {
        let mut out_line = Vec::new();
        let mut initial_pos = None;

        let mut pos = 0;

        while pos - *initial_pos.get_or_insert(pos) < line.len() as u32 {
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
            debug_assert!(pos > start);

            if pos < start {
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

            out_line.push(style.style(&line[start as usize..pos as usize]));
        }

        out_line
    }
}
