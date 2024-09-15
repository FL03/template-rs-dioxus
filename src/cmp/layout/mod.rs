/*
    Appellation: layout <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::scaffold::Scaffold;

pub mod scaffold;

pub(crate) mod prelude {
    pub use super::scaffold::Scaffold;
}
