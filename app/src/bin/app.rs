/*
    Appellation: template-rs-dioxide <app>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use curiosity::app;

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch_with_props(app, (), dioxus_web::Config::new());
    #[cfg(any(macos, unix, windows))]
    dioxus_desktop::launch_with_props(app, (), dioxus_desktop::Config::new());
}