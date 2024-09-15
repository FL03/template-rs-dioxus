/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

wrapper!(Id(String), Timestamp(String), Title(String));

impl Id {
    pub fn v4() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::v4()
    }
}

impl From<uuid::Uuid> for Id {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid.to_string())
    }
}

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

impl<Tz> From<chrono::DateTime<Tz>> for Timestamp
where
    Tz: chrono::TimeZone,
    Tz::Offset: core::fmt::Display,
{
    fn from(dt: chrono::DateTime<Tz>) -> Self {
        Self(dt.format("%A, %B %e, %Y").to_string())
    }
}

impl Default for Title {
    fn default() -> Self {
        Self(crate::APP_NAME.to_string())
    }
}
