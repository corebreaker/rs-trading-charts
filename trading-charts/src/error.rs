use js_sys::Reflect;
use chrono::ParseError;
use gloo_console::externs::error as console_error;
use serde_wasm_bindgen::{Error as SerdeError, to_value};
use wasm_bindgen::JsValue;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    error::Error as StdError,
};

#[derive(Debug, Clone)]
pub struct JsError {
    message: String,
    prefix:  Option<String>,
    data:    Option<JsValue>,
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
            prefix:  None,
            data:    None,
        }
    }

    pub fn new_from_str(message: impl AsRef<str>) -> Self {
        Self {
            message: message.as_ref().to_string(),
            prefix:  None,
            data:    None,
        }
    }

    pub fn with_prefix(self, prefix: impl AsRef<str>) -> Self {
        Self {
            prefix: Some(prefix.as_ref().to_string()),
            ..self
        }
    }

    pub fn with_data(self, data: JsValue) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }

    pub fn with_serializable_data(self, data: impl serde::Serialize) -> Self {
        let data =
            to_value(&data).unwrap_or_else(|err| JsValue::from_str(&format!("Serialization error on data: {err}")));

        self.with_data(data)
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn log(&self) {
        let msg = match &self.prefix {
            None => JsValue::from_str(&self.message),
            Some(prefix) => JsValue::from_str(&format!("{prefix}: {}", self.message)),
        };

        let mut args = vec![msg];
        if let Some(data) = &self.data {
            args.push(JsValue::from_str(" - with data"));
            args.push(data.clone());
        };

        console_error(args.into_boxed_slice());
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
            prefix:  None,
            data:    None,
        }
    }
}

impl From<ParseError> for JsError {
    fn from(err: ParseError) -> Self {
        Self {
            message: format!("Parse error: {err}"),
            prefix:  None,
            data:    None,
        }
    }
}
