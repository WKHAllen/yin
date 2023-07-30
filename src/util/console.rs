/// Logs to the console.
#[allow(unused_macros)]
macro_rules! console_log {
    ( $($arg:tt)* ) => {{
        ::web_sys::console::log_1(&::wasm_bindgen::JsValue::from_str(&::std::format!($($arg)*)));
    }};
}

#[allow(unused_imports)]
pub(self) use console_log;
