use std::fmt::Debug;

use ecolor::Color32;
use emath::{Align2, Pos2, Rect, lerp};
use indexmap::IndexMap;
use oklab::Oklab;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smart_default::SmartDefault;

use crate::actions::{DrawableAction, SlideParams};

#[derive(Deserialize, Serialize, SmartDefault, Debug)]
pub struct Slide {
    pub objects: IndexMap<smartstring::alias::String, SlideObj, ahash::RandomState>,
    pub actions: SmallVec<[DrawableAction; 2]>,
    pub slide_params: SlideParams,
    /// The background color is in the Oklab colors space so the
    /// interpolation looks better
    pub bg: BgColor,
    pub create_edges: bool,
}

impl Slide {
    pub fn uses_viewbox(&self, viewbox: &smartstring::alias::String) -> bool {
        for (_, obj) in &self.objects {
            if obj
                .viewbox
                .as_ref()
                .map(|vb| vb.eq(viewbox))
                .unwrap_or_default()
            {
                return true;
            }
        }
        false
    }
}

#[derive(Deserialize, Serialize, SmartDefault, Debug, Clone, Copy, PartialEq)]
pub struct BgColor {
    #[default = 0.222]
    pub bg_l: f32,
    pub bg_a: f32,
    pub bg_b: f32,
    #[default = 1.0]
    pub alpha: f32,
}

impl BgColor {
    pub fn interpolate_bg(&self, other: &Self, time: f32) -> Color32 {
        let interpolated_bg = BgColor {
            bg_l: lerp(self.bg_l..=other.bg_l, time),
            bg_a: lerp(self.bg_a..=other.bg_a, time),
            bg_b: lerp(self.bg_b..=other.bg_b, time),
            alpha: lerp(self.alpha..=other.alpha, time),
        };

        interpolated_bg.into()
    }
}

impl From<BgColor> for Color32 {
    fn from(value: BgColor) -> Self {
        let oklab_bg = Oklab {
            l: value.bg_l,
            a: value.bg_a,
            b: value.bg_b,
        };

        let linear_bg = oklab::oklab_to_linear_srgb(oklab_bg);
        ecolor::Rgba::from_rgba_unmultiplied(linear_bg.r, linear_bg.g, linear_bg.b, value.alpha)
            .into()
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
/// `None` in viewbox or `vb_from` indicates an invalid
/// syntax tree, and there should be errors generated
pub struct SlideObj {
    pub viewbox: Option<SlideVb>,
    pub vb_from: Option<SlideVb>,
    pub positions: ObjPositions,
}

impl SlideObj {
    pub fn resolve_from_other(&mut self, mut other: Option<&Self>) {
        if other.is_none() || matches!(other.map(|o| o.positions.state), Some(ObjState::Exiting)) {
            self.positions.state = ObjState::Entering;
            other = None;
        }
        if self.positions.to_alignment.is_none() {
            self.positions.to_alignment = if matches!(self.positions.state, ObjState::Exiting) {
                self.positions.from_alignment
            } else {
                other.and_then(|o| o.positions.to_alignment)
            };
        }
        if self.positions.from_alignment.is_none()
            || matches!(self.positions.state, ObjState::Exiting)
        {
            self.positions.from_alignment = other.and_then(|o| o.positions.to_alignment);
        }
        if self.viewbox.is_none() {
            self.viewbox = other.and_then(|o| o.viewbox.clone());
        }
        if self.vb_from.is_none() {
            match self.positions.state {
                ObjState::Entering => self.vb_from = self.viewbox.clone(),
                ObjState::OnScreen | ObjState::Exiting => {
                    self.vb_from = other.and_then(|o| o.viewbox.clone())
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Copy)]
pub struct ObjPositions {
    pub from_alignment: Option<Align2>,
    pub to_alignment: Option<Align2>,
    pub state: ObjState,
}

#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug)]
pub enum ObjState {
    Entering,
    #[default]
    OnScreen,
    Exiting,
}

pub const BASE_SIZE: Rect = Rect {
    min: Pos2::ZERO,
    max: Pos2::new(1920.0, 1080.0),
};

#[derive(Deserialize, Serialize, SmartDefault, Debug, Clone)]
pub enum SlideVb {
    #[default]
    Viewbox(ViewboxRef),
    InnerVb {
        split: SmallVec<[Rect; 4]>,
        subbox: usize,
    },
}

impl SlideVb {
    pub fn set_subbox(&mut self, new_subbox: usize) {
        match self {
            SlideVb::Viewbox(vb) => vb.subbox = new_subbox,
            SlideVb::InnerVb { subbox, .. } => *subbox = new_subbox,
        }
    }
}

impl PartialEq<smartstring::alias::String> for SlideVb {
    fn eq(&self, other: &smartstring::alias::String) -> bool {
        match self {
            Self::Viewbox(vb) => vb.vb_name.eq(other),
            _ => false,
        }
    }
}

impl Debug for ObjPositions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;

        fn debug_align(align: Align2, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match align {
                Align2::LEFT_BOTTOM => write!(f, "<_"),
                Align2::LEFT_CENTER => write!(f, "<<"),
                Align2::LEFT_TOP => write!(f, "^<"),
                Align2::CENTER_BOTTOM => write!(f, "__"),
                Align2::CENTER_CENTER => write!(f, ".."),
                Align2::CENTER_TOP => write!(f, "^^"),
                Align2::RIGHT_BOTTOM => write!(f, ">_"),
                Align2::RIGHT_CENTER => write!(f, ">>"),
                Align2::RIGHT_TOP => write!(f, "^>"),
            }
        }

        if let Some(from) = self.from_alignment {
            debug_align(from, f)?;
        }

        if matches!(self.state, ObjState::Exiting) {
            return write!(f, "|\"");
        }

        if let Some(to) = self.to_alignment {
            debug_align(to, f)?;
        }

        write!(f, "\"")?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ViewboxRef {
    pub vb_name: VbIdentifier,
    pub subbox: usize,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub enum VbIdentifier {
    Named(smartstring::alias::String),
    Rect(Rect),
    #[default]
    Size,
}

impl PartialEq<smartstring::alias::String> for VbIdentifier {
    fn eq(&self, other: &smartstring::alias::String) -> bool {
        match self {
            Self::Named(n) => n.eq(other),
            _ => false,
        }
    }
}
