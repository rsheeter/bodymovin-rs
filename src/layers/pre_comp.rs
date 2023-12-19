use crate::{properties, util};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCompMixin {
    #[serde(default = "util::a_u8_0_please")]
    ty: u8,
    #[serde(rename = "refId")]
    pub ref_id: String,
    #[serde(rename = "tm", default)]
    pub time_remapping: properties::Scalar,
}

pub type PreComp = super::Layer<PreCompMixin>;
