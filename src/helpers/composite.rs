use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Composite {
    Above = 1,
    Below = 2,
}

impl Default for Composite {
    fn default() -> Self {
        Self::Above
    }
}
