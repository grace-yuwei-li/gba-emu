use std::fmt::Display;

use wasm_bindgen::JsValue;

pub trait ToJsResult<T> {
    fn to_js_result(self) -> Result<T, JsValue>;
}

pub trait OptionToJsResult<T> {
    fn to_js_result(self, msg: &str) -> Result<T, JsValue>;
}

impl<T, E: Display> ToJsResult<T> for Result<T, E> {
    fn to_js_result(self) -> Result<T, JsValue> {
        self.map_err(|e| JsValue::from(e.to_string()))
    }
}

impl<T> OptionToJsResult<T> for Option<T> {
    fn to_js_result(self, msg: &str) -> Result<T, JsValue> {
        self.ok_or_else(|| JsValue::from_str(msg))
    }
}
