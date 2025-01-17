use crate::{helpers, properties};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Trim {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "s")]
    pub start: properties::Scalar,
    #[serde(rename = "e")]
    pub end: properties::Scalar,
    #[serde(rename = "o")]
    pub offset: properties::Scalar,
    #[serde(rename = "m")]
    pub mode: helpers::TrimMode,
}
