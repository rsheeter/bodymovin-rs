use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// TODO: Is there a nicer way to deserialize this nested data?
/* "t" {
        "d" {
            "k" {
                "s" {
                    "t": "Text",
                    "lh": line height,
                    "fc": font color
                }
            }
        }
    }
*/
#[derive(Serialize, Deserialize)]
pub struct TextProperties {
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "lh")]
    pub line_height: f64,
    #[serde(rename = "fc")]
    pub font_color: Vec<f64>,
}
#[derive(Serialize, Deserialize)]
pub struct TextKeyframeData {
    #[serde(rename = "s")]
    pub properties: TextProperties,
}
#[derive(Serialize, Deserialize)]
pub struct TextDocumentData {
    #[serde(rename = "k")]
    pub keyframe_data: Vec<TextKeyframeData>,
}
#[derive(Serialize, Deserialize)]
pub struct TextData {
    #[serde(rename = "d")]
    pub document_data: TextDocumentData,
}
#[derive(Serialize, Deserialize)]
pub struct TextMixin {
    #[serde(rename = "t")]
    pub text_data: TextData,
}

impl Debug for TextMixin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextMixin")
            .field(
                "text",
                &self.text_data.document_data.keyframe_data[0]
                    .properties
                    .text,
            )
            .field(
                "line height",
                &self.text_data.document_data.keyframe_data[0]
                    .properties
                    .line_height,
            )
            .finish()
    }
}

pub type Text = super::Layer<TextMixin>;
