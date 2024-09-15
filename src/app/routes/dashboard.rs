/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::badge::GrayBadge as Badge;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "flex h-screen bg-gray-100",
            // Sidebar
            aside { class: "w-64 bg-white shadow-md",
                div { class: "p-4",
                    h1 { class: "text-2xl font-bold text-gray-800", {crate::APP_NAME} }
                    Badge { text: "Admin".to_string() }
                }
                nav { class: "mt-4",
                    Link { class: "flex items-center px-4 py-2 text-gray-700 hover:bg-gray-200", to: Route::Dashboard {},
                        // svg { class: "w-5 h-5 mr-2", /* Dashboard icon SVG */ }
                        "Dashboard"
                    }
                    Link { class: "flex items-center px-4 py-2 text-gray-700 hover:bg-gray-200", to: Route::Tasks {},
                        // svg { class: "w-5 h-5 mr-2", /* Settings icon SVG */ }
                        "Tasks"
                    }
                }
            }

            // Main Content
            div { class: "flex-1 flex flex-col overflow-hidden",
                // Header
                header { class: "flex items-center justify-between p-4 bg-white shadow-md",
                    div { class: "flex items-center",
                        input {
                            class: "max-w-64 px-4 py-2 text-zinc-800 bg-zinc-200 rounded-full focus:outline focus:bg-white",
                            placeholder: "Search...",
                            r#type: "text"
                        }
                    }
                    div { class: "flex items-center",
                        button { class: "p-1 text-gray-400 hover:text-gray-500",
                            // svg { class: "w-5 h-5", /* Bell icon SVG */ }
                        }
                        // User menu dropdown (simplified for this example)
                        button { class: "ml-2 flex items-center text-sm",
                            img {
                                class: "w-8 h-8 rounded-full",
                                src: "/favicon.ico",
                                alt: "User avatar"
                            }
                            // svg { class: "w-4 h-4 ml-2", /* ChevronDown icon SVG */ }
                        }
                    }
                }

                // Dashboard Content
                main { class: "flex-1 overflow-x-hidden overflow-y-auto bg-gray-100",
                    div { class: "container mx-auto px-6 py-8",
                        h3 { class: "text-3xl font-medium text-gray-700", "Dashboard" }
                        div { class: "mt-8",
                            // Dashboard widgets
                            div { class: "grid grid-cols-1 gap-6 mb-6 lg:grid-cols-3",
                                div { class: "w-full px-4 py-5 bg-white rounded-lg shadow",
                                    div { class: "text-sm font-medium text-gray-500 truncate", "Total Users" }
                                    div { class: "mt-1 text-3xl font-semibold text-gray-900", "12,345" }
                                }
                                div { class: "w-full px-4 py-5 bg-white rounded-lg shadow",
                                    div { class: "text-sm font-medium text-gray-500 truncate", "Total Profit" }
                                    div { class: "mt-1 text-3xl font-semibold text-gray-900", "$ 54,321" }
                                }
                                div { class: "w-full px-4 py-5 bg-white rounded-lg shadow",
                                    div { class: "text-sm font-medium text-gray-500 truncate", "Total Orders" }
                                    div { class: "mt-1 text-3xl font-semibold text-gray-900", "20,000" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
