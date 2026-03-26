---
title: Dependency Hygiene
impact: MEDIUM
impactDescription: Unnecessary dependencies increase attack surface and build times
tags: supply-chain, dependencies, hygiene, security, maintenance
---

## Dependency Hygiene

**Impact: MEDIUM (unnecessary dependencies increase attack surface and build times)**

Fewer dependencies mean a smaller attack surface, faster builds, and less maintenance burden. Every dependency is code you did not write and must trust.

**Minimal dependency principle:** Every dependency should be justified. If a crate provides a single utility function you could write in 20 lines, do not depend on it.

**Tools for dependency hygiene:**

| Tool | Purpose | Command |
|---|---|---|
| `cargo-machete` | Find unused dependencies | `cargo machete` |
| `cargo tree -d` | Find duplicate dependency versions | `cargo tree -d` |
| `cargo fetch --locked` | Verify `Cargo.lock` matches `Cargo.toml` | `cargo fetch --locked` |
| `cargo deny check` | Unified policy (advisories, licenses, duplicates) | `cargo deny check` |

**CI practices:**
- Pin dependency versions in CI with `--locked` to prevent silent updates
- Run `cargo machete` to detect and remove unused deps
- Run `cargo tree -d` to identify duplicate versions and unify them
- Require justification for new dependencies in PR descriptions

**Incorrect (unnecessary dependency, no lockfile verification):**

```toml
[dependencies]
# left-pad equivalent — one function, pulled in an entire crate
is-even = "1.0"
```

```yaml
# CI does not verify lockfile — deps can silently change
- run: cargo build
```

**Correct (minimal deps, lockfile pinned, unused deps detected):**

```toml
[dependencies]
# Only deps that provide substantial value and are well-maintained
serde = { version = "1", features = ["derive"] }
```

```yaml
# CI verifies lockfile integrity and checks for unused deps
- run: cargo fetch --locked
- run: cargo machete --with-metadata
- run: cargo build --locked
```
