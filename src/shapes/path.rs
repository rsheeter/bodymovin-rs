use crate::properties;
use serde::{Deserialize, Serialize};

/// A cubic b-spline, a subpath in svg or kurbo terms.
///
/// <https://lottiefiles.github.io/lottie-docs/shapes/#path>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "d")]
    pub direction: Option<f64>,
    #[serde(rename = "ks")]
    pub vertices: properties::Shape,
}
