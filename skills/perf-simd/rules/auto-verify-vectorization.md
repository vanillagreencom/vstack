---
title: Verify Auto-Vectorization
impact: HIGH
impactDescription: scalar code runs where SIMD should be free
tags: auto-vectorization, llvm, assembly, verification
---

## Verify Auto-Vectorization

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
