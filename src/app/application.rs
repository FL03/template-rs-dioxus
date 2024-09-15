/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use crate::app::routes::Route;
use crate::{Timestamp, SAMPLE_PROFILES, SAMPLE_TASKS};
use dioxus::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(Settings::default()));
    use_context_provider(|| Signal::new(Timestamp::now()));
    use_context_provider(|| Signal::new(SAMPLE_PROFILES.clone()));
    use_context_provider(|| Signal::new(SAMPLE_TASKS.clone()));

    rsx! {
        div {
            class: "flex min-h-screen min-w-screen m-0 p-0 text-dark bg-zinc-300 dark:bg-zinc-900 dark:text-white",
            div {
                class: "container mx-auto",
                Router::<Route> {}
            }
        }
    }
}
