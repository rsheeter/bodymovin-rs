mod ellipse;
mod fill;
mod gradient_fill;
mod gradient_stroke;
mod group;
mod merge;
mod path;
mod rect;
mod repeater;
mod rounded_corners;
mod star;
mod stroke;
mod transform;
mod trim;

pub use self::{
    ellipse::*, fill::*, gradient_fill::*, gradient_stroke::*, group::*, merge::*, path::*,
    rect::*, repeater::*, repeater::*, rounded_corners::*, star::*, stroke::*, transform::*,
    trim::*,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GradientType {
    Linear = 1,
    Radial = 2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "ty")]
pub enum AnyShape {
    #[serde(rename = "sh")]
    Shape(SubPath),
    #[serde(rename = "rc")]
    Rect(Rect),
    #[serde(rename = "el")]
    Ellipse(Ellipse),
    #[serde(rename = "sr")]
    Star(Star),
    #[serde(rename = "fl")]
    Fill(Fill),
    #[serde(rename = "gf")]
    GradientFill(GradientFill),
    #[serde(rename = "gs")]
    GradientStroke(GradientStroke),
    #[serde(rename = "st")]
    Stroke(Stroke),
    #[serde(rename = "mm")]
    Merge(Merge),
    #[serde(rename = "tm")]
    Trim(Trim),
    #[serde(rename = "rp")]
    Repeater(Repeater),
    #[serde(rename = "gr")]
    Group(Group),
    #[serde(rename = "rd")]
    RoundedCorners(RoundedCorners),
    #[serde(rename = "tr")]
    Transform(Transform),
}

impl AnyShape {
    pub fn name(&self) -> Option<&str> {
        match self {
            AnyShape::Shape(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Rect(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Ellipse(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Star(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Fill(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::GradientFill(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::GradientStroke(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Stroke(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Merge(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Trim(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Repeater(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Group(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::RoundedCorners(shape) => shape.name.as_ref().map(|x| x.as_str()),
            AnyShape::Transform(shape) => shape.name.as_ref().map(|x| x.as_str()),
        }
    }
}
