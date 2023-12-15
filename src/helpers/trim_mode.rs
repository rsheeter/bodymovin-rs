use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TrimMode {
    Simultaneously = 1,
    Individually = 2,
}

impl Default for TrimMode {
    fn default() -> Self {
        Self::Simultaneously
    }
}
