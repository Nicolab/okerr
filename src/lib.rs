//! # Ergonomic result / error handling helpers built on anyhow and thiserror.
//!
//! Related docs:
//! - [Github: okerr](https://github.com/nicolab/okerr)
//! - [Docs.rs: anyhow](https://docs.rs/anyhow/latest/anyhow/)
//! - [Docs.rs: thiserror](https://docs.rs/thiserror/latest/thiserror/)
//!
//! This crate is a re-export of `anyhow` and `thiserror`.
//!
//! - It provides a `Result` type that is `anyhow::Result`
//! and `okerr::derive::Error` is a re-export of `thiserror::Error`.
//! - It also provides a `err!` macro that is a shorthand for `Err(anyhow::anyhow!(...))` or `Err(okerr::anyerr!(...))`.
//! - It also provides a `fail!` macro that is `anyhow::bail!`.
//! - It also provides a `anyerr!` macro that is `anyhow::anyhow!`.
//!
//! # Example
//!
//! ```
//! use okerr::{Result, err, fail, anyerr};
//!
//! fn divide(a: i32, b: i32) -> Result<i32> {
//!     if b == 0 {
//!         err!("Cannot divide by zero")
//!     } else {
//!         Ok(a / b)
//!     }
//! }
//!
//! fn maybe_fail(should_fail: bool) -> Result<String> {
//!   if should_fail {
//!     fail!("Oops!");
//!   }
//!
//!   Ok("No error".to_string())
//! }
//!
//! fn main() {
//!     let result = divide(10, 2);
//!     assert!(result.is_ok());
//!     assert_eq!(result.unwrap(), 5);
//!
//!     let result = divide(10, 0);
//!     assert!(result.is_err());
//!     assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
//!
//!     // fail! does early return
//!     let result = maybe_fail(true);
//!     assert!(result.is_err());
//!     assert_eq!(result.unwrap_err().to_string(), "Oops!");
//!
//!     // No error
//!     let result = maybe_fail(false);
//!     assert!(result.is_ok());
//!     assert_eq!(result.unwrap(), "No error");
//!
//!     // Same as anyhow!(...).
//!     // Creates an Error directly,
//!     // from a string or any std::error::Error
//!     let error = anyerr!("Oops!");
//!     assert_eq!(error.to_string(), "Oops!");
//! }
//! ```
//!
//! # Example with `okerr::derive::Error`
//!
//! ```
//! use okerr::{Result, err, derive::Error};
//!
//! #[derive(Error, Debug)]
//! enum MyError {
//!     #[error("Cannot divide by zero")]
//!     DivideByZero,
//!     #[error("Cannot divide by {0}")]
//!     DivideBy(i32),
//! }
//!
//! fn divide(a: i32, b: i32) -> Result<i32> {
//!     if b == 0 {
//!         err!(MyError::DivideByZero)
//!     } else if b < 0 {
//!         err!(MyError::DivideBy(b))
//!     } else {
//!         Ok(a / b)
//!     }
//! }
//!
//! fn main() {
//!     let result = divide(10, 2);
//!     assert!(result.is_ok());
//!     assert_eq!(result.unwrap(), 5);
//!
//!     let result = divide(10, 0);
//!     assert!(result.is_err());
//!     assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
//!
//!     let result = divide(10, -2);
//!     assert!(result.is_err());
//!     assert_eq!(result.unwrap_err().to_string(), "Cannot divide by -2");
//! }
//! ```
//!
//! It's very simple, very lightweight
//! (about ten lines of codes in the `okerr` crate),
//! it provides consistency and a better DX. 100% compatible with `anyhow` and `thiserror`, convert easily error from a boxed error (like eyre::Report and others).
pub use anyhow::*;

/// Sugar for re-exporting thiserror::Error.
/// `okerr::derive::Error` is a re-export of `thiserror::Error`.
/// - https://docs.rs/thiserror/latest/thiserror/
pub mod derive {
    // Re-export thiserror::Error
    pub use thiserror::Error;
}

/// Same as `anyhow!`.
/// - [Docs.rs: macro anyhow!](https://docs.rs/anyhow/latest/anyhow/macro.anyhow.html)
#[macro_export]
macro_rules! anyerr {
    ($($tt:tt)*) => { anyhow::anyhow!($($tt)*) };
}

/// Shorthand for `Err(anyerr!(...))` or `Err(anyhow!(...))`.
/// - [Docs.rs: macro anyhow!](https://docs.rs/anyhow/latest/anyhow/macro.anyhow.html)
#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(anyhow::anyhow!($($tt)*)) };
}

/// Same as `anyhow::bail!`.
/// - [Docs.rs: macro bail!](https://docs.rs/anyhow/latest/anyhow/macro.bail.html)
#[macro_export]
macro_rules! fail {
    ($($tt:tt)*) => { anyhow::bail!($($tt)*) };
}

/// Convert a boxed error into an okerr/anyhow Error.
///
/// # Example:
/// ```
/// use okerr::{Result, from_boxed_error};
///
/// fn returns_eyre_error() -> eyre::Result<i32> {
///     Err(eyre::eyre!("eyre error"))
/// }
///
/// fn convert_eyre() -> Result<i32> {
///     returns_eyre_error().map_err(|e| from_boxed_error(e.into()))
/// }
///
/// let result = convert_eyre();
/// assert!(result.is_err());
/// assert!(result.unwrap_err().to_string().contains("eyre error"));
/// ```
pub fn from_boxed_error(
    boxed_err: Box<dyn std::error::Error + Send + Sync + 'static>,
) -> crate::Error {
    crate::Error::from_boxed(boxed_err)
}

/// Wrap a Result into an okerr/anyhow Error.
///
/// Equivalent to `result.map_err(okerr::Error::new)`
pub fn wrap_err<T, E: std::error::Error + Send + Sync + 'static>(
    result: Result<T, E>,
) -> Result<T> {
    result.map_err(crate::Error::new)
}
