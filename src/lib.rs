//! # libweb
//!
//! Implements the web API with fallbacks for desktop applications.
#![deny(missing_docs)]
#![deny(warnings)]

pub mod console;

/// The `Error` type used throughout the crate.
pub type Error = failure::Error;
/// The `Result` type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
