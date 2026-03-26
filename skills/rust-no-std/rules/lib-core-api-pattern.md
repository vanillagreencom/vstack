---
title: Core API Pattern
impact: HIGH
tags: api-design, zero-allocation, borrowed-data, portability
---

## Core API Pattern

**Impact: HIGH (allocating APIs exclude no_std consumers without alloc)**

Design core APIs with zero allocation. Accept `&[T]` not `Vec<T>`, accept `&str` not `String`, return references or write to caller-provided buffers. Gate convenience methods that allocate behind the `alloc` feature. This maximizes portability.

**Incorrect (API requires allocation):**

```rust
pub fn parse_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // ... parsing logic ...
    tokens
}

pub fn format_message(parts: &[&str]) -> String {
    parts.join(", ")
}
```

**Correct (core API with optional alloc convenience):**

```rust
/// Core API: writes tokens to caller-provided buffer, returns count.
pub fn parse_tokens_into<'a>(
    input: &'a str,
    buf: &mut [Token<'a>],
) -> usize {
    let mut count = 0;
    // ... parsing logic, writing to buf ...
    count
}

/// Core API: writes formatted message to any `core::fmt::Write`.
pub fn format_message(
    parts: &[&str],
    writer: &mut dyn core::fmt::Write,
) -> core::fmt::Result {
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            writer.write_str(", ")?;
        }
        writer.write_str(part)?;
    }
    Ok(())
}

/// Alloc convenience: collects all tokens into a Vec.
#[cfg(feature = "alloc")]
pub fn parse_tokens(input: &str) -> alloc::vec::Vec<Token> {
    let mut tokens = alloc::vec::Vec::new();
    // ... parsing logic ...
    tokens
}
```
