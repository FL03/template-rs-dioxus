/*
    Appellation: nav <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{appbar::AppBar, navbar::Navbar};

mod appbar;
mod navbar;

pub(crate) mod prelude {
    pub use super::appbar::AppBar;
    pub use super::navbar::Navbar;
}
