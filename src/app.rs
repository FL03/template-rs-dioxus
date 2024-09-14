/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{ctx::Context, settings::*};

mod ctx;
mod settings;

use crate::cmp::Navbar;
use crate::routes::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(Context::default()));
    use_context_provider(|| Signal::new(crate::SAMPLE_PROFILES.clone()));
    use_context_provider(|| Signal::new(crate::SAMPLE_TASKS.clone()));

    rsx! {
        div {
            class: "flex min-h-screen min-w-screen m-0 p-0 shadow text-dark bg-zinc-300 dark:bg-zinc-900 dark:text-white",
            
            div {
                class: "container mx-auto ",
                Navbar {
                    links: vec![
                        rsx! { Link { to: Route::Home {}, "Home" } },
                        rsx! { Link { to: Route::Tasks {}, "Tasks" } },
                    ],
                    title: "Dioxus"
                }
                Router::<Route> {}
            }
        }
    }
}
