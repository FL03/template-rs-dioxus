/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, settings::Settings};

mod settings;

pub mod kinds {
    #[doc(inline)]
    pub use self::account::AccountCnf;

    mod account;
}

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::settings::*;
}
