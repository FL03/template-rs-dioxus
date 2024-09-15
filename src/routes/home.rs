/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            class: "flex flex-col centered",
            div {
                class: "rounded bg-zinc-700",
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    }
}
