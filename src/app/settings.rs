/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::ThemeMode;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ThemeSettings {
    pub mode: ThemeMode,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Settings {
    pub theme: ThemeMode,
    pub title: String,
    pub version: String,
}

impl Settings {
    pub fn new(theme: ThemeMode, title: impl ToString, version: impl ToString) -> Self {
        Self {
            theme,
            title: title.to_string(),
            version: version.to_string(),
        }
    }

    pub fn with_theme(self, theme: ThemeMode) -> Self {
        Self { theme, ..self }
    }

    pub fn with_title(self, title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            ..self
        }
    }

    pub fn with_version(self, version: impl ToString) -> Self {
        Self {
            version: version.to_string(),
            ..self
        }
    }

    pub fn theme(&self) -> ThemeMode {
        self.theme
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Light,
            title: crate::TITLE.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl core::fmt::Display for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
