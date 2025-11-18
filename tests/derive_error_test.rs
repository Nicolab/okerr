//! Tests for okerr::derive::Error (re-export of thiserror::Error)

use okerr::{Result, derive::Error, err};

/// Simple error enumeration for testing
#[derive(Error, Debug)]
enum SimpleError {
    #[error("simple error occurred")]
    Simple,
    #[error("error with value: {0}")]
    WithValue(i32),
    #[error("error with named field: {field}")]
    WithNamedField { field: String },
}

/// Inner error for testing #[from] attribute
#[derive(Error, Debug)]
#[error("inner error: {0}")]
struct InnerError(String);

/// Outer error with automatic conversion
#[derive(Error, Debug)]
enum OuterError {
    #[error("outer error")]
    Outer(#[from] InnerError),
    #[error("io error")]
    Io(#[from] std::io::Error),
}

#[test]
fn derive_error_simple_variant() {
    let error = SimpleError::Simple;
    assert_eq!(error.to_string(), "simple error occurred");
}

#[test]
fn derive_error_with_value() {
    let error = SimpleError::WithValue(42);
    assert_eq!(error.to_string(), "error with value: 42");
}

#[test]
fn derive_error_with_named_field() {
    let error = SimpleError::WithNamedField {
        field: "test".to_string(),
    };
    assert_eq!(error.to_string(), "error with named field: test");
}

#[test]
fn derive_error_in_result() {
    fn divide(a: i32, b: i32) -> Result<i32> {
        if b == 0 {
            err!(SimpleError::Simple)
        } else if b < 0 {
            err!(SimpleError::WithValue(b))
        } else {
            Ok(a / b)
        }
    }

    let result = divide(10, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let result = divide(10, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "simple error occurred");

    let result = divide(10, -2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "error with value: -2");
}

#[test]
fn derive_error_from_attribute() {
    let inner = InnerError("test".to_string());
    let outer: OuterError = inner.into();

    match outer {
        OuterError::Outer(_) => {
            assert_eq!(outer.to_string(), "outer error");
        }
        _ => panic!("Expected OuterError::Outer"),
    }
}

#[test]
fn derive_error_from_std_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let outer: OuterError = io_err.into();

    match outer {
        OuterError::Io(_) => {
            assert_eq!(outer.to_string(), "io error");
        }
        _ => panic!("Expected OuterError::Io"),
    }
}

#[test]
fn derive_error_implements_std_error() {
    let error = SimpleError::Simple;
    let _: &dyn std::error::Error = &error;
}

#[test]
fn derive_error_debug_format() {
    let error = SimpleError::WithValue(42);
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("WithValue"));
    assert!(debug_str.contains("42"));
}
