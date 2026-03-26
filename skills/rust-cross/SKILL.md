---
name: rust-cross
description: Rust cross-compilation for different architectures and OSes — target triples, toolchain setup, cross/zigbuild tools, static binaries, and CI matrix testing. Use when building Rust for non-native targets or setting up multi-platform CI.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust Cross-Compilation

Target configuration, cross-compilation tools, static binary recipes, and multi-platform CI for Rust projects.

## When to Apply

Reference these guidelines when:
- Building Rust for a non-native target architecture or OS
- Setting up `.cargo/config.toml` for cross-compilation
- Choosing between `cross`, `cargo-zigbuild`, or `-Zbuild-std`
- Creating fully static musl binaries or handling OpenSSL in cross builds
- Configuring GitHub Actions CI for multi-target builds
- Using `#[cfg(...)]` for platform-specific code paths

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Target Configuration | HIGH | `target-` |
| 2 | Cross-Compilation Tools | HIGH | `tool-` |
| 3 | Static Binaries | HIGH | `static-` |
| 4 | Testing & CI | MEDIUM | `ci-` |

## Quick Reference

### 1. Target Configuration (HIGH)

- `target-cargo-config` - `.cargo/config.toml` per-target settings, build defaults, environment variables

### 2. Cross-Compilation Tools (HIGH)

- `tool-zigbuild` - `cargo-zigbuild` with Zig as C/C++ cross-compiler and precise glibc targeting
- `tool-build-std` - `-Zbuild-std` for rebuilding std on nightly, custom target specs for bare-metal

### 3. Static Binaries (HIGH)

- `static-openssl-strategies` - Four strategies for OpenSSL in cross builds: rustls, vendored, Dockerfile, zigbuild

### 4. Testing & CI (MEDIUM)

- `ci-qemu-testing` - QEMU runner for cross-compiled tests, ARM64 memory ordering verification
- `ci-conditional-compilation` - `#[cfg(...)]` patterns, `cfg_if` crate, testing all cfg branches

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/target-cargo-config.md
rules/tool-zigbuild.md
rules/static-openssl-strategies.md
rules/ci-qemu-testing.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library API |
| cross | `/cross-rs/cross` | Docker-based cross-compilation |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
