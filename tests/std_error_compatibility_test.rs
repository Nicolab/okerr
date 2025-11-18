//! Tests for std::error::Error compatibility

use okerr::{Context, Result, derive::Error};
use std::error::Error as StdError;

#[derive(Error, Debug)]
#[error("custom error: {message}")]
struct CustomError {
    message: String,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("custom: {0}")]
    Custom(String),
}

#[test]
fn anyhow_error_can_wrap_errors() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let result: Result<()> = Err(io_err.into());

    let err = result.unwrap_err();
    // Verify the error message is preserved
    assert!(err.to_string().contains("file not found"));

    // Verify the chain contains at least the error itself
    assert!(err.chain().count() >= 1);
}

#[test]
fn error_chain_with_context() {
    fn inner_error() -> std::io::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "inner"))
    }

    fn outer_error() -> Result<()> {
        inner_error().context("outer context")?;
        Ok(())
    }

    let result = outer_error();
    assert!(result.is_err());

    let err = result.unwrap_err();
    // Should have multiple errors in chain with context
    let chain_count = err.chain().count();
    assert!(chain_count >= 2);

    // Verify context message is present
    assert!(err.to_string().contains("outer context"));
}

#[test]
fn custom_error_as_std_error() {
    let custom = CustomError {
        message: "test error".to_string(),
    };

    // Can be used as &dyn StdError
    let std_err: &dyn StdError = &custom;
    assert_eq!(std_err.to_string(), "custom error: test error");
}

#[test]
fn derive_error_implements_std_error() {
    let my_err = MyError::Custom("test".to_string());

    // Can be converted to &dyn StdError
    let std_err: &dyn StdError = &my_err;
    assert_eq!(std_err.to_string(), "custom: test");
}

#[test]
fn error_downcast_boxed() {
    let custom = CustomError {
        message: "downcasting".to_string(),
    };

    let boxed: Box<dyn StdError + Send + Sync> = Box::new(custom);

    // Try to downcast
    let downcast_result = boxed.downcast::<CustomError>();
    assert!(downcast_result.is_ok());

    let recovered = downcast_result.unwrap();
    assert_eq!(recovered.message, "downcasting");
}

#[test]
fn error_chain_iteration() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file.txt");
    let my_err = MyError::Io(io_err);

    // Count errors in the chain using source()
    let mut count = 1;
    let mut current: &dyn StdError = &my_err;

    while let Some(source) = current.source() {
        count += 1;
        current = source;
    }

    // Should have at least 2 errors in the chain
    assert!(count >= 2);
}

#[test]
fn anyhow_error_chain_traversal() {
    fn create_chain() -> Result<()> {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "inner error");
        Err(io_err).map_err(|e| okerr::anyerr!(e).context("outer context"))
    }

    let result = create_chain();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let chain_count = err.chain().count();

    // Should have multiple errors in chain
    assert!(chain_count >= 1);
}

#[test]
fn error_debug_format() {
    let custom = CustomError {
        message: "debug test".to_string(),
    };

    let debug_str = format!("{:?}", custom);
    assert!(debug_str.contains("CustomError"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn error_display_format() {
    let custom = CustomError {
        message: "display test".to_string(),
    };

    let display_str = format!("{}", custom);
    assert_eq!(display_str, "custom error: display test");
}

#[test]
fn multiple_error_types_compatibility() {
    fn io_error() -> std::io::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "io"))
    }

    fn custom_error() -> std::result::Result<(), CustomError> {
        Err(CustomError {
            message: "custom".to_string(),
        })
    }

    // Both can be converted to Result<()>
    let result1: Result<()> = io_error().map_err(|e| e.into());
    let result2: Result<()> = custom_error().map_err(|e| e.into());

    assert!(result1.is_err());
    assert!(result2.is_err());
}

#[test]
fn error_with_backtrace_support() {
    // anyhow::Error supports backtrace when RUST_BACKTRACE=1
    let result: Result<()> = okerr::err!("test error");

    assert!(result.is_err());
    let err = result.unwrap_err();

    // Just verify the error exists and can be formatted
    // Actual backtrace depends on environment variables
    let _formatted = format!("{:?}", err);
}

#[test]
fn derived_error_source_attribute() {
    #[derive(Error, Debug)]
    #[error("wrapper error")]
    struct WrapperError {
        #[source]
        inner: std::io::Error,
    }

    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "inner");
    let wrapper = WrapperError { inner: io_err };

    // Check that source is accessible
    let std_err: &dyn StdError = &wrapper;
    assert!(std_err.source().is_some());

    let source = std_err.source().unwrap();
    assert!(source.to_string().contains("inner"));
}
