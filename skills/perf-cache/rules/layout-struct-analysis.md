---
title: Struct Layout Analysis with pahole
impact: CRITICAL
impactDescription: Hidden padding can double struct size and halve cache efficiency
tags: layout, pahole, padding, repr, cache-line
---

## Struct Layout Analysis with pahole

**Impact: CRITICAL (hidden padding can double struct size and halve cache efficiency)**

`pahole -C MyStruct ./target/release/mybin` shows field offsets, sizes, and padding holes in Rust structs. Rust reorders fields by default (unlike C) to minimize padding, but `#[repr(C)]` preserves declaration order. Verify with pahole that hot structs fit in 1-2 cache lines (64-128 bytes). If not, split into hot/cold structs.

**Incorrect (assuming struct fits in one cache line without verifying):**

```rust
#[repr(C)]
struct Tick {
    flags: u8,         // offset 0, size 1
    // 7 bytes padding
    price: f64,        // offset 8, size 8
    side: u8,          // offset 16, size 1
    // 7 bytes padding
    timestamp: u64,    // offset 24, size 8
    exchange: [u8; 32], // offset 32, size 32
}
// Total: 64 bytes but 14 bytes wasted on padding
// Without pahole you wouldn't know
```

**Correct (verify layout, let Rust reorder or manually optimize):**

```bash
# Check actual layout
pahole -C Tick ./target/release/mybin

# Output shows offsets, sizes, padding holes
# Fix: remove repr(C) to let Rust optimize, or reorder fields
```

```rust
// Rust default layout reorders to eliminate padding
struct Tick {
    price: f64,
    timestamp: u64,
    exchange: [u8; 32],
    flags: u8,
    side: u8,
}
// Rust packs this to 50 bytes (no padding waste)
// Verify: pahole -C Tick ./target/release/mybin
```
