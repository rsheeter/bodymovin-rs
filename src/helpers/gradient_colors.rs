use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GradientColors {
    #[serde(rename = "p")]
    pub count: f64,
    #[serde(rename = "k")]
    pub colors: properties::MultiDimensional,
}
