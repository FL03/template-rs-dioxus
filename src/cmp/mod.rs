/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Components (cmp)
#[doc(inline)]
pub use self::{
    badge::Badge,
    card::Card,
    icons::prelude::*,
    layout::Scaffold,
    nav::{AppBar, Navbar},
};

pub mod badge;
pub mod card;
pub mod icons;
pub mod layout;
pub mod nav;

pub(crate) mod prelude {
    pub use super::badge::*;
    pub use super::card::*;
    pub use super::icons::prelude::*;
    pub use super::layout::prelude::*;
    pub use super::nav::prelude::*;
}
