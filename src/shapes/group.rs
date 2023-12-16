use crate::shapes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "np")]
    pub number_of_properties: Option<i64>,
    #[serde(rename = "it")]
    pub items: Vec<shapes::AnyShape>,
}
