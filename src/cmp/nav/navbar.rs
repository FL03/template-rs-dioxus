/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::AppBar;
use crate::{Route, TITLE};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let links = vec![
        rsx! { Link { to: Route::Home {}, "Home" } },
        rsx! { Link { to: Route::Tasks {}, "Tasks" } },
    ];
    let title = TITLE.to_string();
    rsx! {
        AppBar { links, title },
        Outlet::<crate::Route> {}
    }
}
