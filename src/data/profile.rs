/*
    Appellation: profile <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::{Id, Name, Timestamp};
use std::collections::HashMap;

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
    pub(crate) id: Id,
    pub(crate) email: String,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) created_at: Timestamp,
    pub(crate) updated_at: Timestamp,
}

impl Profile {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: Id::v4(),
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

pub struct Contact {
    pub id: Id,
    pub name: Name,
    pub nickname: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
    pub email: HashMap<String, String>,
    pub phone: HashMap<String, String>,
    pub urls: HashMap<String, String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

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
pub struct Account {}
