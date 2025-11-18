//! Tests for ensure! macro (re-export from anyhow)

use okerr::{Result, ensure};

#[test]
fn ensure_macro_with_true_condition() {
    fn check_positive(n: i32) -> Result<i32> {
        ensure!(n > 0, "number must be positive");
        Ok(n)
    }

    let result = check_positive(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn ensure_macro_with_false_condition() {
    fn check_positive(n: i32) -> Result<i32> {
        ensure!(n > 0, "number must be positive");
        Ok(n)
    }

    let result = check_positive(-5);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "number must be positive");
}

#[test]
fn ensure_macro_with_formatted_message() {
    fn validate_range(n: i32, min: i32, max: i32) -> Result<i32> {
        ensure!(
            n >= min && n <= max,
            "value {} is out of range [{}, {}]",
            n,
            min,
            max
        );
        Ok(n)
    }

    let result = validate_range(50, 0, 100);
    assert!(result.is_ok());

    let result = validate_range(150, 0, 100);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "value 150 is out of range [0, 100]"
    );
}

#[test]
fn ensure_macro_multiple_conditions() {
    fn validate_string(s: &str) -> Result<String> {
        ensure!(!s.is_empty(), "string cannot be empty");
        ensure!(s.len() <= 10, "string too long: {} chars", s.len());
        Ok(s.to_uppercase())
    }

    let result = validate_string("hello");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO");

    let result = validate_string("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "string cannot be empty");

    let result = validate_string("this is too long");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("string too long"));
}

#[test]
fn ensure_macro_in_loop() {
    fn all_positive(numbers: &[i32]) -> Result<i32> {
        let mut sum = 0;
        for &num in numbers {
            ensure!(num > 0, "found negative or zero number: {}", num);
            sum += num;
        }
        Ok(sum)
    }

    let result = all_positive(&[1, 2, 3, 4, 5]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 15);

    let result = all_positive(&[1, 2, -3, 4]);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("found negative or zero number: -3")
    );
}

#[test]
fn ensure_macro_with_complex_condition() {
    fn validate_user(name: &str, age: i32) -> Result<()> {
        ensure!(
            !name.is_empty() && age >= 18,
            "invalid user: name='{}', age={}",
            name,
            age
        );
        Ok(())
    }

    let result = validate_user("Alice", 25);
    assert!(result.is_ok());

    let result = validate_user("", 25);
    assert!(result.is_err());

    let result = validate_user("Bob", 15);
    assert!(result.is_err());
}

#[test]
fn ensure_macro_early_return() {
    fn process(value: i32) -> Result<i32> {
        ensure!(value != 0, "value cannot be zero");

        // This code should not execute if ensure! fails
        let result = 100 / value;
        Ok(result)
    }

    let result = process(10);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10);

    let result = process(0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "value cannot be zero");
}

#[test]
fn ensure_macro_is_from_anyhow() {
    // Verify that ensure! is the anyhow re-export by checking behavior
    fn test() -> Result<()> {
        ensure!(true, "should not fail");
        ensure!(1 + 1 == 2, "math should work");
        Ok(())
    }

    let result = test();
    assert!(result.is_ok());
}
