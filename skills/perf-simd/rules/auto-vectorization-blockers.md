---
title: Auto-Vectorization Blockers
impact: HIGH
impactDescription: LLVM silently falls back to scalar
tags: auto-vectorization, llvm, blockers, bounds-checks
---

## Auto-Vectorization Blockers

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
