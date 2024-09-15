/*
    Appellation: appbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn AppBar(links: Vec<Element>, title: String) -> Element {
    rsx! {
        div {
            class: "container mx-auto p-4 bg-transparent",
            nav {
                class: "flex items-center justify-between shadow rounded-lg p-4",
                div {
                    class: "flex flex-shrink text-start",
                    Link {
                        to: "/" ,
                        "{title}"
                    }
                }
                div {
                    class: "flex flex-grow justify-end",
                    ul {
                        class: "flex justify-start",
                        {{
                            links.iter().map(|link| {
                                rsx! {
                                    li {
                                        class: "block mx-2",
                                        {link}
                                    }
                                }
                            })
                        }}
                    }
                    div {
                        class: "flex flex-shrink centered",
                        button {
                            class: "block mx-2",
                            onclick: |_| {
                                log::tracing::info!("Profile button clicked");
                            },
                            "Profile"
                        }
                    }
                }
            }
        }
    }
}
