use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum AppMode {
    Desktop,
    SSR,
    #[default]
    Web
}



#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Application {
    mode: AppMode,
    release: bool
}

impl Application {
    pub fn new() -> Self {
        Self {
            mode: Default::default(),
            release: false
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}