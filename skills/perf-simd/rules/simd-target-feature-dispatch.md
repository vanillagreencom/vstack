---
title: Target Feature Runtime Dispatch
impact: HIGH
impactDescription: crashes on hardware missing required SIMD features
tags: simd, target-feature, dispatch, is_x86_feature_detected, multiversion
---

## Target Feature Runtime Dispatch

**Impact: HIGH (crashes on hardware missing required SIMD features)**

Runtime CPU detection + dispatch pattern: detect at startup, branch to optimized path. Functions marked `#[target_feature(enable = "avx2")]` must be `unsafe fn` — the caller must verify the feature exists. The `multiversion` crate automates this. NEVER use `-C target-cpu=native` in distributed binaries — breaks on older hardware.

**Incorrect (compiled with target-cpu=native, crashes on older CPUs):**

```rust
// Cargo.toml or .cargo/config.toml:
// [build]
// rustflags = ["-C", "target-cpu=native"]

fn process(data: &[f64]) -> f64 {
    // Uses AVX2 instructions unconditionally — SIGILL on SSE-only hardware
    data.iter().sum()
}
```

**Correct (runtime detection with safe dispatch):**

```rust
use std::arch::x86_64::*;

pub fn process(data: &[f64]) -> f64 {
    if is_x86_feature_detected!("avx2") {
        // SAFETY: feature detected above
        unsafe { process_avx2(data) }
    } else {
        process_scalar(data)
    }
}

#[target_feature(enable = "avx2")]
unsafe fn process_avx2(data: &[f64]) -> f64 {
    // AVX2-optimized implementation
    // ...
    # 0.0
}

fn process_scalar(data: &[f64]) -> f64 {
    data.iter().sum()
}
```
