/*
    Appellation: data <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[doc(hidden)]
pub use self::samples::*;
#[doc(hidden)]
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
    pub type Id = String;

    pub type Uid = uuid::Uuid;

    pub type Date<Tz = chrono::Local> = chrono::DateTime<Tz>;
}

#[doc(hidden)]
pub mod samples {
    use crate::data::{Profile, Task};
    use crate::{profile, task};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref SAMPLE_PROFILES: Vec<Profile> = profile![
            "Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy"
        ];
        pub static ref SAMPLE_TASKS: Vec<Task> = task![
            "Task 1", "Task 2", "Task 3", "Task 4", "Task 5", "Task 6", "Task 7", "Task 8",
            "Task 9", "Task 10"
        ];
    }
}
