---
title: Array-of-Structs vs Struct-of-Arrays
impact: CRITICAL
impactDescription: AoS wastes 7/8 of every cache line when accessing single fields
tags: layout, cache, soa, aos, iteration
---

## Array-of-Structs vs Struct-of-Arrays

**Impact: CRITICAL (AoS wastes 7/8 of every cache line when accessing single fields)**

If you only access 1 field of an 8-field 64-byte struct, AoS wastes 7/8 of every cache line fetched. Convert hot-path data to SoA when iterating over single fields (prices, quantities, timestamps). Keep AoS when accessing multiple fields per element. Measure with `perf stat -e L1-dcache-load-misses`.

**Incorrect (AoS layout — scanning prices loads entire 64-byte structs):**

```rust
struct Order {
    price: f64,       // 8 bytes — the only field we need
    qty: f64,         // 8 bytes — wasted cache space
    timestamp: u64,   // 8 bytes — wasted
    side: u8,         // 1 byte  — wasted
    exchange: [u8; 16], // 16 bytes — wasted
    id: u64,          // 8 bytes — wasted
    flags: u32,       // 4 bytes — wasted
    seq: u64,         // 8 bytes — wasted
}

// Iterating prices pulls in ~64 bytes per order, uses 8
fn best_price(orders: &[Order]) -> f64 {
    orders.iter().map(|o| o.price).fold(f64::MAX, f64::min)
}
```

**Correct (SoA layout — scanning prices touches only price data):**

```rust
struct OrderBook {
    prices: Vec<f64>,
    qtys: Vec<f64>,
    timestamps: Vec<u64>,
    sides: Vec<u8>,
    exchanges: Vec<[u8; 16]>,
    ids: Vec<u64>,
    flags: Vec<u32>,
    seqs: Vec<u64>,
}

// Iterating prices loads only f64 values — ~7x fewer cache misses
fn best_price(book: &OrderBook) -> f64 {
    book.prices.iter().copied().fold(f64::MAX, f64::min)
}
```
