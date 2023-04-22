/*
    Appellation: template-rs-dioxide <web>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

fn main() -> anyhow::Result<()> {
    #[cfg(target_family = "wasm")]
    {
        use curiosity::{app, ApplicationScope};

        let scope = ApplicationScope::new();
        dioxus_web::launch_with_props(app, scope, dioxus_web::Config::new());
    }
    

    Ok(())
}