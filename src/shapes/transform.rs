use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "a")]
    pub anchor_point: properties::MultiDimensional,
    #[serde(rename = "p")]
    pub position: properties::MultiDimensional,
    #[serde(rename = "s")]
    pub scale: properties::MultiDimensional,
    #[serde(rename = "r")]
    pub rotation: properties::ScalarOrMultiKeyframe,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "sk")]
    pub skew: properties::Scalar,
    #[serde(rename = "sa", default)]
    pub skew_axis: properties::Scalar,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            name: Default::default(),
            anchor_point: properties::MultiDimensional::zero(),
            position: properties::MultiDimensional::zero(),
            scale: properties::MultiDimensional::hundred(),
            rotation: Default::default(),
            opacity: properties::Scalar::hundred(),
            skew: Default::default(),
            skew_axis: Default::default(),
        }
    }
}
