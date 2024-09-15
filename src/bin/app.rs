/*
    Appellation: app <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use dioxus_logger::tracing;
    // Init logger
    dioxus_logger::init(tracing::Level::TRACE)?;
    tracing::info!("Starting the application...");
    dioxus::launch(sdk::App);

    Ok(())
}
