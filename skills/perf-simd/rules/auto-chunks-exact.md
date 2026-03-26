---
title: Use chunks_exact for Vector Width
impact: HIGH
impactDescription: unknown trip count blocks vectorization
tags: auto-vectorization, chunks_exact, array_chunks, loop
---

## Use chunks_exact for Vector Width

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
