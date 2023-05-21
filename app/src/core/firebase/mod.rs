/*
    Appellation: firebase <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#[cfg(target_arch = "wasm32-unknown-unknown")]
pub use self::wasm::*;

pub mod auth;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub app_id: String,
    pub auth_domain: String,
    pub database_url: String,
    pub messaging_sender_id: String,
    pub project_id: String,
    pub storage_bucket: String,
}

impl FirebaseConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("FIREBASE_API_KEY").unwrap_or_default(),
            app_id: std::env::var("FIREBASE_APP_ID").unwrap_or_default(),
            auth_domain: std::env::var("FIREBASE_AUTH_DOMAIN").unwrap_or_default(),
            database_url: std::env::var("FIREBASE_DATABASE_URL").unwrap_or_default(),
            messaging_sender_id: std::env::var("FIREBASE_MESSAGING_SENDER_ID").unwrap_or_default(),
            project_id: std::env::var("FIREBASE_PID").unwrap_or_default(),
            storage_bucket: std::env::var("FIREBASE_STORAGE_BUCKET").unwrap_or_default(),
        }
    }
    pub fn set_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }
    pub fn set_pid(mut self, pid: String) -> Self {
        self.project_id = pid;
        self
    }
}

#[cfg(target_arch = "wasm32-unknown-unknown")]
impl From<FirebaseConfig> for wasm_bindgen::prelude::JsValue {
    fn from(config: FirebaseConfig) -> Self {
        let config = js_sys::Object::new();
        js_sys::Reflect::set(&config, &JsValue::from_str("apiKey"), &JsValue::from_str(&config.api_key)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("appId"), &JsValue::from_str(&config.app_id)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("authDomain"), &JsValue::from_str(&config.auth_domain)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("databaseURL"), &JsValue::from_str(&config.database_url)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("messagingSenderId"), &JsValue::from_str(&config.messaging_sender_id)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("projectId"), &JsValue::from_str(&config.project_id)).unwrap();
        js_sys::Reflect::set(&config, &JsValue::from_str("storageBucket"), &JsValue::from_str(&config.storage_bucket)).unwrap();
        config.into()
    }
}

#[cfg(target_arch = "wasm32-unknown-unknown")]
mod wasm {
    use js_sys::Reflect;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "firebase/app")]
    extern {

        type FirebaseApp;

        #[wasm_bindgen(constructor, js_name = "initializeApp")]
        fn initialize_app(config: JsValue) -> FirebaseApp;
    }

    #[wasm_bindgen]
    pub fn init_firebase() -> FirebaseApp {
        let config: JsValue = super::FirebaseConfig::from_env().into();
        initialize_app(config)
    }
}
