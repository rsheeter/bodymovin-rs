use crate::{shapes, util};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Bounds {
    #[serde(rename = "l")]
    pub left: f64,
    #[serde(rename = "t")]
    pub top: f64,
    #[serde(rename = "b")]
    pub bottom: f64,
    #[serde(rename = "r")]
    pub right: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ShapeMixin {
    pub ty: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,
    pub shapes: Vec<shapes::AnyShape>,
}

pub type Shape = super::Layer<ShapeMixin>;

impl Default for ShapeMixin {
    fn default() -> Self {
        Self {
            ty: util::a_u8_4_please(),
            bounds: None,
            shapes: vec![]
        }
    }
}