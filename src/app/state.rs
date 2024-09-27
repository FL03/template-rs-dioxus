/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::{Profile, Task};
use crate::{Id, Settings, Timestamp};

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct AppStore {
    pub cache: String,
    pub profiles: Vec<Profile>,
    pub tasks: Vec<Task>,
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
pub struct AppState {
    pub id: Id,
    pub settings: Settings,
    pub store: AppStore,
    pub timestamp: Timestamp,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            id: Id::v4(),
            settings: Settings::default(),
            store: AppStore::new(),
            timestamp: Timestamp::now(),
        }
    }

    pub fn from_config(settings: Settings) -> Self {
        Self {
            id: Id::v4(),
            settings,
            store: AppStore::new(),
            timestamp: Timestamp::now(),
        }
    }

    pub fn with_id(self, id: Id) -> Self {
        Self { id, ..self }
    }

    pub fn with_config(self, settings: Settings) -> Self {
        Self { settings, ..self }
    }

    pub fn with_store(self, store: AppStore) -> Self {
        Self { store, ..self }
    }

    pub const fn id(&self) -> &Id {
        &self.id
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    pub const fn store(&self) -> &AppStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut AppStore {
        &mut self.store
    }

    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    pub fn set_store(&mut self, store: AppStore) {
        self.store = store;
    }

    pub fn as_signal(&self) -> dioxus::prelude::Signal<AppState> {
        dioxus::prelude::Signal::from(self.clone())
    }
}

impl core::fmt::Display for AppState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

mod impl_store {
    use super::*;
    use crate::data::samples::*;

    impl AppStore {
        pub fn new() -> Self {
            Self {
                cache: String::new(),
                profiles: SAMPLE_PROFILES.clone(),
                tasks: SAMPLE_TASKS.clone(),
            }
        }
    }

    impl Default for AppStore {
        fn default() -> Self {
            Self::new()
        }
    }
}
