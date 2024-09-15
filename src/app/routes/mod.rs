/*
    Appellation: screens <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{dashboard::Dashboard, errors::*, settings::Settings, tasks::*};

mod dashboard;
mod settings;
mod tasks;

mod errors {
    #[doc(inline)]
    pub use self::page_not_found::PageNotFound;

    mod page_not_found;
}

pub(crate) mod prelude {
    pub use super::dashboard::Dashboard;
    pub use super::errors::*;
    pub use super::settings::Settings;
    pub use super::tasks::Tasks;
    pub use super::Route;
}

use crate::cmp::Navbar;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(Navbar)]
        #[route("/tasks")]
        Tasks {},
    #[end_layout]
    #[route("/")]
    Dashboard {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
