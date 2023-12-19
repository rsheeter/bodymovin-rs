use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "d")]
    pub direction: f64,
    #[serde(rename = "p")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s")]
    pub size: properties::MultiDimensional,
    #[serde(rename = "r")]
    pub rounded_corners: properties::Scalar,
}
