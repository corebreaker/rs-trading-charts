use js_sys::Reflect;
use chrono::ParseError;
use gloo_console::error;
use serde_wasm_bindgen::Error as SerdeError;
use wasm_bindgen::JsValue;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    error::Error as StdError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsError {
    message: String,
}

impl JsError {
    pub fn new(js: JsValue) -> Self {
        let msg_prop_name = JsValue::from_str("message");
        let msg_prop_value = Reflect::get(&js, &msg_prop_name);
        let message = match msg_prop_value.as_ref() {
            Ok(msg) => msg,
            Err(_) => &js,
        };

        Self {
            message: message.as_string().unwrap_or_else(|| format!("{message:?}")),
        }
    }

    pub fn new_from_str(message: impl AsRef<str>) -> Self {
        Self {
            message: message.as_ref().to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn log(&self) {
        error!(&self.message);
    }
}

impl Display for JsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <String as Display>::fmt(&self.message, f)
    }
}

impl StdError for JsError {}

impl From<JsValue> for JsError {
    fn from(js: JsValue) -> Self {
        Self::new(js)
    }
}

impl From<SerdeError> for JsError {
    fn from(err: SerdeError) -> Self {
        Self {
            message: format!("JSON error: {err}"),
        }
    }
}

impl From<ParseError> for JsError {
    fn from(err: ParseError) -> Self {
        Self {
            message: format!("Parse error: {err}"),
        }
    }
}
