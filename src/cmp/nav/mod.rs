/*
    Appellation: nav <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

mod appbar;
mod navbar;
mod sidebar;
mod toolbar;

pub(crate) mod prelude {
    pub use super::appbar::AppBar;
    pub use super::navbar::Navbar;
    pub use super::sidebar::Sidebar;
    pub use super::toolbar::Toolbar;
}
