/*
    Appellation: theme <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::ThemeMode;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ThemeSettings {
    pub mode: ThemeMode,
}

pub struct Theme {
    pub font_size: u16,
    pub background_color: String,
    pub color: String,
}
