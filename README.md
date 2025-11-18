# okerr - Ergonomic result / error handling

[![Crates.io](https://img.shields.io/crates/v/okerr.svg)](https://crates.io/crates/okerr)
[![Docs.rs](https://docs.rs/okerr/badge.svg)](https://docs.rs/okerr)
[![License](https://img.shields.io/crates/l/okerr.svg)](https://github.com/nicolab/okerr/blob/main/LICENSE)

`okerr` is a ergonomic result / error handling helpers built on anyhow and thiserror.

Most Rust projects handle errors with `anyhow` or `eyre`, and `thiserror` (for `derive(Error)`). Sometimes we have to juggle between.

`okerr` solves the problem, this crate makes this more convenient by providing consistency and an idiomatic API.

The `okerr` crate is mostly re-exported from `anyhow`, making it 100% compatible in both directions.

Thanks to `okerr::from_boxed_error`, it's easy to capture and convert `eyre` errors without including `eyre` as a dependency!

All of that, about ten lines of code (very lightweight, no overhead, no cost of abstraction). Fully tested of course!

More idiomatic aliases for clearer code:

```rust
anyerr!("Oops!"); // Instead of `anyhow!("Oops!")
err!("Oops!"); // Instead of `Err(anyhow!("Oops!")) or `Err(anyerr!("Oops!"))
fail!("Oops!"); // Instead of `bail!("Oops!")

// ---

use okerr::derive::Error;

#[derive(Error, Debug)]

// Instead of
use thiserror::Error;

#[derive(Error, Debug)]

// ---

// Convert from a boxed error (like eyre::Report)
let error = okerr::from_boxed_error(error.into())

// Create/convert from any std::error::Error
let error = okerr::anyerr!(error)

// Create/convert from a string
let error = okerr::anyerr!("Oops!")

// Wrap a Result into an okerr::Error
// Equivalent to result.map_err(okerr::Error::new)
let result = okerr::wrap_err(result)
```

It's very simple, very lightweight (about ten lines of codes in the `okerr` crate), it provides consistency and a better DX. 100% compatible with `anyhow` and `thiserror`, convert easily error from a boxed error (like eyre::Report and others).

## Related docs

- [Docs.rs: okerr](https://docs.rs/okerr/latest/okerr/)
- [Docs.rs: anyhow](https://docs.rs/anyhow/latest/anyhow/)
- [Docs.rs: thiserror](https://docs.rs/thiserror/latest/thiserror/)

## Examples

### Anyhow like

With `okerr::Result`, `okerr::err!`, `okerr::fail!` and `okerr::anyerr!`:

```rust
use okerr::{Result, err, fail, anyerr};

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        err!("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn maybe_fail(should_fail: bool) -> Result<String> {
  if should_fail {
    fail!("Oops!");
  }

  Ok("No error".to_string())
}

fn main() {
    let result = divide(10, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let result = divide(10, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");

    // fail! does early return
    let result = maybe_fail(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Oops!");

    // No error
    let result = maybe_fail(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "No error");

    // Same as anyhow!(...).
    // Creates an Error directly,
    // from a string or any std::error::Error
    let error = anyerr!("Oops!");
    assert_eq!(error.to_string(), "Oops!");
}
```

### Thiserror like

With `okerr::derive::Error`:

```rust
use okerr::{Result, err, derive::Error};

#[derive(Error, Debug)]
enum MyError {
    #[error("Cannot divide by zero")]
    DivideByZero,
    #[error("Cannot divide by {0}")]
    DivideBy(i32),
}

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        err!(MyError::DivideByZero)
    } else if b < 0 {
        err!(MyError::DivideBy(b))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let result = divide(10, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");

    let result = divide(10, -2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Cannot divide by -2");
}
```

## Tests

`okerr` is fully tested! Run all tests:

```sh
cargo test
```

## Thanks

Thanks to [@dtolnay](https://github.com/dtolnay), the author of [anyhow](https://github.com/dtolnay/anyhow) and [thiserror](https://github.com/dtolnay/thiserror), who provided 2 great ways to handle errors with Rust. 👍

## LICENSE

[MIT](https://github.com/nicolab/okerr/blob/main/LICENSE) (c) 2025, Nicolas Talle.

## Author

- [Nicolas Talle](https://ntalle.com)
- <https://www.linkedin.com/in/ntalle/>

> Buy me a coffee ☕ via [PayPal](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=PGRH4ZXP36GUC)!
