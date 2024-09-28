/*
    Appellation: template-rs-dioxus <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # template-rs-dioxus
//!
//! A template application for the Dioxus framework.
#![allow(non_snake_case)]
#![crate_name = "template_rs_dioxus"]
#![crate_type = "lib"]

#[doc(inline)]
pub use self::{
    app::{App, Context, Route, Settings},
    theme::Theme,
    traits::prelude::*,
    types::prelude::*,
    utils::*,
};

#[macro_use]
pub(crate) mod macros;

pub mod app;
pub mod cmp;
pub mod data;
pub mod theme;
pub mod traits;
pub mod types;
pub mod utils;

pub mod prelude {
    pub use super::app::prelude::*;
    pub use super::cmp::prelude::*;
    pub use super::data::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}

extern crate dioxus as dx;
extern crate dioxus_logger as dxl;

// Urls are relative to your Cargo.toml file
// const _TAILWIND_URL: &str = manganis::mg!(file("https://cdn.tailwindcss.com"));

const _TAILWIND: &str = manganis::mg!(file("public/tailwind.css"));

pub const APP_NAME: &str = "Rustic";




pub(crate) mod primitives {
    use super::Route;
    use lazy_static::lazy_static;

    type LinkMap = std::collections::HashMap<&'static str, &'static str>;


    lazy_static! {
        pub static ref LINKS: LinkMap = LinkMap::from_iter([
            ("home", "/"),
            ("dashboard", "/dashboard"),
            ("profile", "/profile"),
            ("settings", "/settings"),
            ("tasks", "/tasks"),
        ]);
    }
}