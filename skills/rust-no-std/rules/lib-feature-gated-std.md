---
title: Feature-Gated std
impact: HIGH
tags: features, cargo-toml, cfg, std, alloc, portability
---

## Feature-Gated std

**Impact: HIGH (without feature gates, library forces std on all consumers)**

Use Cargo features to let consumers choose their tier. Default to `std` for ergonomics; no_std consumers opt in with `default-features = false`. Structure features hierarchically: `std` implies `alloc`, `alloc` implies core-only.

**Incorrect (no feature gates — forces std on all consumers):**

```toml
# Cargo.toml
[dependencies]
serde = "1"
```

```rust
// lib.rs
use std::io::Write;
use std::collections::HashMap;
```

**Correct (feature-gated Cargo.toml and lib.rs):**

```toml
# Cargo.toml
[features]
default = ["std"]
std = ["alloc", "serde/std"]
alloc = ["serde/alloc"]

[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }
```

```rust
// lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Core API — always available
pub mod parser;

// Alloc API — needs allocator
#[cfg(feature = "alloc")]
pub mod collections;

// Std API — needs OS
#[cfg(feature = "std")]
pub mod io_utils;
```
