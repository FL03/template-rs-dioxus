/*
    Appellation: template-rs-dioxus <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(non_snake_case)]

#[doc(inline)]
pub use self::{
    app::{App, Context, Route, Settings},
    traits::prelude::*,
    types::prelude::*,
};

#[macro_use]
pub(crate) mod macros;

pub mod app;
pub mod cmp;
pub mod data;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::app::prelude::*;
    pub use super::cmp::prelude::*;
    pub use super::data::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}

use data::{Profile, Task};

extern crate dioxus as dx;
extern crate dioxus_logger as dxl;

// Urls are relative to your Cargo.toml file
// const _TAILWIND_URL: &str = manganis::mg!(file("https://cdn.tailwindcss.com"));

pub const APP_NAME: &str = "RMS";

macro_rules! profile {
    ($($name:expr),* $(,)?) => {
        vec![
            $(
                $crate::data::Profile::new($name)
            ),*
        ]
    };
}

macro_rules! task {
    ($($name:expr),* $(,)?) => {
        vec![
            $(
                $crate::data::Task::new($name)
            ),*
        ]
    };
}

lazy_static::lazy_static! {
    static ref SAMPLE_PROFILES: Vec<Profile> = profile![
        "Alice",
        "Bob",
        "Charlie",
        "David",
        "Eve",
        "Frank",
        "Grace",
        "Heidi",
        "Ivan",
        "Judy"
    ];

    static ref SAMPLE_TASKS: Vec<Task> = task![
        "Task 1",
        "Task 2",
        "Task 3",
        "Task 4",
        "Task 5",
        "Task 6",
        "Task 7",
        "Task 8",
        "Task 9",
        "Task 10"
    ];
}
