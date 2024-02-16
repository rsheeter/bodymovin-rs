use serde::{
    de::{self, Deserialize as _, Deserializer, Unexpected},
    Serializer,
};

pub const fn one_please() -> f64 {
    1.0
}

pub const fn a_u8_0_please() -> u8 {
    1
}

pub const fn a_u8_1_please() -> u8 {
    1
}

pub const fn a_u8_2_please() -> u8 {
    2
}

pub const fn a_u8_3_please() -> u8 {
    3
}

pub const fn a_u8_4_please() -> u8 {
    4
}

pub const fn a_u8_5_please() -> u8 {
    5
}

pub fn is_empty<T>(v: &Vec<T>) -> bool {
    v.is_empty()
}

pub fn is_default<T>(v: &T) -> bool
where
    T: Default + PartialEq,
{
    *v == T::default()
}

pub fn bool_is_false(v: &bool) -> bool {
    *v == false
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn bool_to_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(*value as i32)
}
