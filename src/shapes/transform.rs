use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "a", default = "properties::MultiDimensional::zero")]
    pub anchor_point: properties::MultiDimensional,
    #[serde(rename = "p", default = "properties::MultiDimensional::zero")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s", default = "properties::MultiDimensional::hundred")]
    pub scale: properties::MultiDimensional,
    #[serde(rename = "r", default)]
    pub rotation: properties::Scalar,
    #[serde(rename = "o", default = "properties::Scalar::hundred")]
    pub opacity: properties::Scalar,
    #[serde(rename = "sk", default)]
    pub skew: properties::Scalar,
    #[serde(rename = "sa", default)]
    pub skew_axis: properties::Scalar,
}
