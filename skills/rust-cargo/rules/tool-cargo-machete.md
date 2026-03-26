---
title: cargo-machete Unused Dependency Detection
impact: HIGH
impactDescription: bloated dependency tree increases compile time and attack surface
tags: machete, udeps, dependencies, cleanup
---

## cargo-machete Unused Dependency Detection

**Impact: HIGH (bloated dependency tree increases compile time and attack surface)**

Detect unused dependencies with `cargo-machete`. Run before releases to trim the dependency tree. Known false positives: proc-macro dependencies (used at compile time only), build-only dependencies (`build-dependencies`), and crates used only via `#[cfg]`-gated code. Combine with `cargo-udeps` (requires nightly) for compile-time verification that catches cases machete's heuristic misses.

**Incorrect (unused dependencies accumulate silently):**

```toml
# Cargo.toml — accumulated deps never cleaned up
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"          # Was used, removed from code, still in Cargo.toml
rand = "0.8"              # Only used in a deleted test
regex = "1"               # Replaced by manual parsing months ago
tokio = { version = "1", features = ["full"] }
```

**Correct (regular auditing with machete):**

```bash
# Quick heuristic scan (works on stable)
cargo install cargo-machete
cargo machete

# Compile-time verification (requires nightly)
cargo install cargo-udeps --locked
cargo +nightly udeps --workspace

# Suppress false positives in Cargo.toml
[package.metadata.cargo-machete]
ignored = ["proc-macro-crate"]  # Used by proc-macro at compile time
```
