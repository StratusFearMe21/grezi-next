use std::time::Duration;

use ropey::RopeSlice;
use tree_house::{
    InjectionLanguageMarker, LanguageLoader, Syntax,
    highlighter::{HighlightEvent, Highlighter},
};

use crate::{
    parse::{
        helix_loader::grammar::HelixLanguageLoader, slideshow::text::dark_plus_theme::THEME_NAMES,
    },
    text::{Attrs, TextParagraph},
};

use super::dark_plus_theme::THEME_COLORS;

// Copied mostly from https://github.com/helix-editor/helix/blob/7e4e556f84cd657dc99e3e0acfa7442170a01a11/helix-term/src/ui/markdown.rs#L31
pub fn format_highlighted(
    value: RopeSlice<'_>,
    language: RopeSlice<'_>,
    default_attrs: &Attrs,
    paragraph: &mut TextParagraph,
) -> bool {
    let language_loader = HelixLanguageLoader::new(&THEME_NAMES);
    let Some(initial_language) =
        language_loader.language_for_marker(InjectionLanguageMarker::Match(language))
    else {
        tracing::error!("Failed to highlight '{language}'");
        return false;
    };
    let Ok(initial_syntax) = Syntax::new(
        value.clone(),
        initial_language,
        Duration::from_secs(5),
        &language_loader,
    ) else {
        tracing::error!("Failed to highlight '{language}'");
        return false;
    };
    let mut syntax_highlighter =
        Highlighter::new(&initial_syntax, value.clone(), &language_loader, ..);
    let mut syntax_highlight_stack = Vec::new();
    let mut pos = 0;

    while pos < value.len_bytes() as u32 {
        if pos == syntax_highlighter.next_event_offset() {
            let (event, new_highlights) = syntax_highlighter.advance();
            if event == HighlightEvent::Refresh {
                syntax_highlight_stack.clear();
            }
            syntax_highlight_stack.extend(new_highlights);
        }

        let start = pos;
        pos = syntax_highlighter.next_event_offset();
        if pos == u32::MAX {
            pos = value.len_bytes() as u32;
        }
        if pos == start {
            continue;
        }
        // The highlighter should always move forward.
        // If the highlighter malfunctions, bail on syntax highlighting and log an error.
        debug_assert!(pos > start);
        if pos < start {
            tracing::error!("Failed to highlight '{language}'");
            return false;
        }

        let mut attrs = default_attrs.clone();

        syntax_highlight_stack
            .iter()
            .flat_map(|highlight| THEME_COLORS[highlight.idx()])
            .for_each(|highlight| attrs.apply_modifier(*highlight));

        let slice = value.byte_slice(start as usize..pos as usize);

        paragraph.rich_text.push((slice.chunks().collect(), attrs));
    }

    true
}
