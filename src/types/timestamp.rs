/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Timestamp(pub String);

impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Local::now().format("%A, %B %e, %Y").to_string())
    }

    pub const fn get(&self) -> &String {
        &self.0
    }

    pub fn set(&mut self, value: String) {
        self.0 = value;
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
