/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::cmp::nav::Sidebar;
use dioxus::prelude::*;

#[component]
pub fn Homepage() -> Element {
    rsx! {
        main { class: "flex-grow dark:text-white",
            div { class: "p-4",
                h1 { class: "text-2xl font-semibold", "Welcome to Proton" },
                p { class: "text-gray", "Proton is a dashboard template built with Dioxus." }
            }
        }
    }
}
