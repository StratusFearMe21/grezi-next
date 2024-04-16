use std::{
    borrow::Cow,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    sync::Arc,
};

use arc_swap::ArcSwap;
use egui_glyphon::glyphon::{Attrs, AttrsOwned, Color, Metrics, Style, Weight};
use helix_core::tree_sitter::Node;
use helix_core::{
    ropey::{Rope, RopeBuilder},
    tree_sitter::Range,
};
use helix_core::{
    syntax::{HighlightConfiguration, HighlightEvent, InjectionLanguageMarker},
    Syntax,
};
use helix_view::theme::Modifier;

use super::{
    objects::cosmic_jotdown::{JotdownItem, RichText},
    GrzCursor, NodeKind, PassThroughHasher,
};

#[derive(Clone)]
pub struct HelixCell {
    theme: helix_view::Theme,
    text_style: helix_view::theme::Style,
    loader: Arc<ArcSwap<helix_core::syntax::Loader>>,
    loaded_syntaxes: std::collections::HashMap<
        u64,
        Arc<HighlightConfiguration>,
        BuildHasherDefault<PassThroughHasher>,
    >,
}

pub fn highlight_text(
    text: Node<'_>,
    lang: (Cow<'_, str>, Range),
    font_id: Attrs<'_>,
    helix_cell: &mut Option<HelixCell>,
    source: &Rope,
    hasher: &ahash::RandomState,
) -> Result<JotdownItem, super::Error> {
    let mut job = Vec::new();
    let helix = helix_cell.get_or_insert_with(|| {
        let mut theme_parent_dirs = vec![helix_loader::config_dir()];
        theme_parent_dirs.extend(helix_loader::runtime_dirs().iter().cloned());
        let theme_loader = Arc::new(helix_view::theme::Loader::new(&theme_parent_dirs));

        let theme = theme_loader.load("dark_plus").unwrap();

        let text_style = theme.get("ui.text");
        let syn_loader_conf = helix_core::config::user_lang_config().unwrap();
        let loader = Arc::new(ArcSwap::new(Arc::new(
            helix_core::syntax::Loader::new(syn_loader_conf).unwrap(),
        )));

        HelixCell {
            theme,
            text_style,
            loader,
            loaded_syntaxes: std::collections::HashMap::default(),
        }
    });

    let mut rope = RopeBuilder::new();

    let mut walker = GrzCursor::from_node(text, source);
    walker.goto_first_child()?;

    let mut slice: Cow<'_, str>;
    loop {
        match NodeKind::from(walker.node().kind_id()) {
            NodeKind::StringContent | NodeKind::RawStringContent => rope.append({
                slice = source.byte_slice(walker.node().byte_range()).into();
                slice.as_ref()
            }),
            NodeKind::EscapeSequence => rope.append({
                slice = source
                    .byte_slice(
                        walker.node().byte_range().start + 1..walker.node().byte_range().end,
                    )
                    .into();
                slice.as_ref()
            }),
            _ => break,
        }
        walker.goto_next_sibling()?;
    }

    let rope = rope.finish();

    let hash = {
        let mut hasher = hasher.build_hasher();
        std::hash::Hash::hash(&lang, &mut hasher);
        hasher.finish()
    };
    let highlight_config = helix.loaded_syntaxes.entry(hash).or_insert_with(|| {
        if let Some(highlight) = helix
            .loader
            .load()
            .language_configuration_for_injection_string(&InjectionLanguageMarker::Name(lang.0))
            .and_then(|config| config.highlight_config(helix.theme.scopes()))
        {
            highlight
        } else {
            helix
                .loader
                .load()
                .language_configuration_for_injection_string(&InjectionLanguageMarker::Name(
                    "markdown".into(),
                ))
                .and_then(|config| config.highlight_config(helix.theme.scopes()))
                .unwrap()
        }
    });

    let syntax = Syntax::new(
        rope.slice(..),
        Arc::clone(highlight_config),
        Arc::clone(&helix.loader),
    )
    .unwrap();

    let highlight_iter = syntax
        .highlight_iter(rope.slice(..), None, None)
        .map(|e| e.unwrap());
    let mut highlights = Vec::new();

    for event in highlight_iter {
        match event {
            HighlightEvent::HighlightStart(span) => {
                highlights.push(span);
            }
            HighlightEvent::HighlightEnd => {
                highlights.pop();
            }
            HighlightEvent::Source { start, end } => {
                let style = highlights.iter().fold(helix.text_style, |acc, span| {
                    acc.patch(helix.theme.highlight(span.0))
                });

                let helix_view::theme::Color::Rgb(f_r, f_g, f_b) = style.fg.unwrap() else {
                    todo!()
                };

                let slice = rope.slice(start..end);

                for line in slice.lines() {
                    if line.len_bytes() != 0 {
                        job.push(RichText(
                            line.to_string(),
                            AttrsOwned::new({
                                let mut attrs = font_id.color(Color::rgb(f_r, f_g, f_b));
                                if style.add_modifier.contains(Modifier::BOLD) {
                                    attrs = attrs.weight(Weight::BOLD);
                                }
                                if style.add_modifier.contains(Modifier::ITALIC) {
                                    attrs = attrs.style(Style::Italic);
                                }
                                attrs
                            }),
                        ));
                    }
                }
            }
        }
    }
    Ok(JotdownItem {
        indent: super::objects::cosmic_jotdown::Indent {
            modifier: None,
            indent: 0.0,
        },
        buffer: job,
        metrics: Metrics::new(1.0, 1.0),
        margin: 0.0,
        url_map: None,
    })
}
