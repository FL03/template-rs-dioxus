/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::routes::Route;
use dioxus::prelude::*;



pub fn App() -> Element {
    rsx! {
        div {
            class: "block m-0 p-0 text-black dark:text-white",
            Router::<Route> {}
        }
    }
}
