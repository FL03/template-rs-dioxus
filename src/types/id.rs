/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Owned(pub [u8; 16]);

impl Owned {
    pub fn v4() -> Self {
        Self(*uuid::Uuid::new_v4().as_bytes())
    }
}

wrapper!(Id(String));

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
