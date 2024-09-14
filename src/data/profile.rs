/*
    Appellation: profile <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Uid;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Profile {
    pub(crate) id: String,
    pub(crate) name: String,
}

impl Profile {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: Uid::new_v4().to_string(),
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
