---
title: AVX-512 Frequency Throttling
impact: HIGH
impactDescription: entire core slows 10-20% even for non-SIMD code
tags: simd, avx512, throttling, intel, amd, frequency
---

## AVX-512 Frequency Throttling

**Impact: HIGH (entire core slows 10-20% even for non-SIMD code)**

AVX-512 causes frequency throttling on Intel via license levels: L0 (base), L1 (AVX2), L2 (AVX-512 heavy). Using AVX-512 can slow down the ENTIRE core by 10-20% even for non-SIMD code running alongside. On AMD Zen 4+, AVX-512 does NOT cause throttling — safe to use.

Decision: prefer AVX2 on Intel unless sustained batch processing justifies the frequency penalty. Measure total throughput (including surrounding scalar code), not just the SIMD function in isolation.

**Incorrect (using AVX-512 on Intel for short bursts mixed with scalar):**

```rust
#[target_feature(enable = "avx512f")]
unsafe fn quick_check(data: &[f64; 8]) -> bool {
    // Short AVX-512 burst triggers L2 throttle for ~milliseconds
    // Surrounding scalar code pays the frequency penalty
    let v = _mm512_loadu_pd(data.as_ptr());
    let zero = _mm512_setzero_pd();
    let mask = _mm512_cmp_pd_mask::<_CMP_GT_OQ>(v, zero);
    mask == 0xFF
}
```

**Correct (AVX2 for short bursts, AVX-512 only for sustained batch on Intel):**

```rust
#[target_feature(enable = "avx2")]
unsafe fn quick_check(data: &[f64; 8]) -> bool {
    // AVX2: no L2 throttle, negligible frequency impact
    let v1 = _mm256_loadu_pd(data.as_ptr());
    let v2 = _mm256_loadu_pd(data.as_ptr().add(4));
    let zero = _mm256_setzero_pd();
    let cmp1 = _mm256_cmp_pd::<_CMP_GT_OQ>(v1, zero);
    let cmp2 = _mm256_cmp_pd::<_CMP_GT_OQ>(v2, zero);
    let mask1 = _mm256_movemask_pd(cmp1);
    let mask2 = _mm256_movemask_pd(cmp2);
    mask1 == 0xF && mask2 == 0xF
}
```
