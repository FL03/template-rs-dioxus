/*
    Appellation: screens <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{home::Home, tasks::*};

mod home;
mod tasks;

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/tasks")]
    Tasks {},
}

pub(crate) mod prelude {
    pub use super::home::Home;
    pub use super::tasks::Tasks;
    pub use super::Route;
}
