/*
    Appellation: auth <firebase>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#[cfg(target_arch = "wasm32-unknown-unknown")]
pub use self::wasm::*;

pub const FIREBASE_BASE_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:";

pub struct FirebaseAuth {
    pub api_key: String,
    pub refresh_token: String,
    pub id_token: String,
    pub local_id: String,
    pub expires_in: String,
}

#[cfg(target_arch = "wasm32-unknown-unknown")]
mod wasm {
    use crate::core::firebase::FirebaseApp;
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "firebase/auth")]
    extern {
        
        type Auth;
        type AuthProvider;

        type GoogleAuthProvider;


        #[wasm_bindgen(constructor)]
        fn getAuth(app: &FirebaseApp) -> Auth;

        #[wasm_bindgen(method, js_name = "signInWithEmailAndPassword")]
        fn signInWithEmailAndPassword(this: &FirebaseApp, email: String, password: String) -> Promise;

        #[wasm_bindgen(method, js_name = "createUserWithEmailAndPassword")]
        fn createUserWithEmailAndPassword(this: &FirebaseApp, email: String, password: String) -> Promise;

        #[wasm_bindgen(method, js_name = "sendPasswordResetEmail")]
        fn sendPasswordResetEmail(this: &FirebaseApp, email: String) -> Promise;

        #[wasm_bindgen(method, js_name = "confirmPasswordReset")]
        fn confirmPasswordReset(this: &FirebaseApp, code: String, newPassword: String) -> Promise;

        #[wasm_bindgen(method, js_name = "signOut")]
        fn signOut(this: &FirebaseApp) -> Promise;

        #[wasm_bindgen(method, js_name = "signInWithPopup")]
        fn signInWithPopup(this: &FirebaseApp, provider: AuthProvider) -> Promise;
    }
}