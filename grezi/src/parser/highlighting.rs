use std::{
    borrow::Cow,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    sync::Arc,
};

use eframe::{
    egui::TextFormat,
    emath::Align,
    epaint::{
        text::{LayoutJob, TextWrapping},
        Color32, FontId, Stroke,
    },
};
use helix_core::tree_sitter::Node;
use helix_core::{
    ropey::{Rope, RopeBuilder},
    tree_sitter::Range,
};
use helix_core::{
    syntax::{HighlightConfiguration, HighlightEvent, InjectionLanguageMarker},
    Syntax,
};
use helix_view::theme::Color;
use helix_view::theme::Modifier;

use super::{GrzCursor, NodeKind, PassThroughHasher};

#[derive(Clone)]
pub struct HelixCell {
    theme: helix_view::Theme,
    text_style: helix_view::theme::Style,
    loader: Arc<helix_core::syntax::Loader>,
    loaded_syntaxes: std::collections::HashMap<
        u64,
        Arc<HighlightConfiguration>,
        BuildHasherDefault<PassThroughHasher>,
    >,
}

pub fn highlight_text(
    text: Node<'_>,
    lang: (Cow<'_, str>, Range),
    align: Align,
    font_id: FontId,
    helix_cell: &mut Option<HelixCell>,
    source: &Rope,
    hasher: &ahash::RandomState,
) -> Result<LayoutJob, super::Error> {
    let mut layout_job = LayoutJob {
        halign: align,
        break_on_newline: true,
        wrap: TextWrapping {
            max_width: 0.0,
            max_rows: u32::MAX as usize,
            ..Default::default()
        },
        ..Default::default()
    };
    let helix = helix_cell.get_or_insert_with(|| {
        let mut theme_parent_dirs = vec![helix_loader::config_dir()];
        theme_parent_dirs.extend(helix_loader::runtime_dirs().iter().cloned());
        let theme_loader = Arc::new(helix_view::theme::Loader::new(&theme_parent_dirs));

        let theme = theme_loader.load("dark_plus").unwrap();

        let text_style = theme.get("ui.text");
        let syn_loader_conf = helix_core::config::user_syntax_loader().unwrap();
        let loader = Arc::new(helix_core::syntax::Loader::new(syn_loader_conf));

        HelixCell {
            theme,
            text_style,
            loader,
            loaded_syntaxes: std::collections::HashMap::default(),
        }
    });

    let mut rope = RopeBuilder::new();

    let mut walker = GrzCursor::from_node(text);
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
            .language_configuration_for_injection_string(&InjectionLanguageMarker::Name(lang.0))
            .and_then(|config| config.highlight_config(helix.theme.scopes()))
        {
            highlight
        } else {
            helix
                .loader
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

                let Color::Rgb(f_r, f_g, f_b) = style.fg.unwrap() else {
                    todo!()
                };

                let slice = rope.slice(start..end);

                for line in slice.lines() {
                    let line_str: std::borrow::Cow<'_, str> = line.into();
                    if !line_str.is_empty() {
                        layout_job.append(
                            line_str.as_ref(),
                            0.0,
                            TextFormat {
                                font_id: font_id.clone(),
                                color: Color32::from_rgb(f_r, f_g, f_b),
                                background: Color32::TRANSPARENT,
                                italics: style.add_modifier.contains(Modifier::ITALIC),
                                underline: if style.underline_style.is_some() {
                                    let Color::Rgb(u_r, u_g, u_b) = style.underline_color.unwrap()
                                    else {
                                        unreachable!()
                                    };
                                    Stroke::new(3.0, Color32::from_rgb(u_r, u_g, u_b))
                                } else {
                                    Stroke::NONE
                                },
                                strikethrough: if style.add_modifier.contains(Modifier::CROSSED_OUT)
                                {
                                    Stroke::new(3.0, Color32::from_rgb(f_r, f_g, f_b))
                                } else {
                                    Stroke::NONE
                                },
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }
    Ok(layout_job)
}
