/*
    Appellation: curiosity <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{app::*, primitives::*, utils::*};

mod app;
mod utils;

pub mod actors;
pub mod components;

mod primitives {
    use wasm_bindgen::prelude::JsError;

    /// Type alias for a [Result] of type T and error [JsError]
    pub type JsResult<T = ()> = Result<T, JsError>;
}
