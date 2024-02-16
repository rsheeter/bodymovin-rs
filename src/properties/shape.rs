use crate::{properties, util};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ShapeValue {
    #[serde(rename = "c")]
    pub closed: Option<bool>,
    #[serde(rename = "i")]
    pub in_point: Vec<(f64, f64)>,
    #[serde(rename = "o")]
    pub out_point: Vec<(f64, f64)>,
    #[serde(rename = "v")]
    pub vertices: Vec<(f64, f64)>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShapeKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<Vec<ShapeValue>>,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub end_value: Option<Vec<ShapeValue>>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(
        rename = "h",
        deserialize_with = "util::bool_from_int",
        serialize_with = "util::bool_to_int",
        skip_serializing_if = "util::is_default",
        default
    )]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::BezierEase>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

pub type Shape = properties::Property<ShapeValue, ShapeKeyframe>;
