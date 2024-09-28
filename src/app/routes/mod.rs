/*
    Appellation: screens <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    dash::{Dashboard, DashboardCtx, DashboardScaffold},
    errors::*,
    home::Homepage,
    profile::prelude::*,
    settings::Settings,
    tasks::Tasks,
};

mod dash;
mod home;
mod profile;
mod settings;
mod tasks;

mod errors {
    #[doc(inline)]
    pub use self::page_not_found::PageNotFound;

    mod page_not_found;
}

pub(crate) mod prelude {
    pub use super::dash::{Dashboard, DashboardScaffold};
    pub use super::errors::*;
    pub use super::home::Homepage;
    pub use super::profile::ProfilePage;
    pub use super::settings::Settings;
    pub use super::tasks::Tasks;
    pub use super::Route;
}

use crate::cmp::Navbar;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    /// The layout for most of the platform
    #[layout(Navbar)]
        #[route("/")]
        Homepage {},
        #[route("/settings")]
        Settings {},
        #[route("/tasks")]
        Tasks {},
    #[end_layout]
    /// The dashboard
    #[route("/dashboard")]
    Dashboard {},
    #[route("/profile/:uid")]
    ProfilePage {
        uid: String,
    },
    /// The 404 page
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
