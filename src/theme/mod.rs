/*
    Appellation: theme <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The theme module provides the necessary tools to create and manage themes.
//! 
//! 
#[doc(inline)]
pub use self::interface::Theme;

pub mod config;

mod interface;

pub mod wraps {

    wrapper!(BackgroundColor(String));
}