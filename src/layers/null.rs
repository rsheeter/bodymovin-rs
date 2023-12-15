use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NullMixin {}

pub type Null = super::Layer<NullMixin>;
