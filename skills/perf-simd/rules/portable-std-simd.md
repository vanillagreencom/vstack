---
title: Portable std::simd API
impact: MEDIUM
impactDescription: misses cross-platform SIMD or uses intrinsics where portable API suffices
tags: portable, std-simd, nightly, cross-platform
---

## Portable std::simd API

**Impact: MEDIUM (misses cross-platform SIMD or uses intrinsics where portable API suffices)**

`std::simd` (nightly, `#![feature(portable_simd)]`): platform-independent SIMD via `Simd<f64, 4>`, with operator overloading (`+`, `*`, etc.). More readable than raw intrinsics. Falls back to scalar on unsupported platforms. Use for new code when nightly is acceptable. Use raw intrinsics when you need specific instructions (shuffles, gathers, FMA) not exposed by portable SIMD.

**Incorrect (raw intrinsics for simple operations that portable SIMD handles):**

```rust
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn add_arrays(a: &[f64; 4], b: &[f64; 4]) -> [f64; 4] {
    let va = _mm256_loadu_pd(a.as_ptr());
    let vb = _mm256_loadu_pd(b.as_ptr());
    let vc = _mm256_add_pd(va, vb);
    let mut out = [0.0f64; 4];
    _mm256_storeu_pd(out.as_mut_ptr(), vc);
    out
}
```

**Correct (portable SIMD — safe, readable, cross-platform):**

```rust
#![feature(portable_simd)]
use std::simd::f64x4;

fn add_arrays(a: &[f64; 4], b: &[f64; 4]) -> [f64; 4] {
    let va = f64x4::from_array(*a);
    let vb = f64x4::from_array(*b);
    let vc = va + vb; // Operator overloading, no unsafe
    vc.to_array()
}
```
