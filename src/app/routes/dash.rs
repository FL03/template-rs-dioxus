/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::scaffold::DashboardScaffold;

mod scaffold;

#[doc(hidden)]
pub(crate) mod prev_dash;

use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        DashboardScaffold {
            child: rsx! {
                div {
                    class: "flex flex-col items-center justify-center",
                    "Dashboard"
                }
            }
        }
    }
}

pub struct ProfileRef;

pub struct CurrentUser {
    pub username: String,
    pub uid: String,
}

pub struct DashboardCtx {}
