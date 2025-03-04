use std::fmt::Debug;

use ecolor::Color32;
use emath::{Align2, Pos2, Rect};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smart_default::SmartDefault;

use crate::actions::{DrawableAction, SlideParams};

#[derive(Deserialize, Serialize, SmartDefault, Debug)]
pub struct Slide {
    pub objects: IndexMap<smartstring::alias::String, SlideObj, ahash::RandomState>,
    pub actions: SmallVec<[DrawableAction; 2]>,
    pub slide_params: SlideParams,
    #[default(Color32::from_gray(27))]
    pub bg: Color32,
    #[serde(skip)]
    #[cfg(feature = "parse")]
    // Used to remove slides that are no longer in the syntax tree
    #[default = true]
    pub present: bool,
    pub create_edges: bool,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
/// `None` in viewbox or `vb_from` indicates an invalid
/// syntax tree, and there should be errors generated
pub struct SlideObj {
    pub viewbox: Option<SlideVb>,
    pub vb_from: Option<SlideVb>,
    pub positions: ObjPositions,
}

#[derive(Deserialize, Serialize, Default, Clone, Copy)]
pub enum ObjState {
    Entering,
    #[default]
    OnScreen,
    Exiting,
}

#[derive(Deserialize, Serialize, Default, Clone, Copy)]
pub struct ObjPositions {
    pub from_alignment: Option<Align2>,
    pub to_alignment: Option<Align2>,
    pub state: ObjState,
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
