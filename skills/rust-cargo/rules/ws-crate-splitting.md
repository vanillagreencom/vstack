---
title: Crate Splitting Strategy
impact: HIGH
impactDescription: serial compilation bottlenecks and slow incremental builds
tags: workspace, crate-splitting, proc-macro, incremental
---

## Crate Splitting Strategy

**Impact: HIGH (serial compilation bottlenecks and slow incremental builds)**

Split workspace crates strategically to maximize build parallelism and minimize incremental rebuild scope. Key splitting triggers: separate proc-macros (they compile serially and block all dependents), break circular dependencies, and isolate frequently-changed code so edits don't invalidate the entire dependency graph. Group by coupling — types and functions that change together stay in the same crate.

**Incorrect (monolith crate with embedded proc-macro):**

```toml
# Cargo.toml — single crate does everything
[package]
name = "myapp"

[dependencies]
syn = "2"       # proc-macro deps compile even for non-macro code
quote = "1"
proc-macro2 = "1"

[lib]
proc-macro = true  # Entire crate is proc-macro — nothing compiles in parallel
```

**Correct (split by compilation characteristics):**

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/myapp",         # Binary — depends on core + macros
    "crates/myapp-core",    # Types, traits — no heavy deps, changes rarely
    "crates/myapp-macros",  # Proc-macro — compiles in parallel with core
    "crates/myapp-engine",  # Business logic — changes frequently, fast incremental
]
resolver = "2"
```
