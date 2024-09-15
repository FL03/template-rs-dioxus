/*
    Appellation: card <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn Card(child: Element) -> Element {
    rsx! {
        div { class: "bg-white shadow-md rounded-lg p-6", {child} }
    }
}
