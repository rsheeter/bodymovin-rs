use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NullMixin {}

pub type Null = super::Layer<NullMixin>;
