use crate::layers;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreComp {
    pub id: String,
    #[serde(default)]
    pub layers: Vec<layers::AnyLayer>,
}
