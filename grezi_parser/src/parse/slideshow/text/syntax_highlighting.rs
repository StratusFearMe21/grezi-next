use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent};

use crate::{
    parse::helix_loader,
    text::{Attrs, Modifier, TextParagraph},
};

use super::dark_plus_theme::THEME_COLORS;

macro_rules! get_highlight_config {
    ($language:expr) => {
        HELIX_HIGHLIGHT_CONFIGS.with_borrow_mut(|configs| {
            Rc::clone(configs.entry($language.into()).or_insert_with(move || {
                let mut config = match helix_loader::grammar::get_highlight_configuration($language)
                {
                    Ok(config) => config,
                    Err(e) => return Rc::new(Err(e)),
                };
                config.configure(super::dark_plus_theme::THEME_NAMES);
                Rc::new(Ok(config))
            }))
        })
    };
}

thread_local! {
    static HELIX_HIGHLIGHT_CONFIGS: RefCell<HashMap<smartstring::alias::String, Rc<Result<HighlightConfiguration, libloading::Error>>, ahash::RandomState>> = RefCell::new(HashMap::default());
}

pub fn format_highlighted(
    value: &str,
    language: &str,
    default_attrs: &Attrs,
    paragraph: &mut TextParagraph,
) -> bool {
    let mut highlighter = tree_sitter_highlight::Highlighter::new();
    let initial_config = unsafe {
        std::mem::transmute::<
            &Result<HighlightConfiguration, libloading::Error>,
            &Result<HighlightConfiguration, libloading::Error>,
        >(get_highlight_config!(language).deref())
    };
    let Ok(initial_config) = initial_config else {
        return false;
    };
    let Ok(highlight_iter) =
        highlighter.highlight(initial_config, value.as_bytes(), None, |name| {
            if let Ok(config) = get_highlight_config!(name).deref() {
                Some(unsafe {
                    std::mem::transmute::<
                        &tree_sitter_highlight::HighlightConfiguration,
                        &tree_sitter_highlight::HighlightConfiguration,
                    >(config)
                })
            } else {
                None
            }
        })
    else {
        return false;
    };

    let mut modifiers_active = Vec::new();

    for highlight in highlight_iter {
        let Ok(highlight_event) = highlight else {
            return false;
        };

        match highlight_event {
            HighlightEvent::Source { start, end } => {
                let mut attrs = default_attrs.clone();

                for modifier in modifiers_active
                    .iter()
                    .flat_map(|mods: &&[Modifier]| mods.iter())
                {
                    attrs.apply_modifier(*modifier);
                }

                paragraph
                    .rich_text
                    .push(((&value[start..end]).into(), attrs));
            }
            HighlightEvent::HighlightEnd => {
                modifiers_active.pop();
            }
            HighlightEvent::HighlightStart(highlight) => {
                modifiers_active.push(THEME_COLORS[highlight.0]);
            }
        }
    }

    true
}
