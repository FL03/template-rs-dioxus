/*
    Appellation: dash <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::{badge::GrayBadge as Badge, nav::Sidebar};
use dioxus::prelude::*;

pub enum Sized {
    Sm,
    Md,
    Lg,
    Xl,
    XXL,
}

pub struct WrapperProps {
    pub background: String,
    pub margin: usize,
    pub padding: usize,

    pub child: Element,
}
/// [`DashboardScaffold`] defines the base layout for the dashboard
#[component]
pub fn DashboardScaffold(child: Element) -> Element {
    let clock = "12:00 PM";
    let weather = "87°F";
    rsx! {
        // Base layout
        body { class: "min-h-screen bg-gray-100 text-gray-900 flex",
            //
            aside { class: "w-64 bg-white shadow-md",
                div { class: "p-4",
                    h1 { class: "text-2xl font-bold text-gray-800", "Proton" }
                    Badge { text: "owner".to_string() }
                }
                nav { class: "mt-4",
                }
            }
            div { class: "flex-grow flex flex-col",
                header { class: "w-full bg-blue-600 text-white p-2 flex items-center justify-between",
                    div { class: "flex items-center space-x-2",
                        i { "data-lucide": "atom", class: "h-6 w-6" }
                        span { class: "font-bold text-lg", "Proton" }
                    }
                    div { class: "xs:hidden flex items-center space-x-4",
                        div { class: "flex items-center space-x-2",
                            i { "data-lucide": "cloud", class: "h-5 w-5" }
                            span { class: "inline", {weather} }
                        }
                        div { class: "text-sm sm:text-base font-medium",
                            span { class: "inline", {clock} }
                        }
                        button { class: "text-white",
                            i { "data-lucide": "menu", class: "h-6 w-6" }
                        }
                    }
                }
                div { class: "p-4 flex-grow",
                    div { class: "w-full sm:w-64 bg-white rounded-lg shadow-md cursor-pointer hover:shadow-lg transition-shadow",
                        div { class: "p-4 flex items-center space-x-4",
                            img {
                                height: "40",
                                alt: "User avatar",
                                src: "/placeholder.svg?height=40&width=40",
                                width: "40",
                                class: "rounded-full"
                            }
                            div {
                                h2 { class: "font-semibold", "John Doe" }
                                p { class: "text-sm text-gray-500", "@johndoe" }
                            }
                        }
                    }
                }
                div { class: "p-4 w-full max-w-lg mx-auto",
                    div { class: "relative",
                        i {
                            "data-lucide": "search",
                            class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
                        }
                        input {
                            placeholder: "Search...",
                            r#type: "search",
                            class: "pl-10 pr-4 py-2 w-full border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        }
                    }
                }
            }
            div { class: "fixed right-0 top-14 bottom-0 bg-gray-200 p-2 flex flex-col justify-center shadow-lg",
                button { class: "mb-2 p-2 rounded-md hover:bg-gray-300",
                    i { "data-lucide": "home", class: "h-6 w-6" }
                    span { class: "sr-only", "Home" }
                }
                button { class: "mb-2 p-2 rounded-md hover:bg-gray-300",
                    i { "data-lucide": "mail", class: "h-6 w-6" }
                    span { class: "sr-only", "Mail" }
                }
                button { class: "mb-2 p-2 rounded-md hover:bg-gray-300",
                    i { "data-lucide": "calendar", class: "h-6 w-6" }
                    span { class: "sr-only", "Calendar" }
                }
                button { class: "mb-2 p-2 rounded-md hover:bg-gray-300",
                    i { "data-lucide": "file-text", class: "h-6 w-6" }
                    span { class: "sr-only", "Documents" }
                }
                button { class: "mb-2 p-2 rounded-md hover:bg-gray-300",
                    i { "data-lucide": "layout-dashboard", class: "h-6 w-6" }
                    span { class: "sr-only", "Dashboard" }
                }
            }
        }
    }
}