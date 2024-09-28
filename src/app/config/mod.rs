/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::settings::Settings;

mod settings;

pub(crate) mod prelude {
    pub use super::settings::*;
}
