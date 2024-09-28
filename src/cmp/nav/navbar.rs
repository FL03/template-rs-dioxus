/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::AppBar;
use crate::{Route, APP_NAME};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let links = vec![
        rsx! { Link { to: Route::Dashboard {}, "Dashboard" } },
        rsx! { Link { to: Route::Tasks {}, "Tasks" } },
    ];
    let title = APP_NAME.to_string();
    rsx! {
        AppBar { links, title }
        Outlet::<crate::Route> {}
    }
}
