# Perf SIMD

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when writing,
> reviewing, or optimizing SIMD and vectorized Rust code. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Abstract

SIMD intrinsics, auto-vectorization verification, and runtime dispatch for Rust hot paths, prioritized by impact from high (auto-vectorization, manual SIMD) to medium (portable SIMD).

---

## Table of Contents

1. [Auto-Vectorization](#1-auto-vectorization) — **HIGH**
   - 1.1 [Verify Auto-Vectorization](#11-verify-auto-vectorization)
   - 1.2 [Auto-Vectorization Blockers](#12-auto-vectorization-blockers)
   - 1.3 [Use chunks_exact for Vector Width](#13-use-chunks_exact-for-vector-width)
2. [Manual SIMD](#2-manual-simd) — **HIGH**
   - 2.1 [Target Feature Runtime Dispatch](#21-target-feature-runtime-dispatch)
   - 2.2 [SIMD Intrinsic Patterns for f64 Hot Paths](#22-simd-intrinsic-patterns-for-f64-hot-paths)
   - 2.3 [AVX-512 Frequency Throttling](#23-avx-512-frequency-throttling)
3. [Portable SIMD](#3-portable-simd) — **MEDIUM**
   - 3.1 [Portable std::simd API](#31-portable-stdsimd-api)
   - 3.2 [Drop-in SIMD-Accelerated Crates](#32-drop-in-simd-accelerated-crates)

---

## 1. Auto-Vectorization

**Impact: HIGH**

Verifying and unblocking LLVM auto-vectorization in Rust hot paths. Violations leave performance on the table — scalar code where SIMD should be free.

### 1.1 Verify Auto-Vectorization

**Impact: HIGH (scalar code runs where SIMD should be free)**

Check if LLVM auto-vectorized your loop:

```bash
RUSTFLAGS="-C llvm-args=-pass-remarks=loop-vectorize" cargo build --release 2>&1 | grep -i vectorize
```

In assembly (`cargo asm` or `--emit=asm`): look for `v` prefix instructions (`vaddpd`, `vmulps`, `vfmadd213pd`) vs scalar (`addsd`, `mulsd`). Verify with iai-callgrind instruction count reduction — vectorized code should show ~4x fewer instructions for f64 operations.

**Incorrect (assuming auto-vectorization without checking):**

```rust
// "This loop is simple, LLVM will vectorize it"
fn sum_products(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    // May or may not be vectorized — never verified
}
```

**Correct (verified with pass-remarks and assembly inspection):**

```rust
// Verified: pass-remarks confirms vectorization,
// assembly shows vfmadd231pd instructions
fn sum_products(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i]; // LLVM vectorizes: vfmadd231pd
    }
    sum
}
```

### 1.2 Auto-Vectorization Blockers

**Impact: HIGH (LLVM silently falls back to scalar)**

Common Rust blockers that prevent LLVM auto-vectorization:

| Blocker | Fix |
|---------|-----|
| Bounds checks in loops | `get_unchecked` or `chunks_exact` |
| Iterator chains with closures | LLVM may not inline; use indexed loops |
| Early returns/breaks | Restructure to process full vectors |
| Enum discriminant checks | Match outside the hot loop |
| Non-contiguous memory access | Gather/scatter or restructure data layout |
| Function calls in loop body | Mark `#[inline]` or `#[inline(always)]` |

Rust advantage: `&[T]` / `&mut [T]` have no-alias guarantees (implicit `restrict`), giving Rust better auto-vectorization than C in many cases where the C compiler must assume potential aliasing.

**Incorrect (bounds checks block vectorization):**

```rust
fn scale(data: &mut [f64], factor: f64) {
    for i in 0..data.len() {
        data[i] *= factor; // Bounds check on every iteration
    }
}
```

**Correct (chunks_exact eliminates bounds checks):**

```rust
fn scale(data: &mut [f64], factor: f64) {
    for chunk in data.chunks_exact_mut(4) {
        chunk[0] *= factor;
        chunk[1] *= factor;
        chunk[2] *= factor;
        chunk[3] *= factor;
    }
    for val in data.chunks_exact_mut(4).into_remainder() {
        *val *= factor;
    }
}
```

### 1.3 Use chunks_exact for Vector Width

**Impact: HIGH (unknown trip count blocks vectorization)**

Use `chunks_exact(4)` or `array_chunks::<4>()` (nightly) to help LLVM prove the loop trip count is a multiple of the vector width. Remainder handled separately. This often unblocks auto-vectorization where a plain `for i in 0..len` fails due to unknown trip count.

**Incorrect (LLVM cannot prove trip count is multiple of vector width):**

```rust
fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i]; // May generate scalar + vector + cleanup
    }
    sum
}
```

**Correct (chunks_exact proves 4-element groups to LLVM):**

```rust
fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut sums = [0.0f64; 4];

    let a_chunks = a.chunks_exact(4);
    let b_chunks = b.chunks_exact(4);
    let a_rem = a_chunks.remainder();
    let b_rem = b_chunks.remainder();

    for (ac, bc) in a_chunks.zip(b_chunks) {
        sums[0] += ac[0] * bc[0];
        sums[1] += ac[1] * bc[1];
        sums[2] += ac[2] * bc[2];
        sums[3] += ac[3] * bc[3];
    }

    let mut total: f64 = sums.iter().sum();
    for (x, y) in a_rem.iter().zip(b_rem.iter()) {
        total += x * y;
    }
    total
}
```

---

## 2. Manual SIMD

**Impact: HIGH**

Explicit SIMD intrinsics via core::arch, runtime CPU detection, and dispatch patterns. Violations cause crashes on unsupported hardware, frequency throttling, or incorrect results from misaligned access.

### 2.1 Target Feature Runtime Dispatch

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

### 2.2 SIMD Intrinsic Patterns for f64 Hot Paths

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

### 2.3 AVX-512 Frequency Throttling

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

---

## 3. Portable SIMD

**Impact: MEDIUM**

Platform-independent SIMD via std::simd and drop-in SIMD-accelerated crates. Reduces manual intrinsic complexity when nightly is acceptable or when crate alternatives exist.

### 3.1 Portable std::simd API

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

### 3.2 Drop-in SIMD-Accelerated Crates

**Impact: MEDIUM (reimplementing what battle-tested crates already provide)**

For batch floating-point and data parsing, use crates that leverage SIMD internally without requiring manual intrinsics:

| Crate | Replaces | Speedup |
|-------|----------|---------|
| `fast-float2` | `str::parse::<f64>()` | 2-10x faster decimal string to f64 |
| `simd-json` | `serde_json::from_str` | 2-4x faster JSON parsing |

These are drop-in replacements requiring no unsafe code in your codebase.

**Incorrect (standard library parsing in hot path):**

```rust
fn parse_prices(lines: &[&str]) -> Vec<f64> {
    lines.iter()
        .map(|s| s.parse::<f64>().unwrap()) // ~100ns per parse
        .collect()
}
```

**Correct (fast-float2 for SIMD-accelerated parsing):**

```rust
use fast_float2::parse;

fn parse_prices(lines: &[&str]) -> Vec<f64> {
    lines.iter()
        .map(|s| {
            let (val, _) = parse::<f64, _>(s.as_bytes()).unwrap(); // ~15ns per parse
            val
        })
        .collect()
}
```
