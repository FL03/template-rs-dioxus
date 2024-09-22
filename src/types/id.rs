/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
struct O<'a>(pub &'a [u8]);

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Owned(pub Vec<u8>);

impl Owned {
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid.to_string().as_bytes().to_vec())
    }
    pub fn v4() -> Self {
        Self::from_uuid(uuid::Uuid::new_v4())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl core::convert::AsRef<[u8]> for Owned {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::borrow::Borrow<[u8]> for Owned {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl core::ops::Deref for Owned {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Owned {
    fn default() -> Self {
        Self::v4()
    }
}

impl core::fmt::Display for Owned {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let uid = uuid::Uuid::new_v4();
        let id = Owned::from_uuid(uid);
        assert_eq!(id.to_string(), uid.to_string());
    }
}
