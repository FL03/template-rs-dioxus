/*
    Appellation: sidebar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::badge::GrayBadge as Badge;
use dioxus::prelude::*;

/// [`Sidebar`] defines the drawer element for the dashboard; i.e. a widget containing
/// navigational links and/or information regarding the account, application / system, etc.
/// Typically, these elements remain hidden until the user interacts with one of its triggers.
pub fn Sidebar() -> Element {
    rsx! {
        aside { class: "w-64 bg-white shadow-md",
            div { class: "p-4",
                h1 { class: "text-2xl font-bold text-gray-800", {crate::APP_NAME} }
                Badge { text: "Owner".to_string() }
            }
            nav { class: "mt-4",
                // Link {
                //     class: "flex items-center px-4 py-2 text-gray-700 hover:bg-gray-200",
                //     to: "/dashboard",
                //     // svg { class: "w-5 h-5 mr-2", /* Dashboard icon SVG */ }
                //     "Dashboard"
                // }
            }
        }
    }
}

///
pub fn SidebarHeader() -> Element {
    rsx! {
        div { class: "p-4",
            h1 { class: "text-2xl font-bold text-gray-800", {crate::APP_NAME} }
            Badge { text: "Admin".to_string() }
        }
    }
}
