/*
    Appellation: appbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn AppBar(links: Vec<Element>, title: String) -> Element {
    rsx! {
        div {
            class: "block items-center max-w-screen",
            nav {
                class: "flex flex-nowrap items-center justify-between shadow rounded-lg p-4",
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
                            class: "block mx-2 px-2 rounded-lg shadow text-dark bg-gradient-to-r from-cyan-700 to-blue-700",
                            onclick: |_| {
                                dxl::tracing::info!("Profile button clicked");
                            },
                            "Profile"
                        }
                    }
                }
            }
        }
    }
}
