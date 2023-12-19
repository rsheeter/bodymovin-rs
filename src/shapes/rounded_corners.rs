use crate::properties;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoundedCorners {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "r")]
    pub radius: properties::Scalar,
}
