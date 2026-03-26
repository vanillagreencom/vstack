---
title: SIMD Intrinsic Patterns for f64 Hot Paths
impact: HIGH
impactDescription: incorrect intrinsic use causes silent wrong results or segfaults
tags: simd, intrinsics, avx2, f64, fma, alignment
---

## SIMD Intrinsic Patterns for f64 Hot Paths

**Impact: HIGH (incorrect intrinsic use causes silent wrong results or segfaults)**

Key AVX2 patterns for f64 hot paths via `core::arch::x86_64::*`:

| Intrinsic | Operation | Use Case |
|-----------|-----------|----------|
| `_mm256_loadu_pd` | Unaligned load 4 f64 | Reading price arrays |
| `_mm256_mul_pd` | Batch multiply 4 f64 | price * quantity |
| `_mm256_fmadd_pd` | Fused multiply-add: `a*b+c` | price*qty+fee in one instruction |
| `_mm256_cmp_pd` | Batch comparison with epsilon | Tolerance checks |
| `_mm256_blendv_pd` | Conditional select without branch | Branchless min/max |

Alignment: `#[repr(align(32))]` for AVX2, `#[repr(align(64))]` for AVX-512. Use `_mm256_loadu_pd` (unaligned) unless data is guaranteed aligned, in which case `_mm256_load_pd` is marginally faster.

**Incorrect (scalar multiply-add in loop):**

```rust
fn apply_fees(prices: &[f64], quantities: &[f64], fee: f64, out: &mut [f64]) {
    for i in 0..prices.len() {
        out[i] = prices[i] * quantities[i] + fee; // Two instructions: mul + add
    }
}
```

**Correct (FMA intrinsic — one instruction for multiply-add):**

```rust
use std::arch::x86_64::*;

#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn apply_fees_avx2(prices: &[f64], quantities: &[f64], fee: f64, out: &mut [f64]) {
    let fee_vec = _mm256_set1_pd(fee);
    let chunks = prices.len() / 4;
    for i in 0..chunks {
        let p = _mm256_loadu_pd(prices.as_ptr().add(i * 4));
        let q = _mm256_loadu_pd(quantities.as_ptr().add(i * 4));
        let result = _mm256_fmadd_pd(p, q, fee_vec); // p*q+fee in one instruction
        _mm256_storeu_pd(out.as_mut_ptr().add(i * 4), result);
    }
    // Handle remainder with scalar
    for i in (chunks * 4)..prices.len() {
        out[i] = prices[i] * quantities[i] + fee;
    }
}
```
