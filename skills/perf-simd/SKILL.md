---
name: perf-simd
description: SIMD intrinsics, auto-vectorization verification, and runtime dispatch for Rust hot paths. Use when writing, reviewing, or optimizing vectorized code — covers auto-vectorization blockers, manual AVX2/AVX-512 intrinsics, portable SIMD, and CPU feature detection.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Perf SIMD

SIMD intrinsics, auto-vectorization, and runtime dispatch for Rust hot paths, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Writing or reviewing hot-path numeric code (price calculations, batch transforms)
- Verifying LLVM auto-vectorization output
- Using `core::arch::x86_64` intrinsics or `std::simd`
- Adding runtime CPU feature detection and dispatch
- Choosing between AVX2 and AVX-512 on Intel vs AMD

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Auto-Vectorization | HIGH | `auto-` |
| 2 | Manual SIMD | HIGH | `simd-` |
| 3 | Portable SIMD | MEDIUM | `portable-` |

## Quick Reference

### 1. Auto-Vectorization (HIGH)

- `auto-verify-vectorization` - Check LLVM vectorization with pass-remarks; verify v-prefix instructions in assembly
- `auto-vectorization-blockers` - Bounds checks, closures, early returns, enum checks, non-contiguous access, non-inline calls
- `auto-chunks-exact` - Use chunks_exact(4) or array_chunks to prove trip count for LLVM

### 2. Manual SIMD (HIGH)

- `simd-target-feature-dispatch` - Runtime is_x86_feature_detected! + #[target_feature] dispatch pattern
- `simd-intrinsic-patterns` - Key AVX2 f64 patterns: loadu, mul, fmadd, cmp, blendv; alignment rules
- `simd-avx512-throttling` - Intel frequency throttling vs AMD Zen 4+; prefer AVX2 on Intel

### 3. Portable SIMD (MEDIUM)

- `portable-std-simd` - std::simd nightly API with Simd<f64, 4> and operator overloading
- `portable-fast-math` - Drop-in SIMD-accelerated crates: fast-float2, simd-json

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/auto-verify-vectorization.md
rules/simd-target-feature-dispatch.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Rust std | `/websites/doc_rust-lang_stable_std` | Standard library API, core::arch |
| criterion | `/criterion-rs/criterion.rs` | Benchmarking vectorized code |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
