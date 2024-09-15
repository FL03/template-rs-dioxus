/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

wrapper!(Timestamp(String), Title(String));

impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Local::now().format("%A, %B %e, %Y").to_string())
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Default for Title {
    fn default() -> Self {
        Self(crate::TITLE.to_string())
    }
}

impl<Tz> From<chrono::DateTime<Tz>> for Timestamp where Tz: chrono::TimeZone, Tz::Offset: core::fmt::Display {
    fn from(dt: chrono::DateTime<Tz>) -> Self {
        Self(dt.format("%A, %B %e, %Y").to_string())
    }
}