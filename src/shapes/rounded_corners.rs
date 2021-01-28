use crate::properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoundedCorners {
    #[serde(rename = "mn")]
    pub match_name: String,
    #[serde(rename = "nm")]
    pub name: String,
    #[serde(rename = "r")]
    pub radius: properties::EitherValue,
}
