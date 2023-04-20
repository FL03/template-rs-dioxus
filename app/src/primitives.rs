
pub use self::{constants::*, statics::*, types::*};

mod constants {}

mod statics {}

mod types {
    use wasm_bindgen::prelude::JsError;

    /// Type alias for a [Result] of type T and error [JsError]
    pub type JsResult<T = ()> = Result<T, JsError>;
}