/*
    Appellation: template-rs-dioxus <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(non_snake_case)]

#[doc(inline)]
pub use self::{app::*, routes::Route, types::*};

#[macro_use]
pub(crate) mod macros;

pub mod app;
pub mod cmp;
pub mod data;
pub mod routes;
pub mod types;

use data::{Profile, Task};

extern crate dioxus_logger as log;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

pub const TITLE: &str = "RMS";

lazy_static::lazy_static! {
    static ref SAMPLE_PROFILES: Vec<Profile> = vec![
        Profile::new("Alice"),
        Profile::new("Bob"),
        Profile::new("Charlie"),
        Profile::new("David"),
        Profile::new("Eve"),

    ];

    static ref SAMPLE_TASKS: Vec<Task> = vec![
        Task::new("Alice's Task", "Do this"),
        Task::new("Bob's Task", "Complete another task"),
        Task::new("Charlie's Task", "Notify the team"),
        Task::new("David's Task", "Create the report"),
        Task::new("Eve's Task", "Summarize the data"),
    ];
}

pub mod prelude {
    pub use super::app::prelude::*;
    pub use super::cmp::prelude::*;
    pub use super::data::prelude::*;
    pub use super::routes::prelude::*;
    pub use super::types::prelude::*;
}
