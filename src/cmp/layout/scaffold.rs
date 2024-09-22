/*
    Appellation: scaffold <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::cmp::AppBar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Scaffold(title: String) -> Element {
    let links = vec![
        rsx! { Link { to: "/" {}, "Home" } },
        rsx! { Link { to: Route::Tasks {}, "Tasks" } },
    ];
    rsx! {
        AppBar { links, title }
        main { class: "flex flex-col min-h-full", Outlet::<crate::Route> {} }
    }
}

pub enum Links {
    Route(Route),
    String(String),
}

pub struct Hyperlink {
    pub to: Links,
    pub children: String,
}

pub struct LinkTree {
    pub links: Vec<Hyperlink>,
}
