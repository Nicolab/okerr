//! Tests for wrap_err function (wrapping Result<T, E> into Result<T>)

use okerr::{Result, wrap_err};
use std::io;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl std::error::Error for CustomError {}

#[test]
fn wrap_err_with_io_error() {
    fn read_file() -> std::result::Result<String, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "file.txt not found",
        ))
    }

    let result: Result<String> = wrap_err(read_file());

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("file.txt not found")
    );
}

#[test]
fn wrap_err_with_custom_error() {
    fn custom_operation() -> std::result::Result<i32, CustomError> {
        Err(CustomError {
            message: "operation failed".to_string(),
        })
    }

    let result: Result<i32> = wrap_err(custom_operation());

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("CustomError: operation failed")
    );
}

#[test]
fn wrap_err_preserves_ok_value() {
    fn successful_operation() -> std::result::Result<i32, io::Error> {
        Ok(42)
    }

    let result: Result<i32> = wrap_err(successful_operation());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn wrap_err_chain_multiple() {
    fn step1() -> std::result::Result<i32, io::Error> {
        Ok(10)
    }

    fn step2(value: i32) -> std::result::Result<i32, CustomError> {
        if value < 0 {
            Err(CustomError {
                message: "negative value".to_string(),
            })
        } else {
            Ok(value * 2)
        }
    }

    let result: Result<i32> = wrap_err(step1()).and_then(|v| wrap_err(step2(v)));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20);
}

#[test]
fn wrap_err_with_fmt_error() {
    fn fmt_error() -> std::result::Result<(), std::fmt::Error> {
        Err(std::fmt::Error)
    }

    let result: Result<()> = wrap_err(fmt_error());

    assert!(result.is_err());
}

#[test]
fn wrap_err_in_function_chain() {
    fn parse_number(s: &str) -> std::result::Result<i32, std::num::ParseIntError> {
        s.parse()
    }

    fn validate_number(n: i32) -> Result<i32> {
        if n > 0 {
            Ok(n)
        } else {
            okerr::err!("number must be positive")
        }
    }

    let result: Result<i32> = wrap_err(parse_number("42")).and_then(validate_number);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);

    let result: Result<i32> = wrap_err(parse_number("invalid"));
    assert!(result.is_err());
}

#[test]
fn wrap_err_with_parse_error() {
    let result: Result<i32> = wrap_err("not_a_number".parse::<i32>());

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("invalid") || err_msg.contains("parse"));
}

#[test]
fn wrap_err_multiple_error_types() {
    fn operation1() -> std::result::Result<i32, io::Error> {
        Ok(10)
    }

    fn operation2() -> std::result::Result<i32, CustomError> {
        Ok(20)
    }

    let result1: Result<i32> = wrap_err(operation1());
    let result2: Result<i32> = wrap_err(operation2());

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap() + result2.unwrap(), 30);
}

#[test]
fn wrap_err_equivalent_to_map_err() {
    fn returns_io_error() -> std::result::Result<String, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "test"))
    }

    let result1: Result<String> = wrap_err(returns_io_error());
    let result2: Result<String> = returns_io_error().map_err(okerr::Error::new);

    assert!(result1.is_err());
    assert!(result2.is_err());
    assert_eq!(
        result1.unwrap_err().to_string(),
        result2.unwrap_err().to_string()
    );
}

#[test]
fn wrap_err_preserves_success_through_chain() -> Result<()> {
    fn operation() -> std::result::Result<i32, io::Error> {
        Ok(42)
    }

    let value = wrap_err(operation())?;
    assert_eq!(value, 42);

    Ok(())
}
