/*
    Appellation: home <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::scaffold::DashboardScaffold;

mod scaffold;

#[doc(hidden)]
pub(crate) mod prev_dash;

use crate::cmp::badge::GrayBadge as Badge;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {

    }
}

pub struct ProfileRef;

pub struct CurrentUser {
    pub username: String,
    pub uid: String,
}

pub struct DashboardCtx {
    
}
