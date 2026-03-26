---
title: Feature Flag Discipline
impact: HIGH
impactDescription: broken builds from non-additive features or implicit feature creation
tags: features, feature-flags, additive, safety
---

## Feature Flag Discipline

**Impact: HIGH (broken builds from non-additive features or implicit feature creation)**

Features must be additive — enabling a feature must never remove functionality or change behavior in a way that breaks code compiled without it. Use `dep:optional_dep` syntax (Rust 1.60+) to avoid implicit feature creation from optional dependency names. Default features should cover the common use case. Never feature-gate safety-critical code (safety checks must always be compiled in).

**Incorrect (non-additive feature and implicit feature creation):**

```toml
[features]
default = ["std"]
std = []
no_std = []  # Non-additive: enabling both std and no_std is contradictory

[dependencies]
openssl = { version = "0.10", optional = true }
# Creates implicit feature "openssl" — confusing, leaks dep name as public API
```

```rust
#[cfg(feature = "no_std")]
fn validate_input(data: &[u8]) -> bool {
    // Safety check only compiled in no_std — BUG: std builds skip validation
    data.len() <= MAX_SIZE
}
```

**Correct (additive features with dep: syntax):**

```toml
[features]
default = ["std"]
std = []
# no "no_std" feature — std absence is the no_std path

tls = ["dep:openssl"]  # Explicit dep: syntax, no implicit feature leak

[dependencies]
openssl = { version = "0.10", optional = true }
```

```rust
// Safety validation always compiled — never behind a feature gate
fn validate_input(data: &[u8]) -> bool {
    data.len() <= MAX_SIZE
}

#[cfg(feature = "tls")]
fn connect_tls(addr: &str) -> Result<TlsStream, Error> {
    // Feature-gated functionality that adds capability
    // ...
}
```
