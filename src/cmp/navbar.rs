/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn Navbar(links: Vec<Element>, title: String) -> Element {
    rsx! {
            div {
                class: "container mx-auto p-4 bg-transparent",
                nav {
                    class: "flex justify-between shadow rounded p-4",
                    div {
                        class: "flex text-start",
                        Link {
                            to: "/" {},
                            "{title}"
                        }
                    }
                    div {
                        class: "flex flex-grow justify-end",
                        ul {
                            class: "block",
                            {{
                                links.iter().cloned().map(|link| {
                                    rsx! {
                                        li {
                                            class: "mx-2",
                                            {link}
                                        }
                                    }
                                })
                            }
                        }
                    }
                    div {
                        class: "flex center",
                        Link {
                            to: "/profile" {},
                            "Profile"
                        }
                    }
                }
                }
        }
    }
}
