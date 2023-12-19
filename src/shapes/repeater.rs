use crate::{helpers, properties};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repeater {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "c", default = "properties::Scalar::one")]
    pub copies: properties::Scalar,
    #[serde(rename = "o", default = "properties::Scalar::zero")]
    pub offset: properties::Scalar,
    #[serde(rename = "m", default)]
    pub composite: helpers::Composite,
    #[serde(rename = "tr")]
    pub transform: RepeaterTransform,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepeaterTransform {
    #[serde(rename = "a", default = "properties::MultiDimensional::zero")]
    pub anchor_point: properties::MultiDimensional,
    #[serde(rename = "p", default = "properties::MultiDimensional::zero")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s", default = "properties::MultiDimensional::hundred")]
    pub scale: properties::MultiDimensional,
    #[serde(rename = "r", default = "properties::Scalar::zero")]
    pub rotation: properties::Scalar,
    #[serde(rename = "so", default = "properties::Scalar::hundred")]
    pub start_opacity: properties::Scalar,
    #[serde(rename = "eo", default = "properties::Scalar::hundred")]
    pub end_opacity: properties::Scalar,
}
