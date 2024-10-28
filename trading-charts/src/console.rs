use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[allow(dead_code)]
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);

    #[allow(dead_code)]
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn error(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::console::log(&format_args!($($t)*).to_string()))
}

#[allow(unused_macros)]
macro_rules! console_error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::console::error(&format_args!($($t)*).to_string()))
}
