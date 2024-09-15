/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{application::App, cnf::*, ctx::Context};

pub(crate) mod application;
pub mod cnf;
pub mod ctx;

pub(crate) mod prelude {
    pub use super::application::App;
    pub use super::cnf::prelude::*;
    pub use super::ctx::Context;
}
