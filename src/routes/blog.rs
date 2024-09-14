/*
    Appellation: blog <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}
