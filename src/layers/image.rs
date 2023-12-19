use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageMixin {
    #[serde(default = "util::a_u8_2_please")]
    ty: u8,
    #[serde(rename = "refId")]
    pub ref_id: String,
}

pub type Image = super::Layer<ImageMixin>;
