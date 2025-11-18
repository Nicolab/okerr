//! Tests for the fail! macro (early return with error)

use okerr::{Result, fail};

#[test]
fn fail_macro_early_return() {
    fn test_function(should_fail: bool) -> Result<String> {
        if should_fail {
            fail!("operation failed");
        }
        Ok("success".to_string())
    }

    let result = test_function(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");

    let result = test_function(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "operation failed");
}

#[test]
fn fail_macro_with_format() {
    fn process_value(value: i32) -> Result<i32> {
        if value < 0 {
            fail!("negative value not allowed: {}", value);
        }
        Ok(value * 2)
    }

    let result = process_value(10);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20);

    let result = process_value(-5);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "negative value not allowed: -5"
    );
}

#[test]
fn fail_macro_code_after_is_not_executed() {
    fn test_unreachable() -> Result<i32> {
        fail!("early exit");
        #[allow(unreachable_code)]
        {
            Ok(42) // This should never execute
        }
    }

    let result = test_unreachable();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "early exit");
}

#[test]
fn fail_macro_in_nested_function() {
    fn outer() -> Result<String> {
        inner()?;
        Ok("outer success".to_string())
    }

    fn inner() -> Result<()> {
        fail!("inner failed");
    }

    let result = outer();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "inner failed");
}

#[test]
fn fail_macro_with_multiple_format_args() {
    fn validate(name: &str, age: i32) -> Result<()> {
        if age < 18 {
            fail!("User {} is too young: {}", name, age);
        }
        Ok(())
    }

    let result = validate("Alice", 25);
    assert!(result.is_ok());

    let result = validate("Bob", 15);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "User Bob is too young: 15");
}

#[test]
fn fail_macro_in_loop() {
    fn find_positive(numbers: &[i32]) -> Result<i32> {
        for &num in numbers {
            if num < 0 {
                fail!("found negative number: {}", num);
            }
            if num > 0 {
                return Ok(num);
            }
        }
        fail!("no positive number found");
    }

    let result = find_positive(&[0, 5, 10]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let result = find_positive(&[0, -3, 10]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "found negative number: -3");

    let result = find_positive(&[0, 0, 0]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "no positive number found");
}

#[test]
fn fail_macro_vs_err_macro() {
    // fail! does early return
    fn with_fail() -> Result<i32> {
        fail!("using fail");
        #[allow(unreachable_code)]
        Ok(42)
    }

    // err! returns Err(...) but doesn't early return by itself
    fn with_err() -> Result<i32> {
        return okerr::err!("using err");
    }

    let result1 = with_fail();
    let result2 = with_err();

    assert!(result1.is_err());
    assert!(result2.is_err());
    assert_eq!(result1.unwrap_err().to_string(), "using fail");
    assert_eq!(result2.unwrap_err().to_string(), "using err");
}
