use egui_glyphon::glyphon::{
    cosmic_text::Align, cosmic_text::CacheKeyFlags, Attrs, AttrsOwned, Color, FamilyOwned, Stretch,
    Style, Weight,
};
use serde::{Deserialize, Serialize};

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
pub struct AttrsSerde {
    pub color_opt: Option<u32>,
    #[serde(with = "FamilySerde")]
    pub family_owned: FamilyOwned,
    #[serde(with = "StretchSerde")]
    pub stretch: Stretch,
    #[serde(with = "StyleSerde")]
    pub style: Style,
    #[serde(with = "WeightSerde")]
    pub weight: Weight,
    pub metadata: usize,
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

impl AttrsSerde {
    pub fn as_attrs(&self) -> Attrs<'_> {
        Attrs {
            color_opt: self.color_opt.map(|c| Color(c)),
            family: self.family_owned.as_family(),
            stretch: self.stretch,
            style: self.style,
            weight: self.weight,
            metadata: self.metadata,
            cache_key_flags: CacheKeyFlags::empty(),
        }
    }
}

impl From<AttrsOwned> for AttrsSerde {
    fn from(value: AttrsOwned) -> Self {
        Self {
            color_opt: value.color_opt.map(|c| c.0),
            family_owned: value.family_owned,
            stretch: value.stretch,
            style: value.style,
            weight: value.weight,
            metadata: value.metadata,
        }
    }
}
