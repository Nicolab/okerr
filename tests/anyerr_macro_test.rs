//! Tests for anyerr! macro (alias for anyhow!)

use okerr::{Result, anyerr, derive::Error};

#[derive(Error, Debug)]
#[error("custom error: {0}")]
struct CustomError(String);

#[test]
fn anyerr_macro_creates_error() {
    let error = anyerr!("simple error");
    assert_eq!(error.to_string(), "simple error");
}

#[test]
fn anyerr_macro_with_format() {
    let value = 42;
    let error = anyerr!("error with value: {}", value);
    assert_eq!(error.to_string(), "error with value: 42");
}

#[test]
fn anyerr_macro_with_multiple_args() {
    let a = 10;
    let b = 20;
    let error = anyerr!("values: {} and {}", a, b);
    assert_eq!(error.to_string(), "values: 10 and 20");
}

#[test]
fn anyerr_macro_with_custom_error() {
    let custom = CustomError("test".to_string());
    let error = anyerr!(custom);
    assert!(error.to_string().contains("custom error: test"));
}

#[test]
fn anyerr_macro_in_result_context() {
    fn operation(should_fail: bool) -> Result<i32> {
        if should_fail {
            Err(anyerr!("operation failed"))
        } else {
            Ok(42)
        }
    }

    let result = operation(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);

    let result = operation(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "operation failed");
}

#[test]
fn anyerr_macro_converts_to_anyhow_error() {
    let error = anyerr!("test");
    let _: okerr::Error = error;
}

#[test]
fn anyerr_macro_with_named_args() {
    let name = "Alice";
    let age = 30;
    let error = anyerr!("User {name} is {age} years old", name = name, age = age);
    assert_eq!(error.to_string(), "User Alice is 30 years old");
}

#[test]
fn anyerr_vs_err_macro() {
    // anyerr! creates an Error directly
    let anyerr_result = anyerr!("using anyerr");

    // err! creates Err(...)
    let err_result: Result<()> = okerr::err!("using err");

    // anyerr creates Error
    let _: okerr::Error = anyerr_result;

    // err creates Result
    assert!(err_result.is_err());
}

#[test]
fn anyerr_macro_with_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file.txt");
    let error = anyerr!(io_err);
    assert!(error.to_string().contains("file.txt"));
}

#[test]
fn anyerr_macro_chaining() {
    fn inner() -> std::io::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "inner"))
    }

    fn outer() -> Result<()> {
        inner().map_err(|e| anyerr!(e).context("outer context"))?;
        Ok(())
    }

    let result = outer();
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("outer context"));
}

#[test]
fn anyerr_is_exactly_anyhow() {
    // anyerr! should behave exactly like anyhow!
    let anyerr_error = anyerr!("test message");
    let anyhow_error = okerr::anyhow!("test message");

    // Both should produce the same output
    assert_eq!(anyerr_error.to_string(), anyhow_error.to_string());
}

#[test]
fn anyerr_with_context() {
    let error = anyerr!("base error").context("additional context");
    let chain: Vec<_> = error.chain().map(|e| e.to_string()).collect();
    assert!(chain.len() >= 1);
}

#[test]
fn anyerr_macro_no_conflict_with_variable() {
    // Verify there's no conflict with a variable named anyerr
    let anyerr = "this is a variable";

    // Use the anyerr! macro
    let error_result = okerr::anyerr!("this is from the macro");

    // Verify the variable is still accessible
    assert_eq!(anyerr, "this is a variable");

    // Verify the macro produced an error
    assert_eq!(error_result.to_string(), "this is from the macro");
}

#[test]
fn anyerr_wraps_custom_error() {
    #[derive(Error, Debug)]
    #[error("typed error")]
    struct TypedError;

    let typed = TypedError;
    let error = anyerr!(typed);

    // Verify it's a valid okerr::Error
    let _: okerr::Error = error;
}
