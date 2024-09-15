/*
    Appellation: badge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use dioxus::prelude::*;

#[component]
pub fn Badge(background: String, color: String, text: String) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center px-2 py-1 rounded-md font-medium text-xs {background} {color} ring-1 ring-inset ring-gray-500/10",
            {text}
        }
    }
}

#[component]
pub fn GrayBadge(text: String) -> Element {
    rsx! {
        Badge {
            background: "bg-gray-50".to_string(),
            color: "text-gray-600".to_string(),
            text
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Props)]
pub struct BadgeProp {
    pub background: String,
    pub color: String,
    pub text: String,
}



pub struct BaseProps {
    pub background: String,
    pub border: String,
    pub color: String,
    pub text: String,
}