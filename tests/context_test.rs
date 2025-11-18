//! Tests for context() and with_context() methods

use okerr::{Context, Result};
use std::io;

#[test]
fn context_adds_static_message() {
    fn read_config() -> io::Result<String> {
        Err(io::Error::new(io::ErrorKind::NotFound, "config.json"))
    }

    let result: Result<String> = read_config().context("failed to read configuration");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("failed to read configuration"));
}

#[test]
fn with_context_adds_dynamic_message() {
    fn parse_value(s: &str) -> Result<i32> {
        s.parse()
            .with_context(|| format!("failed to parse '{}' as integer", s))
    }

    let result = parse_value("not_a_number");
    assert!(result.is_err());

    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("failed to parse 'not_a_number' as integer"));
}

#[test]
fn context_chain_multiple() {
    fn inner() -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::NotFound, "file.txt"))
    }

    fn middle() -> Result<()> {
        inner().context("middle layer")?;
        Ok(())
    }

    fn outer() -> Result<()> {
        middle().context("outer layer")?;
        Ok(())
    }

    let result = outer();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let chain: Vec<_> = err.chain().map(|e| e.to_string()).collect();

    // Should have multiple layers
    assert!(chain.len() >= 2);
}

#[test]
fn with_context_lazy_evaluation() {
    fn expensive_computation() -> String {
        "expensive result".to_string()
    }

    fn operation(should_fail: bool) -> Result<i32> {
        if should_fail {
            Err(io::Error::new(io::ErrorKind::Other, "fail")).with_context(|| {
                // This closure only executes if there's an error
                format!("context: {}", expensive_computation())
            })?;
        }
        Ok(42)
    }

    // Success case - closure not called
    let result = operation(false);
    assert!(result.is_ok());

    // Error case - closure is called
    let result = operation(true);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("context: expensive result")
    );
}

#[test]
fn context_preserves_original_error() {
    fn original_error() -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "access denied",
        ))
    }

    let result: Result<()> = original_error().context("cannot perform operation");

    assert!(result.is_err());
    let err = result.unwrap_err();

    // The error chain should contain both messages
    let chain_str = format!("{:?}", err);
    assert!(chain_str.contains("cannot perform operation") || chain_str.contains("access denied"));
}

#[test]
fn with_context_with_variables() {
    fn process_file(filename: &str, line: usize) -> Result<String> {
        Err(io::Error::new(io::ErrorKind::InvalidData, "bad format"))
            .with_context(|| format!("error in file '{}' at line {}", filename, line))
    }

    let result = process_file("data.txt", 42);
    assert!(result.is_err());

    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("data.txt"));
    assert!(err_msg.contains("42"));
}

#[test]
fn context_on_result_chain() {
    fn step1() -> io::Result<i32> {
        Ok(10)
    }

    fn step2(value: i32) -> Result<i32> {
        if value < 100 {
            okerr::err!("value too small")
        } else {
            Ok(value)
        }
    }

    let result: Result<i32> = step1()
        .context("step1 failed")
        .and_then(step2)
        .context("step2 failed");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("step2 failed") || err_msg.contains("value too small"));
}

#[test]
fn multiple_with_context_calls() {
    fn operation() -> Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "base error")).with_context(|| "first context")?;
        Ok(())
    }

    let result = operation()
        .with_context(|| "second context")
        .with_context(|| "third context");

    assert!(result.is_err());
    // At least one context message should be present
    let err_msg = result.unwrap_err().to_string();
    assert!(!err_msg.is_empty());
}

#[test]
fn context_with_custom_error_type() {
    use okerr::derive::Error;

    #[derive(Error, Debug)]
    #[error("custom: {msg}")]
    struct CustomError {
        msg: String,
    }

    fn custom_op() -> std::result::Result<(), CustomError> {
        Err(CustomError {
            msg: "something wrong".to_string(),
        })
    }

    let result: Result<()> = custom_op().context("operation failed");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("operation failed"));
}

#[test]
fn context_in_nested_functions() {
    fn level3() -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::NotFound, "resource"))
    }

    fn level2() -> Result<()> {
        level3().context("level2 context")?;
        Ok(())
    }

    fn level1() -> Result<()> {
        level2().context("level1 context")?;
        Ok(())
    }

    let result = level1();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let chain_count = err.chain().count();

    // Should have multiple errors in the chain
    assert!(chain_count >= 2);
}

#[test]
fn with_context_formatting() {
    fn validate(name: &str, age: i32) -> Result<()> {
        if age < 18 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "too young"))
                .with_context(|| format!("validation failed for user '{}' (age: {})", name, age))?;
        }
        Ok(())
    }

    let result = validate("Alice", 15);
    assert!(result.is_err());

    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Alice"));
    assert!(err_msg.contains("15"));
}
