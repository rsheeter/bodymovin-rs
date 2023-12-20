use crate::{helpers, util};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Layer<M> {
    #[serde(rename = "ks")]
    pub transform: helpers::Transform,
    #[serde(skip_serializing_if = "util::bool_is_false")]
    #[serde(deserialize_with = "util::bool_from_int", default)]
    pub auto_orient: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "w")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "h")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "util::is_default")]
    #[serde(rename = "bm", default)]
    pub blend_mode: helpers::BlendMode,
    #[serde(skip_serializing_if = "util::bool_is_false")]
    #[serde(rename = "ddd", deserialize_with = "util::bool_from_int", default)]
    pub is_3d: bool,
    #[serde(skip_serializing_if = "util::bool_is_false")]
    #[serde(rename = "td", deserialize_with = "util::bool_from_int", default)]
    pub is_track_matte: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tt")]
    pub matte_mode: Option<helpers::MatteMode>,
    #[serde(rename = "ind")]
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cl")]
    pub html_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ln")]
    pub html_id: Option<String>,
    #[serde(rename = "ip")]
    pub in_point: f64,
    #[serde(rename = "op")]
    pub out_point: f64,
    #[serde(rename = "st")]
    pub start_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "util::is_empty")]
    #[serde(rename = "masksProperties", default)]
    pub masks: Vec<helpers::Mask>,
    // TODO: this doesn't seem to work!
    // #[serde(rename = "ef", default)]
    // pub effects: Vec<effects::Index>,
    #[serde(rename = "sr", default = "util::one_please")]
    pub stretch: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parent")]
    pub parent: Option<i64>,
    #[serde(flatten)]
    pub mixin: M,
}
