/*
    Appellation: blog <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::app::Context;
use crate::data::{Profile, Task};
use crate::routes::*;
use dioxus::prelude::*;

#[component]
pub fn Tasks() -> Element {
    // let ctx = use_context::<Signal<Context>>();

    let ctx = use_context::<Signal<Context>>();

    let users = use_context::<Signal<Vec<Profile>>>();
    let tasks = use_context::<Signal<Vec<Task>>>();

    rsx! {
        div {
            crate::cmp::Navbar {
            links: vec![
                rsx! { Link { to: Route::Home {}, "Home" } },
                rsx! { Link { to: Route::Tasks {}, "Tasks" } },
            ],
            title: "Dioxus"
        }
        div { class: "mb-6 p-4 dark:text-dark",
                h1 { class: "text-2xl font-bold", "Tasks" }
                p { class: "text-sm text-gray-600", "{ctx.read().current_date}" }
                p { class: "text-sm text-gray-600",
                }
            }
            div { class: "grid gap-6 md:grid-cols-2",
                div { class: "bg-zinc-700 shadow rounded-lg p-4",
                    h2 { class: "text-xl font-semibold mb-4", "Employees on Shift" }
                    ul {
                        {{
                            users.read().iter().map(|employee| {
                                rsx! {
                                    li { class: "mb-2", "{employee.name}" }
                                }
                            })
                        }}
                    }
                }
                div { class: "dark:bg-zinc-700 shadow rounded-lg p-4",
                    h2 { class: "text-xl font-semibold mb-4", "Tasks" }
                    table { class: "w-full",
                        thead {
                            tr {
                                th { class: "text-left", "Task" }
                                th { class: "text-left", "Assigned To" }
                                th { class: "text-left", "Status" }
                            }
                        }
                        tbody {
                            {{
                                tasks.read().iter().map(|task| {
                                    rsx! {
                                        tr {
                                            td { "{task.description}" }
                                            td {
                                                select {
                                                    onchange: move |_evt| {},
                                                    option { value: "", "Unassigned" }
                                                    {{
                                                        users.read().iter().map(|employee| {
                                                            rsx! {
                                                                option {
                                                                    value: "{employee.id()}",
                                                                    selected: "{task.assignees().contains(&employee.id)}",
                                                                    "{employee.name()}"
                                                                }
                                                            }
                                                        })
                                                    }}
                                                }
                                            }
                                            td {
                                                input {
                                                    r#type: "checkbox",
                                                    checked: "{task.completed}",
                                                    onchange: move |_| {
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                            }}

                        }
                    }
                }
            }
        }

    }
}
