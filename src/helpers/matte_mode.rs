use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MatteMode {
    Normal = 0,
    Alpha = 1,
    InvertAlpha = 2,
    Luma = 3,
    InvertLuma = 4,
}
