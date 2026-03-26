---
name: rust-ffi
description: Rust Foreign Function Interface patterns for safe C interop. Use when writing FFI bindings, wrapping C libraries, or exposing Rust APIs to C — covers repr(C), string handling, ownership transfer, bindgen/cbindgen, safe wrappers, and build/linking.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Rust FFI

Safe and correct patterns for Rust Foreign Function Interface boundaries, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Writing `extern "C"` functions or calling C libraries from Rust
- Creating `#[repr(C)]` types for cross-language use
- Converting strings, slices, or owned data across FFI boundaries
- Using bindgen or cbindgen to generate bindings
- Wrapping raw C handles in safe Rust abstractions
- Configuring build.rs for native library linking

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | String & Data Handling | CRITICAL | `data-` |
| 2 | Bindgen & Cbindgen | HIGH | `gen-` |
| 3 | Safe Wrappers | HIGH | `wrap-` |

## Quick Reference

### 1. String & Data Handling (CRITICAL)

- `data-string-conversion` - CStr/CString for null-terminated C strings; never use str::as_ptr for FFI
- `data-slice-ffi` - Pass slices as separate pointer + length params with SAFETY comments
- `data-ownership-transfer` - Box::into_raw/from_raw for ownership transfer; never mix allocators

### 2. Bindgen & Cbindgen (HIGH)

- `gen-sys-crate-pattern` - Split into mylib-sys (raw bindings) + mylib (safe wrapper)

### 3. Safe Wrappers (HIGH)

- `wrap-handle-pattern` - Wrap raw C handles in newtype with Drop; verify Send/Sync safety
- `wrap-lifetime-binding` - Use PhantomData to tie wrapper lifetime to parent; prevent use-after-free
- `wrap-callback-safety` - Catch panics at FFI boundary; unwinding through C frames is UB

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/data-string-conversion.md
rules/gen-sys-crate-pattern.md
rules/wrap-handle-pattern.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library FFI types (CStr, CString, NonNull) |
| libc | `/rust-lang/libc` | C type definitions and platform constants |
| bindgen | `/rust-lang/rust-bindgen` | Generating Rust FFI bindings from C headers |
| cbindgen | `/mozilla/cbindgen` | Generating C headers from Rust exports |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
