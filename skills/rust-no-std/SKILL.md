---
name: rust-no-std
description: "#![no_std] development: core vs alloc vs std, custom allocators, panic handlers, portable library design, embedded testing"
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust no_std Development

Guidelines for `#![no_std]` Rust development covering environment tiers, runtime handlers, portable library design, embedded patterns, and testing strategies.

## When to Apply

- Writing Rust libraries that must work without the standard library
- Developing for embedded targets (ARM Cortex-M, RISC-V, bare-metal)
- Building portable crates that support both std and no_std consumers
- Implementing custom allocators or panic handlers
- Working with `core` and `alloc` crate APIs

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Environment Tiers | CRITICAL | `env-` |
| 2 | Panic & Allocator | CRITICAL | `rt-` |
| 3 | Portable Library Design | HIGH | `lib-` |
| 4 | Embedded Patterns | HIGH | `embed-` |
| 5 | Testing | MEDIUM | `test-` |

## Quick Reference

### 1. Environment Tiers (CRITICAL)

- `env-no-std-declaration` - `#![no_std]` at crate root. `#![cfg_attr(not(feature = "std"), no_std)]` for dual-mode libraries.

### 2. Panic & Allocator (CRITICAL)

- `rt-oom-handler` - Handle allocation failure. Prefer fallible allocation (`try_reserve`, `Box::try_new`) in embedded.

### 3. Portable Library Design (HIGH)

- `lib-feature-gated-std` - `std` feature gates OS-dependent code. `alloc` feature gates heap allocation. Default to std for ergonomics.
- `lib-core-api-pattern` - Accept `&[T]` not `Vec<T>`, `&str` not `String`. Gate convenience methods behind alloc.
- `lib-error-handling` - `core::fmt::Display` for errors. Conditionally implement `std::error::Error`.

### 4. Embedded Patterns (HIGH)

- `embed-memory-layout` - `memory.x` linker script. Release profile: `opt-level = "z"`, `lto = true`, `panic = "abort"`.

### 5. Testing (MEDIUM)

- `test-host-testing` - `#![cfg_attr(not(test), no_std)]` enables std during tests. Mock hardware via traits.

## How to Use

1. **Claude Code / SKILL.md-aware harnesses:** This file is auto-detected. Rules are applied based on context.
2. **Codex, Copilot, Gemini CLI, and other harnesses:** Use `AGENTS.md` which contains all rules expanded inline.
3. **Individual rules:** Browse `rules/` for specific patterns with examples.

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | core, alloc, no_std attribute, lang items |
| embedded-hal | `/rust-embedded/embedded-hal` | Hardware abstraction traits for embedded |
| heapless | `/rust-embedded/heapless` | Fixed-capacity collections without alloc |

## Full Compiled Document

See [AGENTS.md](./AGENTS.md) for the complete compiled document with all rules, patterns, and references expanded inline.
