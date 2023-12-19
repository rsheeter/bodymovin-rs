use crate::{properties, util};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolidMixin {
    #[serde(default = "util::a_u8_1_please")]
    ty: u8,
    #[serde(rename = "sc")]
    pub color: String,
    #[serde(rename = "sh")]
    pub height: properties::ScalarValue,
    #[serde(rename = "sw")]
    pub width: properties::ScalarValue,
}

pub type Solid = super::Layer<SolidMixin>;
