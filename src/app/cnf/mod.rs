/*
    Appellation: cnf <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{settings::*, theme::*};

pub(crate) mod settings;
pub(crate) mod theme;

pub(crate) mod prelude {
    pub use super::settings::*;
    pub use super::theme::*;
}
