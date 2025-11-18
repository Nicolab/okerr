//! Tests for the err! macro

use okerr::{Result, derive::Error, err};

#[derive(Error, Debug)]
#[error("custom error: {0}")]
struct CustomError(String);

#[test]
fn err_macro_with_string() {
    let result: Result<(), _> = err!("simple error message");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "simple error message");
}

#[test]
fn err_macro_with_format() {
    let value = 42;
    let result: Result<(), _> = err!("error with value: {}", value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "error with value: 42");
}

#[test]
fn err_macro_with_multiple_format_args() {
    let a = 10;
    let b = 20;
    let result: Result<(), _> = err!("values: {} and {}", a, b);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "values: 10 and 20");
}

#[test]
fn err_macro_with_custom_error() {
    fn returns_custom_error() -> Result<()> {
        err!(CustomError("test".to_string()))
    }

    let result = returns_custom_error();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "custom error: test");
}

#[test]
fn err_macro_in_result() {
    fn divide(a: i32, b: i32) -> Result<i32> {
        if b == 0 {
            err!("cannot divide by zero")
        } else {
            Ok(a / b)
        }
    }

    let result = divide(10, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let result = divide(10, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "cannot divide by zero");
}

/// Verify there's no conflict between a variable named `err` and the `err!` macro
#[test]
fn no_conflict_between_variable_and_macro() {
    // Define a variable named `err`
    let err = "this is a variable";

    // Use the err! macro
    let error_result: Result<(), _> = err!("this is from the macro");

    // Verify the variable is still accessible and unchanged
    assert_eq!(err, "this is a variable");

    // Verify the macro produced the expected error
    assert!(error_result.is_err());
    assert_eq!(
        error_result.unwrap_err().to_string(),
        "this is from the macro"
    );

    // Use the variable after the macro
    let _variable_copy = err;

    // Use the macro again
    let _another_error: Result<(), _> = err!("another macro call");
}

#[test]
fn err_macro_with_variable_named_err_in_function() {
    fn test_function() -> Result<String> {
        let err = "variable value";

        // This should not conflict
        if err.is_empty() {
            return err!("empty string");
        }

        Ok(err.to_string())
    }

    let result = test_function();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "variable value");
}

#[test]
fn err_macro_returns_result_with_anyhow_error() {
    let result: Result<(), _> = err!("test");

    // Verify it's a Result containing an anyhow::Error
    assert!(result.is_err());
    let error = result.unwrap_err();
    let _: okerr::Error = error;
}

#[test]
fn err_macro_with_named_arguments() {
    let name = "Alice";
    let age = 30;
    let result: Result<(), _> = err!("User {name} is {age} years old", name = name, age = age);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "User Alice is 30 years old"
    );
}
