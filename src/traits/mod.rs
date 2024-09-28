/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Traits defining shared behavior
//!
//!
#[doc(inline)]
pub use self::wrapper::Wrapper;

pub mod wrapper;

pub(crate) mod prelude {
    pub use super::wrapper::Wrapper;
}

pub trait Entry {
    type Node;
    type Weight;
}
