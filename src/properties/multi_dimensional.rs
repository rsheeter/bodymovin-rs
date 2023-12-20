use super::Value;
use crate::{properties, util};
use serde::{Deserialize, Serialize};

pub type MultiDimensionalValue = Vec<properties::ScalarValue>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MultiDimensionalKeyframe {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s")]
    pub start_value: Option<MultiDimensionalValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "e")]
    pub end_value: Option<MultiDimensionalValue>,
    #[serde(rename = "t")]
    pub start_time: f64,
    #[serde(skip_serializing_if = "util::bool_is_false")]
    #[serde(rename = "h", deserialize_with = "util::bool_from_int", default)]
    pub hold: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub bezier: Option<properties::BezierEase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub spatial_bezier: Option<properties::SpatialBezier>,
}

impl MultiDimensionalKeyframe {
    pub fn scaled(self, scale: &Vec<f64>) -> Self {
        let start_value = self.start_value.as_ref().map(|val| {
            val.iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect::<Vec<_>>()
        });

        let end_value = self.end_value.as_ref().map(|val| {
            val.iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect::<Vec<_>>()
        });

        let spatial_bezier = if let Some(bezier) = &self.spatial_bezier {
            Some(bezier.scaled(scale))
        } else {
            None
        };

        Self {
            start_value,
            end_value,
            start_time: self.start_time,
            hold: self.hold,
            bezier: self.bezier,
            spatial_bezier,
        }
    }
}

pub type MultiDimensional = properties::Property<MultiDimensionalValue, MultiDimensionalKeyframe>;

impl MultiDimensional {
    pub(crate) fn zero() -> Self {
        Self::fixed(vec![0.0; 2])
    }

    pub(crate) fn hundred() -> Self {
        Self::fixed(vec![100.0; 2])
    }

    pub fn scaled(self, scale: &Vec<f64>) -> Self {
        Self {
            animated: if matches!(self.value, Value::Animated(..)) {
                1
            } else {
                0
            },
            value: match self.value {
                Value::Fixed(val) => Value::Fixed(
                    val.iter()
                        .zip(scale.iter())
                        .map(|(val, scale)| val * scale)
                        .collect::<Vec<_>>(),
                ),
                Value::Animated(vals) => {
                    Value::Animated(vals.into_iter().map(|val| val.scaled(scale)).collect())
                }
            },
            expression: self.expression,
            index: self.index,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SplitVector {
    #[serde(rename = "s")]
    pub split: bool,
    #[serde(rename = "x", default = "properties::Scalar::zero")]
    pub x_component: super::Scalar,
    #[serde(rename = "y", default = "properties::Scalar::zero")]
    pub y_component: super::Scalar,
    #[serde(rename = "z", default = "properties::Scalar::zero")]
    pub z_component: super::Scalar,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SplittableMultiDimensional {
    Split(SplitVector),
    Uniform(MultiDimensional),
}

impl SplittableMultiDimensional {
    pub(crate) fn zero() -> Self {
        Self::Uniform(MultiDimensional::zero())
    }
}
