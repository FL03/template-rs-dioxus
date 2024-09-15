/*
    Appellation: page_not_found <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "container mx-auto p-4",
            div {
                class: "flex text-center p-4",
                h1 { "Error: Page Not Found ({route:?})" }
            }
        }
    }
}
