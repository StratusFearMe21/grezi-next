use std::sync::Arc;

use ecolor::Color32;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smart_default::SmartDefault;
use url::Url;

use crate::text::{Align, TextSection};

#[derive(SmartDefault, Serialize, Deserialize, Debug)]
pub struct Object {
    pub parameters: ObjInner,
    #[serde(skip)]
    #[cfg(feature = "parse")]
    // Used to remove objects that are no longer in the syntax tree
    #[default = true]
    pub present: bool,
}

#[derive(SmartDefault, Serialize, Deserialize, derive_more::Debug)]
pub enum ObjInner {
    Text {
        job: SmallVec<[TextSection; 1]>,
        line_height: Option<f32>,
        align: Align,
    },
    Image {
        // The parser feature being enabled
        // implies access to a filesystem
        // or the internet
        #[debug(ignore)]
        data: Arc<[u8]>,
        #[serde(serialize_with = "Url::serialize_internal")]
        #[serde(deserialize_with = "Url::deserialize_internal")]
        url: Url,
        scale: Option<f32>,
        tint: Color32,
    },
    #[default]
    Rect {
        color: Color32,
        stroke: Color32,
        height: f32,
    },
}
