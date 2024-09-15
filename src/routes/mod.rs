/*
    Appellation: screens <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{errors::*, home::Home, tasks::*};

mod home;
mod tasks;

mod errors {
    pub use self::page_not_found::PageNotFound;

    mod page_not_found;
}

use crate::cmp::Navbar;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/tasks")]
        Tasks {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

pub(crate) mod prelude {
    pub use super::home::Home;
    pub use super::tasks::Tasks;
    pub use super::Route;
}
