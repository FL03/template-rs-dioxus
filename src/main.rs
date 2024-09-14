/*
    Appellation: template-rs-dioxus <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(non_snake_case)]

pub mod app;
pub mod screens;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app::App);
}
