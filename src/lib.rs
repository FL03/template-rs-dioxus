/*
    Appellation: template-rs-dioxus <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(non_snake_case)]

#[doc(inline)]
pub use self::{app::*, routes::Route, types::*};

pub mod app;
pub mod cmp;
pub mod data;
pub mod routes;
pub mod types;

use data::{Profile, Task};

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

lazy_static::lazy_static! {
    static ref SAMPLE_PROFILES: Vec<Profile> = vec![
        Profile::new("Alice"),
        Profile::new("Bob"),
        Profile::new("Charlie"),
        Profile::new("David"),
        Profile::new("Eve"),

    ];

    static ref SAMPLE_TASKS: Vec<Task> = vec![
        Task::new("Alice's Task", "description"),
        Task::new("Bob's Task", "description"),
        Task::new("Charlie's Task", "description"),
        Task::new("David's Task", "description"),
        Task::new("Eve's Task", "description"),
    ];
}

pub mod prelude {
    pub use super::app::*;
    pub use super::cmp::prelude::*;
    pub use super::data::prelude::*;
    pub use super::routes::prelude::*;
    pub use super::types::*;
}
