/*
    Appellation: maps <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
//! Maps: A module dedicated to the Google Maps Platform
//!
//!
#[cfg(feature = "wasm")]
pub use self::wasm::*;



#[cfg(feature = "wasm")]
mod wasm {
    use crate::JsResult;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    #[wasm_bindgen]
    pub fn init_map() {
        
    }
}