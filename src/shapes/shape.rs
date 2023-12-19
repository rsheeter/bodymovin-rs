use crate::{properties, util};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Shape {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "d")]
    pub direction: Option<f64>,
    #[serde(skip_serializing_if = "util::bool_is_false")]
    #[serde(default)]
    pub closed: bool,
    #[serde(rename = "ks")]
    pub vertices: properties::Shape,
}
