---
title: no_std Declaration
impact: CRITICAL
tags: no_std, no_main, crate-root, cfg_attr
---

## no_std Declaration

**Impact: CRITICAL (missing declaration links std, breaking no_std targets)**

Place `#![no_std]` at the crate root to opt out of the standard library. For bare-metal targets, also use `#![no_main]` to disable the default runtime entry point. Use `extern crate alloc;` when alloc features are needed. For dual-mode libraries, use `#![cfg_attr(not(feature = "std"), no_std)]`.

**Incorrect (missing no_std declaration):**

```rust
// lib.rs — no declaration, implicitly links std
use std::vec::Vec;

pub fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
```

**Correct (conditional no_std for dual-mode library):**

```rust
// lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[cfg(feature = "alloc")]
pub fn collect_sums(slices: &[&[i32]]) -> Vec<i32> {
    slices.iter().map(|s| sum(s)).collect()
}
```

For bare-metal binaries:

```rust
#![no_std]
#![no_main]

// No fn main() — entry point provided by runtime crate (e.g., cortex-m-rt)
```
