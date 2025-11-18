//! Tests for from_boxed_error function (converting eyre::Report and other boxed errors)

use okerr::{Result, from_boxed_error};

/// Create an eyre::Report error for testing
fn create_eyre_error(msg: String) -> eyre::Report {
    eyre::eyre!(msg)
}

/// Create a nested eyre::Report error
fn create_nested_eyre_error() -> eyre::Report {
    let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    eyre::eyre!(inner).wrap_err("failed to read config")
}

#[test]
fn from_boxed_error_converts_eyre_report() {
    let eyre_err = create_eyre_error("test error".to_string());
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    assert!(okerr_err.to_string().contains("test error"));
}

#[test]
fn from_boxed_error_with_map_err() {
    fn returns_eyre() -> eyre::Result<i32> {
        Err(create_eyre_error("eyre error".to_string()))
    }

    let result: Result<i32> = returns_eyre().map_err(|e| from_boxed_error(e.into()));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("eyre error"));
}

#[test]
fn from_boxed_error_preserves_message() {
    let original_msg = "original error message";
    let eyre_err = create_eyre_error(original_msg.to_string());
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    let error_string = okerr_err.to_string();
    assert!(error_string.contains(original_msg));
}

#[test]
fn from_boxed_error_with_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file.txt not found");
    let boxed: Box<dyn std::error::Error + Send + Sync> = Box::new(io_err);
    let okerr_err = from_boxed_error(boxed);

    assert!(okerr_err.to_string().contains("file.txt not found"));
}

#[test]
fn from_boxed_error_preserves_source() {
    let eyre_err = create_nested_eyre_error();
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    // The error should have a source (the chain of errors)
    // anyhow preserves the error chain from boxed errors
    let error_string = okerr_err.to_string();
    assert!(!error_string.is_empty());
}

#[test]
fn from_boxed_error_chain_traversal() {
    let eyre_err = create_nested_eyre_error();
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    // Count errors in the chain
    let chain_count = okerr_err.chain().count();

    // Should have at least the main error
    assert!(chain_count >= 1);
}

#[test]
fn from_boxed_error_in_result_chain() {
    fn step1() -> eyre::Result<i32> {
        Err(create_eyre_error("step1 failed".to_string()))
    }

    fn step2() -> Result<i32> {
        step1().map_err(|e| from_boxed_error(e.into()))
    }

    fn step3() -> Result<String> {
        let value = step2()?;
        Ok(format!("value: {}", value))
    }

    let result = step3();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("step1 failed"));
}

#[test]
fn from_boxed_error_with_formatted_eyre() {
    let value = 42;
    let eyre_err = eyre::eyre!("invalid value: {}", value);
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    assert!(okerr_err.to_string().contains("invalid value: 42"));
}

#[test]
fn from_boxed_error_multiple_conversions() {
    let errors = vec![
        create_eyre_error("error 1".to_string()),
        create_eyre_error("error 2".to_string()),
        create_eyre_error("error 3".to_string()),
    ];

    let converted: Vec<_> = errors
        .into_iter()
        .map(|e| {
            let boxed: Box<dyn std::error::Error + Send + Sync> = e.into();
            from_boxed_error(boxed)
        })
        .collect();

    assert_eq!(converted.len(), 3);
    assert!(converted[0].to_string().contains("error 1"));
    assert!(converted[1].to_string().contains("error 2"));
    assert!(converted[2].to_string().contains("error 3"));
}

#[test]
fn from_boxed_error_returns_anyhow_error() {
    let eyre_err = create_eyre_error("test".to_string());
    let boxed: Box<dyn std::error::Error + Send + Sync> = eyre_err.into();
    let okerr_err = from_boxed_error(boxed);

    // okerr::Error is anyhow::Error which implements std::error::Error
    // Verify it's the correct type
    let _: okerr::Error = okerr_err;
}
