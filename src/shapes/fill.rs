use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fill {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fillEnabled")]
    pub fill_enabled: Option<bool>,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    /// TODO: color can be in a [0, 255] representation as well
    #[serde(rename = "c")]
    pub color: properties::MultiDimensional,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            match_name: None,
            name: None,
            fill_enabled: None,
            opacity: properties::Scalar::hundred(),
            color: properties::MultiDimensional::fixed(vec![0.0; 3])
        }
    }
}