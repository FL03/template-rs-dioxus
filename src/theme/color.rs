/*
    Appellation: colour <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[doc(inline)]
pub use self::colors::*;

mod colors;

pub struct Color<T> {
    pub node: Colors,
    pub weight: T,
}
