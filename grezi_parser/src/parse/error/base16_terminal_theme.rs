use owo_colors::{Effect, XtermColors};
use phf::OrderedSet;

pub const THEME_NAMES: OrderedSet<&'static str> = phf::phf_ordered_set! {
    "ui.menu",
    "ui.menu.selected",
    "ui.linenr",
    "ui.popup",
    "ui.window",
    "ui.linenr.selected",
    "ui.selection",
    "comment",
    "ui.statusline",
    "ui.statusline.inactive",
    "ui.help",
    "ui.cursor",
    "ui.cursor.primary",
    "ui.virtual.whitespace",
    "ui.virtual.jump-label",
    "ui.virtual.ruler",
    "variable",
    "constant.numeric",
    "constant",
    "attribute",
    "type",
    "ui.cursor.match",
    "string",
    "variable.other.member",
    "constant.character.escape",
    "function",
    "constructor",
    "special",
    "keyword",
    "label",
    "namespace",
    "markup.heading",
    "markup.list",
    "markup.bold",
    "markup.italic",
    "markup.strikethrough",
    "markup.link.url",
    "markup.link.text",
    "markup.quote",
    "markup.raw",
    "diff.plus",
    "diff.delta",
    "diff.minus",
    "diagnostic",
    "ui.gutter",
    "info",
    "hint",
    "debug",
    "warning",
    "error",
};

pub const THEME_COLORS: &[(Option<XtermColors>, Option<XtermColors>, &[Effect])] = &[
    // "ui.menu"
    (Some(XtermColors::LightGray), Some(XtermColors::Gray), &[]),
    // "ui.menu.selected"
    (None, None, &[Effect::Reversed]),
    // "ui.linenr"
    (
        Some(XtermColors::LightGray),
        Some(XtermColors::UserBlack),
        &[],
    ),
    // "ui.popup"
    (None, Some(XtermColors::UserBlack), &[]),
    // "ui.window"
    (None, Some(XtermColors::UserBlack), &[]),
    // "ui.linenr.selected"
    (
        Some(XtermColors::UserWhite),
        Some(XtermColors::UserBlack),
        &[Effect::Bold],
    ),
    // "ui.selection"
    (Some(XtermColors::Gray), None, &[Effect::Reversed]),
    // "comment"
    (Some(XtermColors::Gray), None, &[Effect::Italic]),
    // "ui.statusline"
    (
        Some(XtermColors::UserWhite),
        Some(XtermColors::UserBlack),
        &[],
    ),
    // "ui.statusline.inactive"
    (Some(XtermColors::Gray), Some(XtermColors::UserBlack), &[]),
    // "ui.help"
    (
        Some(XtermColors::UserWhite),
        Some(XtermColors::UserBlack),
        &[],
    ),
    // "ui.cursor"
    (Some(XtermColors::LightGray), None, &[Effect::Reversed]),
    // "ui.cursor.primary"
    (Some(XtermColors::LightGray), None, &[Effect::Reversed]),
    // "ui.virtual.whitespace"
    (Some(XtermColors::LightGray), None, &[]),
    // = "light-gray"
    // "ui.virtual.jump-label"
    (
        Some(XtermColors::UserBlue),
        None,
        &[Effect::Bold, Effect::Underline],
    ),
    // "ui.virtual.ruler"
    (None, Some(XtermColors::UserBlack), &[]),
    // "variable"
    (Some(XtermColors::UserBrightRed), None, &[]),
    // "constant.numeric"
    (Some(XtermColors::UserYellow), None, &[]),
    // "constant"
    (Some(XtermColors::UserYellow), None, &[]),
    // "attribute"
    (Some(XtermColors::UserYellow), None, &[]),
    // "type"
    (Some(XtermColors::UserBrightYellow), None, &[]),
    // "ui.cursor.match"
    (
        Some(XtermColors::UserBrightYellow),
        None,
        &[Effect::Underline],
    ),
    // "string"
    (Some(XtermColors::UserBrightGreen), None, &[]),
    // "variable.other.member"
    (Some(XtermColors::UserBrightGreen), None, &[]),
    // "constant.character.escape"
    (Some(XtermColors::UserBrightCyan), None, &[]),
    // "function"
    (Some(XtermColors::UserBrightBlue), None, &[]),
    // "constructor"
    (Some(XtermColors::UserBrightBlue), None, &[]),
    // "special"
    (Some(XtermColors::UserBrightBlue), None, &[]),
    // "keyword"
    (Some(XtermColors::UserBrightMagenta), None, &[]),
    // "label"
    (Some(XtermColors::UserBrightMagenta), None, &[]),
    // "namespace"
    (Some(XtermColors::UserBrightMagenta), None, &[]),
    // "markup.heading"
    (Some(XtermColors::UserBrightBlue), None, &[]),
    // "markup.list"
    (Some(XtermColors::UserBrightRed), None, &[]),
    // "markup.bold"
    (Some(XtermColors::UserBrightYellow), None, &[Effect::Bold]),
    // "markup.italic"
    (
        Some(XtermColors::UserBrightMagenta),
        None,
        &[Effect::Italic],
    ),
    // "markup.strikethrough"
    (None, None, &[Effect::Strikethrough]),
    // "markup.link.url"
    (Some(XtermColors::UserYellow), None, &[Effect::Underline]),
    // "markup.link.text"
    (Some(XtermColors::UserBrightRed), None, &[]),
    // "markup.quote"
    (Some(XtermColors::UserBrightCyan), None, &[]),
    // "markup.raw"
    (Some(XtermColors::UserBrightGreen), None, &[]),
    // "diff.plus"
    (Some(XtermColors::UserBrightGreen), None, &[]),
    // "diff.delta"
    (Some(XtermColors::UserYellow), None, &[]),
    // "diff.minus"
    (Some(XtermColors::UserBrightRed), None, &[]),
    // "diagnostic"
    (None, None, &[Effect::Underline]),
    // "ui.gutter"
    (None, Some(XtermColors::UserBlack), &[]),
    // "info"
    (Some(XtermColors::UserBrightBlue), None, &[]),
    // "hint"
    (Some(XtermColors::Gray), None, &[]),
    // "debug"
    (Some(XtermColors::Gray), None, &[]),
    // "warning"
    (Some(XtermColors::UserYellow), None, &[]),
    // "error"
    (Some(XtermColors::UserBrightRed), None, &[]),
];
