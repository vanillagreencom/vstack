---
title: Error Handling
impact: HIGH
tags: error, display, fmt-write, std-error, conditional
---

## Error Handling

**Impact: HIGH (unconditional std::error::Error breaks no_std builds)**

`std::error::Error` is not available in core. Use `core::fmt::Display` for error messages and conditionally implement `std::error::Error` when the `std` feature is enabled. Use `core::fmt::Write` for string formatting without alloc.

**Incorrect (unconditionally using std::error::Error):**

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "invalid input"),
            Self::Overflow => write!(f, "numeric overflow"),
        }
    }
}

impl Error for ParseError {}
```

**Correct (conditional std::error::Error):**

```rust
use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "invalid input"),
            Self::Overflow => write!(f, "numeric overflow"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
```

Use `core::fmt::Write` for string formatting without alloc:

```rust
use core::fmt::Write;

struct FixedBuf {
    buf: [u8; 128],
    pos: usize,
}

impl Write for FixedBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len() - self.pos;
        if bytes.len() > remaining {
            return Err(core::fmt::Error);
        }
        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
```
