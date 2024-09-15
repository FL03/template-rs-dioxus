/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{application::App, cnf::*, ctx::Context, routes::*};

pub(crate) mod application;

pub mod cnf;
pub mod ctx;
pub mod routes;

pub(crate) mod prelude {
    pub use super::application::App;
    pub use super::cnf::*;
    pub use super::ctx::Context;
    pub use super::routes::prelude::*;
}
