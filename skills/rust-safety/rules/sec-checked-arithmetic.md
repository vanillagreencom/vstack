---
title: Checked Arithmetic for Overflow
impact: HIGH
impactDescription: Integer overflow in unsafe contexts causes memory corruption
tags: security, overflow, arithmetic, integer
---

## Checked Arithmetic for Overflow

**Impact: HIGH (integer overflow in unsafe contexts causes memory corruption)**

Use checked, saturating, or wrapping arithmetic where overflow is possible — especially in allocation size calculations, pointer offset computations, and array index derivations. In debug builds Rust panics on overflow, but release builds silently wrap, which can cause undersized allocations and buffer overflows.

**Incorrect (unchecked multiplication for allocation):**

```rust
let size = count * elem_size; // wraps silently in release
let ptr = alloc::alloc(Layout::from_size_align(size, align)?);
```

**Correct (checked arithmetic prevents undersized allocation):**

```rust
let size = count.checked_mul(elem_size).ok_or(AllocError::Overflow)?;
let ptr = alloc::alloc(Layout::from_size_align(size, align)?);
```
