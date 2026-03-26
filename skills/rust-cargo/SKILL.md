---
name: rust-cargo
description: Cargo workflows, workspace management, feature flags, build tooling, build performance optimization, and release/CI configuration. Use when setting up workspaces, configuring builds, optimizing compile times, or preparing release profiles.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Cargo Workflows & Build Optimization

Workspace management, build tooling, compilation performance, and release/CI configuration for Rust projects, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Setting up or restructuring a Cargo workspace
- Adding, removing, or configuring dependencies and feature flags
- Optimizing build times (linker, caching, codegen backend)
- Configuring CI pipelines for Rust projects
- Preparing release profiles or reducing binary size
- Running cargo-deny, nextest, or machete audits

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Workspace Management | HIGH | `ws-` |
| 2 | Build Tooling | HIGH | `tool-` |
| 3 | Build Performance | HIGH | `perf-` |
| 4 | Release & CI | MEDIUM | `ci-` |

## Quick Reference

### 1. Workspace Management (HIGH)

- `ws-centralized-deps` - Use `[workspace.dependencies]` for version centralization; `resolver = "2"` mandatory
- `ws-crate-splitting` - Split proc-macros, break circular deps, isolate frequently-changed code
- `ws-feature-flags` - Features must be additive; use `dep:` syntax; never feature-gate safety-critical code

### 2. Build Tooling (HIGH)

- `tool-cargo-deny` - License, advisory, ban, and source policy enforcement via deny.toml
- `tool-cargo-nextest` - Parallel test execution with per-test timeouts, retries, and JUnit output
- `tool-cargo-machete` - Detect unused dependencies; combine with cargo-udeps for compile-time verification

### 3. Build Performance (HIGH)

- `perf-cargo-timings` - HTML timeline of crate parallelism and serial bottlenecks
- `perf-fast-linker` - mold > lld > gold > GNU ld; biggest win for large binaries
- `perf-cranelift-dev` - Cranelift backend for 20-40% faster dev builds; never for release
- `perf-incremental-tips` - debug = 1, split-debuginfo, opt-level = 1 for deps, codegen-units = 256
- `perf-sccache` - Shared compilation cache; local or distributed; CI integration

### 4. Release & CI (MEDIUM)

- `ci-profile-config` - Release profile: opt-level, LTO, codegen-units, strip, panic strategy
- `ci-cargo-llvm-lines` - Measure and fix monomorphization bloat; thin wrapper pattern
- `ci-binary-size` - Per-crate size analysis; reduction combo; track size in CI

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/ws-centralized-deps.md
rules/perf-fast-linker.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Cargo Book | `/websites/doc_rust-lang_cargo` | Cargo reference documentation |
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library API |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
