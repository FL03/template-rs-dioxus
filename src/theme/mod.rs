/*
    Appellation: theme <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The theme module provides the necessary tools to create and manage themes.
//!
//!
#[doc(inline)]
pub use self::{
    color::{Color, Colors},
    interface::Theme,
};

pub mod color;
pub mod config;

mod interface;

pub mod wraps {

    wrapper!(BackgroundColor(String));
}

pub(crate) mod prelude {
    pub use super::color::{colors::*, Color};
    pub use super::interface::Theme;
}
