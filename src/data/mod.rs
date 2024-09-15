/*
    Appellation: data <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub(crate) use self::types::*;
#[doc(inline)]
pub use self::{profile::*, task::*};

pub mod profile;
pub mod task;

pub(crate) mod prelude {
    pub use super::profile::Profile;
    pub use super::task::Task;
}

#[allow(unused)]
pub(crate) mod types {
    pub(crate) type Id = String;

    pub(crate) type Uid = uuid::Uuid;

    pub(crate) type Date = chrono::DateTime<chrono::Utc>;
}
