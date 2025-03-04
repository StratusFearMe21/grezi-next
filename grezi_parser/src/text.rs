use std::sync::Arc;

use ecolor::Color32;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smart_default::SmartDefault;

pub type TextJob = Vec<TextSection>;

#[derive(SmartDefault, Serialize, Deserialize, Debug)]
pub enum TextSection {
    #[default]
    Paragraph(TextParagraph),
    Blockquote(TextJob),
    List(Vec<(TextParagraph, TextJob)>),
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct TextParagraph {
    pub rich_text: SmallVec<[(smartstring::alias::String, Attrs); 1]>,
    pub font_size: f32,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct Attrs {
    #[default(Color32::WHITE)]
    pub color: Color32,
    pub family: Family,
    pub stretch: Stretch,
    pub style: Style,
    pub weight: Weight,
}

impl Attrs {
    pub fn apply_fontstr(&mut self, family: &str) {
        let mut split = family.split(':');
        let base = split.next().unwrap_or_default();

        match base {
            "serif" => self.family = Family::Serif,
            "sans-serif" => self.family = Family::SansSerif,
            "cursive" => self.family = Family::Cursive,
            "fantasy" => self.family = Family::Fantasy,
            "monospace" => self.family = Family::Monospace,
            name => self.family = Family::Name(name.to_owned().into()),
        }

        for s in split {
            match s {
                "normal" => self.style = Style::Normal,
                "italic" => self.style = Style::Italic,
                "oblique" => self.style = Style::Oblique,
                // Thin weight (100), the thinnest value.
                "thin" => self.weight = Weight::THIN,
                // Extra light weight (200).
                "extra_light" => self.weight = Weight::EXTRA_LIGHT,
                // Light weight (300).
                "light" => self.weight = Weight::LIGHT,
                // Normal (400).
                // "normal" => self.weight = Weight::NORMAL,
                // Medium weight (500, higher than normal).
                "medium" => self.weight = Weight::MEDIUM,
                // Semibold weight (600).
                "semibold" => self.weight = Weight::SEMIBOLD,
                // Bold weight (700).
                "bold" => self.weight = Weight::BOLD,
                // Extra-bold weight (800).
                "extra_bold" => self.weight = Weight::EXTRA_BOLD,
                // Black weight (900), the thickest value.
                "black" => self.weight = Weight::BLACK,
                "ultracondensed" => self.stretch = Stretch::UltraCondensed,
                "extracondensed" => self.stretch = Stretch::ExtraCondensed,
                "condensed" => self.stretch = Stretch::Condensed,
                "semicondensed" => self.stretch = Stretch::SemiCondensed,
                "semiexpanded" => self.stretch = Stretch::SemiExpanded,
                "expanded" => self.stretch = Stretch::Expanded,
                "extraexpanded" => self.stretch = Stretch::ExtraExpanded,
                "ultraexpanded" => self.stretch = Stretch::UltraExpanded,
                _ => {}
            }
        }
    }

    pub fn apply_modifier(&mut self, modifier: Modifier) {
        match modifier {
            Modifier::Style(style) => self.style = style,
            Modifier::Weight(weight) => self.weight = weight,
            Modifier::Color(color) => self.color = color,
            Modifier::Strikethrough => {
                // unimplemented
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Modifier {
    Style(Style),
    Weight(Weight),
    Color(Color32),
    Strikethrough,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum Family {
    Name(Arc<str>),
    Serif,
    #[default]
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Style {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub enum Align {
    #[default]
    Left,
    Right,
    Center,
    Justified,
    End,
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Weight(#[default = 400] pub u16);

impl Weight {
    pub const THIN: Weight = Weight(100);
    pub const EXTRA_LIGHT: Weight = Weight(200);
    pub const LIGHT: Weight = Weight(300);
    pub const NORMAL: Weight = Weight(400);
    pub const MEDIUM: Weight = Weight(500);
    pub const SEMIBOLD: Weight = Weight(600);
    pub const BOLD: Weight = Weight(700);
    pub const EXTRA_BOLD: Weight = Weight(800);
    pub const BLACK: Weight = Weight(900);
}
