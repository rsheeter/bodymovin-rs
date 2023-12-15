use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LineCap {
    Butt = 1,
    Round = 2,
    Square = 3,
}

impl Default for LineCap {
    fn default() -> Self {
        Self::Round
    }
}
