use egui_glyphon::glyphon::cosmic_text;

use cosmic_text::{
    Align, AttrsOwned, CacheKeyFlags, Color, FamilyOwned, Metrics, Stretch, Style, Weight,
};

use jotdown::{ListKind, OrderedListNumbering, OrderedListStyle};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Option<ListKind>")]
pub enum ListKindOption {
    Some(#[serde(with = "ListKindSerde")] ListKind),
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "jotdown::ListKind")]
pub enum ListKindSerde {
    Unordered,
    Ordered {
        numbering: OrderedListNumbering,
        style: OrderedListStyle,
        start: u64,
    },
    Task,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "jotdown::OrderedListNumbering")]
pub enum OrderedListNumberingSerde {
    Decimal,
    AlphaLower,
    AlphaUpper,
    RomanLower,
    RomanUpper,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "jotdown::OrderedListStyle")]
pub enum OrderedListStyleSerde {
    Period,
    Paren,
    ParenParen,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Metrics")]
pub struct MetricsSerde {
    pub font_size: f32,
    pub line_height: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "FamilyOwned")]
pub enum FamilySerde {
    Name(String),
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Stretch")]
pub enum StretchSerde {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Style")]
pub enum StyleSerde {
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Weight")]
pub struct WeightSerde(pub u16);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Color")]
pub struct ColorSerde(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Option<Color>")]
pub enum ColorOpt {
    Some(#[serde(with = "ColorSerde")] Color),
    None,
}

mod cache_key_flags {
    use egui_glyphon::glyphon::cosmic_text::CacheKeyFlags;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(_: &CacheKeyFlags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<CacheKeyFlags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _: () = <()>::deserialize(deserializer)?;
        Ok(CacheKeyFlags::empty())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "AttrsOwned")]
pub struct AttrsSerde {
    #[serde(with = "ColorOpt")]
    pub color_opt: Option<Color>,
    #[serde(with = "FamilySerde")]
    pub family_owned: FamilyOwned,
    #[serde(with = "StretchSerde")]
    pub stretch: Stretch,
    #[serde(with = "StyleSerde")]
    pub style: Style,
    #[serde(with = "WeightSerde")]
    pub weight: Weight,
    pub metadata: usize,
    #[serde(with = "cache_key_flags")]
    pub cache_key_flags: CacheKeyFlags,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Option<Align>")]
pub enum AlignSerde {
    Some(#[serde(with = "AlignRef")] Align),
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Align")]
pub enum AlignRef {
    Left,
    Right,
    Center,
    Justified,
    End,
}
