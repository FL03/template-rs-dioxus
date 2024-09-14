/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Route;
use crate::cmp::Navbar;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            class: "container mx-auto p-4",
            Navbar {
                links: vec![
                    rsx! { Link { to: Route::Home {}, "Home" } },
                    rsx! { Link { to: Route::Tasks {}, "Tasks" } },
                ],
                title: "Dioxus"
            }
            div {
                class: "flex text-center p-4",
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    }
}
