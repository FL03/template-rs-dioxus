/*
    Appellation: search <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

/// [`Search`] defines a search input field
pub fn Search(placeholder: Option<&str>) -> Element {
    rsx! {
        input {
            class: "w-full h-10 px-3 text-sm text-gray-700 placeholder-gray-400 bg-white border border-gray-300 rounded-lg appearance-none focus:outline-none focus:shadow-outline",
            r#type: "text",
            placeholder: placeholder.unwrap_or("Search..."),
            oninput: |e| {},
        }
    }
}