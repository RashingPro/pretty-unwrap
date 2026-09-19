<div align="center">
<h1>pretty_unwrap</h1>

<img alt="Crates.io Version" src="https://img.shields.io/crates/v/pretty_unwrap">
<img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/pretty_unwrap">
<img alt="GitHub License" src="https://img.shields.io/github/license/RashingPro/pretty-unwrap">

Zero-dependency Rust's `unwrap()` and similar methods implementation with usage of `Display` instead of `Debug`.
</div>

## Usage

```rust
use pretty_unwrap::UnwrapPretty;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("something bad just happened: {0}")]
    SomethingBadHappened(u32)
}

fn main() {
    let err: Result<(), MyError> = Err(MyError::SomethingBadHappened(123));

    err.unwrap(); // Panics with derived `Debug` implementation - "SomethingBadHappened(123)".
    err.unwrap_pretty(); // Panics with implemented `Display` - "something bad just happened: 123"
}
```
