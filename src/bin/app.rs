/*
    Appellation: app <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use dioxus_logger::tracing;
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    dioxus::launch(sdk::App);

    Ok(())
}
