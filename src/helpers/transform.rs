use crate::properties::{self, SplittableMultiDimensional};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    #[serde(rename = "a")]
    pub anchor_point: properties::MultiDimensional,
    #[serde(rename = "p")]
    pub position: properties::SplittableMultiDimensional,
    #[serde(rename = "s")]
    pub scale: properties::MultiDimensional,
    #[serde(rename = "r")]
    pub rotation: properties::Scalar,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "px")]
    pub position_x: properties::Scalar,
    #[serde(rename = "py")]
    pub position_y: properties::Scalar,
    #[serde(rename = "pz")]
    pub position_z: properties::Scalar,
    #[serde(rename = "sk")]
    pub skew: properties::Scalar,
    #[serde(rename = "sa")]
    pub skew_axis: properties::Scalar,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            anchor_point: properties::MultiDimensional::zero(),
            position: SplittableMultiDimensional::Uniform(properties::MultiDimensional::zero()),
            scale: properties::MultiDimensional::hundred(),
            rotation: Default::default(),
            opacity: properties::Scalar::hundred(),
            position_x: properties::Scalar::zero(),
            position_y: properties::Scalar::zero(),
            position_z: properties::Scalar::zero(),
            skew: Default::default(),
            skew_axis: Default::default(),
        }
    }
}
