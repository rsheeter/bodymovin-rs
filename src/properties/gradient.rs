use crate::{properties, util};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GradientStopsValue {
    #[serde(rename = "c")]
    pub closed: Option<bool>,
    #[serde(rename = "i")]
    pub in_point: Vec<(f64, f64)>,
    #[serde(rename = "o")]
    pub out_point: Vec<(f64, f64)>,
    #[serde(rename = "v")]
    pub vertices: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GradientStopsKeyframe {
    #[serde(rename = "s")]
    pub start_value: Option<Vec<GradientStopsValue>>,
    #[serde(rename = "e")]
    pub end_value: Option<Vec<GradientStopsValue>>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(flatten)]
    pub bezier: Option<properties::Bezier3d>,
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

pub type GradientStops = properties::Property<GradientStopsValue, GradientStopsKeyframe>;
