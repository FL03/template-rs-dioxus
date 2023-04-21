/*
    Appellation: template-rs-dioxide <desktop>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

fn main() {
    #[cfg(any(macos, unix, windows))]
    dioxus_desktop::launch_with_props(curiosity::app, (), dioxus_desktop::Config::new());
}