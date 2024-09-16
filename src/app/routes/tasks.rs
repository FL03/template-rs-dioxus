/*
    Appellation: blog <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::{Profile, Task};
use dioxus::prelude::*;

#[component]
pub fn Tasks() -> Element {
    let date = use_context::<Signal<crate::Timestamp>>();

    let users = use_context::<Signal<Vec<Profile>>>();
    let tasks = use_context::<Signal<Vec<Task>>>();

    rsx! {
        div {
            header {
                div { class: "mb-6 p-4 dark:text-dark",
                    h1 { class: "text-2xl font-bold", "Tasks" }
                    p { class: "text-sm text-gray-600", "{date.read()}" }
                }
            }
            main { class: "flex flex-col p-4 bg-white dark:bg-white rounded-lg shadow-sm",
                div { class: "grid gap-6 md:grid-cols-2",
                    div { class: "bg-zinc-800 shadow rounded-lg p-4",
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
                    div { class: "flex flex-col bg-zinc-800 shadow rounded-lg p-4",
                        h2 { class: "text-xl font-semibold mb-4", "Tasks" }
                        table { class: "flex flex-col",
                            thead {
                                tr {
                                    th { class: "text-left", "Task" }
                                    th { class: "text-left", "Assignees" }
                                    th { class: "text-left", "Status" }
                                }
                            }
                            tbody { class: "block w-full",
                                {{
                                    tasks.read().iter().map(|task| {
                                        rsx! {
                                            tr {
                                                td { "{task.name}" }
                                                td {
                                                    select {
                                                        class: "flex flex-nowrap p-2.5 bg-zinc-700 border border-zinc-400 text-zinc-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
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
                                                        onchange: move |_| {}
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
}
