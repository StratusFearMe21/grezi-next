use std::{collections::HashMap, iter::Peekable, marker::PhantomData, sync::Arc};

use miette::highlighters::{Highlighter, HighlighterState};
use owo_colors::{Style, Styled};
use parking_lot::{Mutex, MutexGuard};
use tracing::instrument;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent};

use crate::parse::helix_loader;

pub struct GrzHighlighter {
    pub source: Arc<str>,
    pub grz_config: HighlightConfiguration,
    pub other_configs:
        Mutex<HashMap<&'static str, Result<HighlightConfiguration, libloading::Error>>>,
    pub highlighter: Mutex<tree_sitter_highlight::Highlighter>,
}

impl GrzHighlighter {
    pub fn new(source: Arc<str>) -> GrzHighlighter {
        let mut grz_config = HighlightConfiguration::new(
            tree_sitter_grz::LANGUAGE.into(),
            "grz",
            tree_sitter_grz::HIGHLIGHTS_QUERY,
            tree_sitter_grz::INJECTIONS_QUERY,
            "",
        )
        .unwrap();
        grz_config.configure(super::base16_terminal_theme::THEME_NAMES);
        GrzHighlighter {
            source,
            grz_config,
            other_configs: Mutex::new(HashMap::new()),
            highlighter: Mutex::new(tree_sitter_highlight::Highlighter::new()),
        }
    }
}

impl Highlighter for GrzHighlighter {
    fn start_highlighter_state<'h>(
        &'h self,
        source: &dyn miette::SpanContents<'_>,
    ) -> Box<dyn miette::highlighters::HighlighterState + 'h> {
        // Weird lifetimes on this trait prevent me from not doing this
        let highlighter = MutexGuard::leak(self.highlighter.lock());
        let other_configs = MutexGuard::leak(self.other_configs.lock());
        let mut highlighter = highlighter
            .highlight(&self.grz_config, self.source.as_bytes(), None, |name| {
                if let Ok(lang) = {
                    // let mut other_configs = self.other_configs.lock();
                    other_configs
                        .entry(
                            // SAFETY: Name is stored in an `Arc<str>`
                            // that won't be dropped until the highlight
                            // finishes
                            unsafe { std::mem::transmute::<&str, &'static str>(name) },
                        )
                        .or_insert_with(|| {
                            let mut language_config =
                                helix_loader::grammar::get_highlight_configuration(name)?;
                            language_config.configure(super::base16_terminal_theme::THEME_NAMES);

                            Ok(language_config)
                        })
                } {
                    Some(unsafe {
                        std::mem::transmute::<
                            &mut tree_sitter_highlight::HighlightConfiguration,
                            &tree_sitter_highlight::HighlightConfiguration,
                        >(lang)
                    })
                } else {
                    None
                }
            })
            .unwrap()
            .peekable();
        let mut highlights: Vec<Highlight> = Vec::new();
        let mut start_at = None;
        while let Some(event) = highlighter.peek() {
            match *event.as_ref().unwrap() {
                HighlightEvent::Source { start, end } => {
                    if (start..end).contains(&source.span().offset()) {
                        start_at = Some(source.span().offset());
                        break;
                    }
                }
                HighlightEvent::HighlightStart(h) => highlights.push(h),
                HighlightEvent::HighlightEnd => {
                    highlights.pop();
                }
            }

            highlighter.next();
        }
        Box::new(GrzHighlighterState {
            highlights,
            highlighter,
            source: Arc::clone(&self.source),
            start_at,
            _marker: &PhantomData,
        })
    }
}

pub struct GrzHighlighterState<
    'a,
    T: Iterator<Item = Result<HighlightEvent, tree_sitter_highlight::Error>> + 'a,
> {
    highlights: Vec<Highlight>,
    highlighter: Peekable<T>,
    source: Arc<str>,
    start_at: Option<usize>,
    _marker: &'a PhantomData<T>,
}

impl<'a, T: Iterator<Item = Result<HighlightEvent, tree_sitter_highlight::Error>> + 'a>
    HighlighterState for GrzHighlighterState<'a, T>
{
    #[instrument(skip(self))]
    fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>> {
        let mut out_line = Vec::new();
        let mut len: usize = 0;
        while let Some(event) = self.highlighter.peek() {
            match *event.as_ref().unwrap() {
                HighlightEvent::Source { start, end } => {
                    let mut style = Style::new();

                    for highlight in self.highlights.iter() {
                        let theme_style = super::base16_terminal_theme::THEME_COLORS[highlight.0];

                        if let Some(fg) = theme_style.0 {
                            style = style.color(fg);
                        }

                        if let Some(bg) = theme_style.1 {
                            style = style.on_color(bg);
                        }

                        style = style.effects(theme_style.2);
                    }

                    let start = if let Some(start_at) = self.start_at {
                        start_at
                    } else {
                        start
                    };
                    if start < end {
                        len += (start..end).len();
                        if let Some(leftover) = len.checked_sub(line.len()) {
                            let range = start..end - leftover;
                            let source = unsafe {
                                std::mem::transmute::<&str, &'s str>(
                                    self.source.get(range.clone()).unwrap(),
                                )
                            };
                            out_line.push(style.style(source));
                            let line_chars = &self.source[end - leftover..]
                                .chars()
                                .take_while(|c| *c == '\n' || *c == '\r')
                                .count();
                            self.start_at = Some(range.end + line_chars);
                            break;
                        } else {
                            let source = unsafe {
                                std::mem::transmute::<&str, &'s str>(
                                    self.source.get(start..end).unwrap(),
                                )
                            };
                            out_line.push(style.style(source));
                            self.start_at = None;
                        }
                    }
                }
                HighlightEvent::HighlightStart(h) => self.highlights.push(h),
                HighlightEvent::HighlightEnd => {
                    self.highlights.pop();
                }
            }

            self.highlighter.next();
        }
        out_line
    }
}
