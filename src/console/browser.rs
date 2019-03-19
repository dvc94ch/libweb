#[macro_export]
macro_rules! console_log {
    ($($arg: tt)*) => (
        let format = format!($($arg)*);
        let value = wasm_bindgen::JsValue::from(format.as_str());
        web_sys::console::log_1(&value);
    )
}

#[macro_export]
macro_rules! console_error {
    ($($arg: tt)*) => (
        let format = format!($($arg)*);
        let value = wasm_bindgen::JsValue::from(format.as_str());
        web_sys::console::error_1(&value);
    )
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_console_log() {
        let world = "world";
        console_log!("hello\n{}", world);
    }

    #[wasm_bindgen_test]
    fn test_console_error() {
        let world = "world";
        console_error!("hello\n{}", world);
    }
}
