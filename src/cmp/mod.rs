/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Components (cmp)
#[doc(inline)]
pub use self::navbar::Navbar;

pub mod navbar;

pub(crate) mod prelude {
    pub use super::navbar::Navbar;
}
