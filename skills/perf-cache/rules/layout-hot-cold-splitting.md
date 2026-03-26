---
title: Hot/Cold Struct Splitting
impact: CRITICAL
impactDescription: Cold fields in hot structs waste cache lines on every access
tags: layout, cache-line, splitting, hot-path, alignment
---

## Hot/Cold Struct Splitting

**Impact: CRITICAL (cold fields in hot structs waste cache lines on every access)**

Separate frequently-accessed fields (hot) from rarely-accessed fields (cold) into different structs. Hot struct fits in one cache line (64 bytes). Cold fields accessed via index/pointer. Use `#[repr(align(64))]` on hot struct for cache-line alignment.

**Incorrect (hot and cold fields mixed — every access loads cold data):**

```rust
struct Tick {
    price: f64,              // hot — accessed every tick
    qty: f64,                // hot — accessed every tick
    timestamp: u64,          // hot — accessed every tick
    flags: u32,              // hot — accessed every tick
    exchange: ArrayString<16>, // cold — accessed on display only
    symbol: ArrayString<16>,   // cold — accessed on display only
    seq: u64,                // cold — accessed on audit only
}
// 76+ bytes — spans 2 cache lines, cold fields pollute L1
```

**Correct (hot struct fits one cache line, cold accessed separately):**

```rust
#[repr(align(64))]
struct TickHot {
    price: f64,       // 8 bytes
    qty: f64,         // 8 bytes
    timestamp: u64,   // 8 bytes
    flags: u32,       // 4 bytes
}
// 28 bytes used, padded to 64 — fits exactly one cache line

struct TickCold {
    exchange: ArrayString<16>,
    symbol: ArrayString<16>,
    seq: u64,
}

// Hot path touches only TickHot — one cache line per tick
// Cold path indexes into TickCold when needed
struct TickStore {
    hot: Vec<TickHot>,
    cold: Vec<TickCold>,
}
```
