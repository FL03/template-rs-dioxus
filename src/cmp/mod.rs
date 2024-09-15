/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Components (cmp)
#[doc(inline)]
pub use self::nav::*;

pub mod nav {
    #[doc(inline)]
    pub use self::{appbar::AppBar, navbar::Navbar};

    mod appbar;
    mod navbar;
}

pub(crate) mod prelude {
    pub use super::nav::*;
}
