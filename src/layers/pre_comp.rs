use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCompMixin {
    #[serde(rename = "refId")]
    pub ref_id: String,
    #[serde(rename = "tm", default)]
    pub time_remapping: properties::Scalar,
}

pub type PreComp = super::Layer<PreCompMixin>;
