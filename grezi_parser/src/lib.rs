use std::collections::HashMap;

use emath::Rect;
use object::Object;
use serde::{Deserialize, Serialize};
use slide::Slide;
use smallvec::SmallVec;

pub mod actions;
pub mod object;
#[cfg(feature = "parse")]
pub mod parse;
pub mod slide;
pub mod text;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct GrzRoot {
    pub viewboxes: HashMap<
        smartstring::alias::String,
        (SmallVec<[Rect; 4]>, /* present */ bool),
        ahash::RandomState,
    >,
    pub objects: HashMap<smartstring::alias::String, Object, ahash::RandomState>,
    pub slides: Vec<Slide>,
}
