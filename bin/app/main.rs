/*
    Appellation: app <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(non_snake_case)]
#![crate_name = "app"]

use dioxus_logger::tracing::{self, Level};
use template_rs_dioxus::app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init logger
    dioxus_logger::init(Level::TRACE)?;
    // Log starting message
    tracing::info!("Starting the application...");
    // Launch the app
    Ok(dioxus::launch(app::run))
}
