/*
    Appellation: profile <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::screen::ProfilePage;

mod screen;

pub mod cmp {
    #[doc(inline)]
    pub use self::card::ProfileCard;

    mod card;
}

pub(crate) mod prelude {
    pub use super::cmp::ProfileCard;
    pub use super::screen::ProfilePage;
}
