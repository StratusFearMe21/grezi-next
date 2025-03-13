use std::sync::Arc;

use ecolor::Color32;
use emath::Align2;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Deserialize, Serialize, SmartDefault, Debug, Clone)]
pub struct SlideParams {
    pub speaker_notes: Option<Arc<str>>,
    pub stagger: f64,
    #[default = 0.5]
    pub time: f64,
    pub next: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DrawableAction {
    Highlight {
        object: smartstring::alias::String,
        locations: Option<[[usize; 3]; 2]>,
        color: Color32,
    },
    Line {
        objects: [smartstring::alias::String; 2],
        locations: [Align2; 2],
        color: Color32,
    },
}
