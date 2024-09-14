/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn Navbar(links: Vec<Element>, title: String) -> Element {
    rsx! {
            div {
                class: "block width-screen mx-auto p-4 bg-transparent",
                nav {
                    class: "flex justify-between shadow rounded p-4",
                    div {
                        class: "flex flex-shrink text-start",
                        Link {
                            to: "/" {},
                            "{title}"
                        }
                    }
                    div {
                        class: "flex flex-grow centered",
                        ul {
                            class: "flex",
                            {{
                                links.iter().cloned().map(|link| {
                                    rsx! {
                                        li {
                                            class: "block mx-2",
                                            {link}
                                        }
                                    }
                                })
                            }
                        }
                    }
                    div {
                        class: "flex centered",
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
