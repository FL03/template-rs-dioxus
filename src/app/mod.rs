/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{cnf::*, ctx::Context, routes::*, state::AppState};

pub mod cnf;
pub mod ctx;
pub mod routes;
pub mod state;
pub mod theme;

pub(crate) mod prelude {
    pub use super::cnf::*;
    pub use super::ctx::Context;
    pub use super::routes::prelude::*;
    pub use super::state::*;
    pub use super::App;
}

use crate::data::samples::*;
use crate::types::Timestamp;
use dioxus::prelude::*;

pub fn App(state: AppState) -> Element {
    use_context_provider(|| Signal::new(state));
    use_context_provider(|| Signal::new(Settings::default()));
    use_context_provider(|| Signal::new(Timestamp::now()));
    use_context_provider(|| Signal::new(SAMPLE_PROFILES.clone()));
    use_context_provider(|| Signal::new(SAMPLE_TASKS.clone()));

    rsx! {
        div { class: "flex min-h-screen min-w-screen m-0 p-0 text-dark bg-zinc-300 dark:bg-zinc-900 dark:text-white",
            div { class: "container mx-auto", Router::<Route> {} }
        }
    }
}

pub fn run() -> Element {
    let state = AppState::new();
    App(state)
}
