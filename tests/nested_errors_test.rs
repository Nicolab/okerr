//! Tests for nested and chained errors

use okerr::{Context, Result, derive::Error};
use std::io;

#[derive(Error, Debug)]
#[error("inner error: {0}")]
struct InnerError(String);

#[derive(Error, Debug)]
#[error("middle error")]
struct MiddleError {
    #[source]
    source: InnerError,
}

#[derive(Error, Debug)]
#[error("outer error")]
struct OuterError {
    #[source]
    source: MiddleError,
}

#[test]
fn nested_errors_with_source_attribute() {
    let inner = InnerError("root cause".to_string());
    let middle = MiddleError { source: inner };
    let outer = OuterError { source: middle };

    // Can access the error as std::error::Error
    let err: &dyn std::error::Error = &outer;

    // Should have a source
    assert!(err.source().is_some());

    // Source should be MiddleError
    let middle_err = err.source().unwrap();
    assert!(middle_err.to_string().contains("middle error"));

    // MiddleError should have InnerError as source
    assert!(middle_err.source().is_some());
}

#[test]
fn error_chain_traversal() {
    let inner = InnerError("deepest error".to_string());
    let middle = MiddleError { source: inner };
    let outer = OuterError { source: middle };

    let err: &dyn std::error::Error = &outer;

    // Count errors in chain
    let mut count = 1;
    let mut current = err;

    while let Some(source) = current.source() {
        count += 1;
        current = source;
    }

    // Should have 3 levels: Outer -> Middle -> Inner
    assert_eq!(count, 3);
}

#[test]
fn nested_errors_in_result_chain() {
    fn level3() -> std::result::Result<(), InnerError> {
        Err(InnerError("level 3 failed".to_string()))
    }

    fn level2() -> std::result::Result<(), MiddleError> {
        level3().map_err(|source| MiddleError { source })
    }

    fn level1() -> std::result::Result<(), OuterError> {
        level2().map_err(|source| OuterError { source })
    }

    let result = level1();
    assert!(result.is_err());

    let err = result.unwrap_err();

    // Verify the chain exists
    let std_err: &dyn std::error::Error = &err;
    assert!(std_err.source().is_some());
}

#[test]
fn anyhow_error_chain() {
    fn inner_operation() -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::NotFound, "file.txt"))
    }

    fn middle_operation() -> Result<()> {
        inner_operation().context("failed to read file")?;
        Ok(())
    }

    fn outer_operation() -> Result<()> {
        middle_operation().context("operation failed")?;
        Ok(())
    }

    let result = outer_operation();
    assert!(result.is_err());

    let err = result.unwrap_err();

    // anyhow provides chain() iterator
    let chain: Vec<_> = err.chain().map(|e| e.to_string()).collect();

    // Should have multiple errors in the chain
    assert!(chain.len() >= 2);
}

#[test]
fn mixed_error_types_chain() {
    #[derive(Error, Debug)]
    enum AppError {
        #[error("io error")]
        Io(#[from] io::Error),
        #[error("parse error")]
        Parse(#[from] std::num::ParseIntError),
        #[error("custom error: {0}")]
        Custom(String),
    }

    fn operation() -> std::result::Result<i32, AppError> {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "config");
        Err(io_err.into())
    }

    // Use Custom variant to avoid warning
    let _custom = AppError::Custom("test".to_string());

    let result: Result<i32> = operation().map_err(|e| e.into());
    assert!(result.is_err());

    let err = result.unwrap_err();
    let chain_count = err.chain().count();
    assert!(chain_count >= 1);
}

#[test]
fn deep_nested_errors() {
    #[derive(Error, Debug)]
    #[error("level 1")]
    struct Level1(#[source] Level2);

    #[derive(Error, Debug)]
    #[error("level 2")]
    struct Level2(#[source] Level3);

    #[derive(Error, Debug)]
    #[error("level 3")]
    struct Level3(#[source] Level4);

    #[derive(Error, Debug)]
    #[error("level 4")]
    struct Level4(#[source] io::Error);

    let l4 = Level4(io::Error::new(io::ErrorKind::Other, "deepest"));
    let l3 = Level3(l4);
    let l2 = Level2(l3);
    let l1 = Level1(l2);

    let err: &dyn std::error::Error = &l1;

    // Count the chain depth
    let mut depth = 1;
    let mut current = err;

    while let Some(source) = current.source() {
        depth += 1;
        current = source;
    }

    // Should have 5 levels
    assert_eq!(depth, 5);
}

#[test]
fn error_chain_with_context() {
    fn step1() -> io::Result<String> {
        Err(io::Error::new(io::ErrorKind::NotFound, "data.txt"))
    }

    fn step2() -> Result<String> {
        step1().context("step 1 failed")?;
        Ok("success".to_string())
    }

    fn step3() -> Result<String> {
        step2().context("step 2 failed")?;
        Ok("success".to_string())
    }

    let result = step3();
    assert!(result.is_err());

    let err = result.unwrap_err();

    // Collect all messages in the chain
    let messages: Vec<_> = err.chain().map(|e| e.to_string()).collect();

    // Should have multiple messages
    assert!(messages.len() >= 2);
}

#[test]
fn preserves_root_cause() {
    let root_cause = InnerError("original problem".to_string());
    let middle = MiddleError { source: root_cause };
    let outer = OuterError { source: middle };

    let err: &dyn std::error::Error = &outer;

    // Navigate to the root cause
    let mut current = err;
    while let Some(source) = current.source() {
        current = source;
    }

    // The deepest error should contain our original message
    assert!(current.to_string().contains("original problem"));
}

#[test]
fn error_downcast_in_chain() {
    #[derive(Error, Debug)]
    #[error("wrapper")]
    struct Wrapper {
        #[source]
        inner: InnerError,
    }

    let inner = InnerError("test".to_string());
    let wrapper = Wrapper { inner };

    let boxed: Box<dyn std::error::Error> = Box::new(wrapper);

    // Should be able to check the error type
    assert!(boxed.is::<Wrapper>());
}

#[test]
fn multiple_branches_error_chain() {
    #[derive(Error, Debug)]
    enum ProcessError {
        #[error("step A failed")]
        StepA(#[source] io::Error),
        #[error("step B failed")]
        StepB(#[source] std::fmt::Error),
    }

    fn process_a() -> std::result::Result<(), ProcessError> {
        let io_err = io::Error::new(io::ErrorKind::Other, "A");
        Err(ProcessError::StepA(io_err))
    }

    // Use StepB variant to avoid warning
    let _step_b = ProcessError::StepB(std::fmt::Error);

    let result = process_a();
    assert!(result.is_err());

    let err = result.unwrap_err();
    match err {
        ProcessError::StepA(ref source) => {
            assert!(source.to_string().contains("A"));
        }
        _ => panic!("Wrong error variant"),
    }
}

#[test]
fn chain_formatting() {
    fn create_nested() -> Result<()> {
        Err(io::Error::new(io::ErrorKind::NotFound, "file"))
            .context("read failed")
            .context("operation failed")
    }

    let result = create_nested();
    assert!(result.is_err());

    let err = result.unwrap_err();

    // Format the full chain
    let debug_output = format!("{:?}", err);
    assert!(!debug_output.is_empty());

    // Display should show something
    let display_output = format!("{}", err);
    assert!(!display_output.is_empty());
}
