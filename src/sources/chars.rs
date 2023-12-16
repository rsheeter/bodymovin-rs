use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharData {
    // TODO: everything
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Char {
    #[serde(rename = "ch")]
    pub value: String,
    #[serde(rename = "fFamily")]
    pub font_family: String,
    #[serde(rename = "size")]
    pub font_size: u64,
    #[serde(rename = "style")]
    pub font_style: String,
    #[serde(rename = "w")]
    pub width: f64,
    #[serde(rename = "data")]
    pub data: Option<CharData>,
}
