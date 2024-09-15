/*
    Appellation: profile <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::{Id, Timestamp};

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
    pub(crate) email: String,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) created_at: Timestamp,
    pub(crate) updated_at: Timestamp,
}

impl Profile {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            email: String::new(),
            username: String::new(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
