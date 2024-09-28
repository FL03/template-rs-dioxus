/*
    Appellation: appbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn AppBar(links: Vec<Element>, title: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col max-w-sm min-h-full max-h-screen bg-black",
        }
    }
}
