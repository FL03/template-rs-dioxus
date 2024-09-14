/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::screens::{Blog, Home};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

pub fn App() -> Element {
    rsx! {
        body {
            class: "container mx-auto dark:bg-gray-500 bg-slate-700 text-black dark:text-white",
            Router::<Route> {}
        }
    }
}
