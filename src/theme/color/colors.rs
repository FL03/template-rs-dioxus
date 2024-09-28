/*
    Appellation: colors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
pub enum Colors {
    Blue,
    Cyan,
    Green,
    Magenta,
    Orange,
    Red,
    White,
}

pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub struct Opaque<S, T> {
    pub inner: S,
    pub a: T,
}
