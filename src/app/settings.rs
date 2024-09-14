/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Settings {
    pub title: String,
    pub version: String,
}

impl Settings {
    pub fn new(title: String, version: String) -> Self {
        Self { title, version }
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
            title: "Dioxus".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl core::fmt::Display for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
