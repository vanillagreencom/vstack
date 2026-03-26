---
title: Centralized Workspace Dependencies
impact: HIGH
impactDescription: version conflicts and duplicate dependency trees
tags: workspace, dependencies, resolver, virtual-manifest
---

## Centralized Workspace Dependencies

**Impact: HIGH (version conflicts and duplicate dependency trees)**

Use `[workspace.dependencies]` in the root `Cargo.toml` to centralize version management. Member crates reference shared dependencies with `dep.workspace = true` instead of specifying versions directly. `resolver = "2"` is mandatory for edition 2021+ (feature unification per-platform, not global). Use the virtual manifest pattern (no `[package]` in root) for multi-crate projects.

**Incorrect (versions scattered across member crates):**

```toml
# crates/core/Cargo.toml
[dependencies]
serde = "1.0.197"
tokio = { version = "1.36", features = ["full"] }

# crates/api/Cargo.toml
[dependencies]
serde = "1.0.193"  # Different version — causes duplicate in tree
tokio = { version = "1.35", features = ["rt"] }
```

**Correct (centralized in workspace root):**

```toml
# Cargo.toml (virtual manifest)
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0.197", features = ["derive"] }
tokio = { version = "1.36", features = ["full"] }

# crates/core/Cargo.toml
[dependencies]
serde.workspace = true
tokio.workspace = true

# crates/api/Cargo.toml
[dependencies]
serde.workspace = true
tokio = { workspace = true, features = ["rt"] }  # Can add features
```
