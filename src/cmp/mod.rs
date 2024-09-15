/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Components (cmp)
#[doc(inline)]
pub use self::{
    layout::Scaffold,
    nav::{AppBar, Navbar},
};

pub mod layout;
pub mod nav;

pub(crate) mod prelude {
    pub use super::layout::prelude::*;
    pub use super::nav::prelude::*;
}
