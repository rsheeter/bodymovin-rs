use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NullMixin {
    #[serde(default = "util::a_u8_3_please")]
    ty: u8,
}

pub type Null = super::Layer<NullMixin>;
