//! # libweb
//!
//! Implements the web API with fallbacks for desktop applications.
#![deny(missing_docs)]
#![deny(warnings)]

pub mod console;
pub mod websocket;

/// The `Error` type used throughout the crate.
pub type Error = failure::Error;
/// The `Result` type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(target_arch = "wasm32")]
fn js_value_to_error(value: wasm_bindgen::JsValue) -> Error {
    failure::format_err!("{:?}", value)
}
