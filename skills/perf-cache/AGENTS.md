# CPU Cache Optimization

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when writing,
> reviewing, or optimizing Rust code for cache performance. Humans may
> also find it useful, but guidance here is optimized for automation
> and consistency by AI-assisted workflows.

---

## Abstract

Data layout, false sharing prevention, prefetching, memory locking, and cache measurement for Rust hot paths. Prioritized by impact from critical (data layout, false sharing) to high (prefetching, measurement).

---

## Table of Contents

1. [Data Layout](#1-data-layout) — **CRITICAL**
   - 1.1 [Array-of-Structs vs Struct-of-Arrays](#11-array-of-structs-vs-struct-of-arrays)
   - 1.2 [Struct Layout Analysis with pahole](#12-struct-layout-analysis-with-pahole)
   - 1.3 [Hot/Cold Struct Splitting](#13-hotcold-struct-splitting)
2. [False Sharing](#2-false-sharing) — **CRITICAL**
   - 2.1 [False Sharing Detection](#21-false-sharing-detection)
   - 2.2 [False Sharing Prevention](#22-false-sharing-prevention)
3. [Prefetching & Pages](#3-prefetching--pages) — **HIGH**
   - 3.1 [Hardware Prefetcher Patterns](#31-hardware-prefetcher-patterns)
   - 3.2 [Huge Pages for TLB Miss Reduction](#32-huge-pages-for-tlb-miss-reduction)
   - 3.3 [mlockall for Page Fault Prevention](#33-mlockall-for-page-fault-prevention)
4. [Measurement](#4-measurement) — **HIGH**
   - 4.1 [Cache Performance Thresholds](#41-cache-performance-thresholds)
   - 4.2 [Cachegrind for Cache Simulation](#42-cachegrind-for-cache-simulation)

---

## 1. Data Layout

**Impact: CRITICAL**

Struct layout, field ordering, hot/cold splitting, and AoS vs SoA decisions. Poor layout wastes cache lines on every access and dominates hot-path latency.

### 1.1 Array-of-Structs vs Struct-of-Arrays

**Impact: CRITICAL (AoS wastes 7/8 of every cache line when accessing single fields)**

If you only access 1 field of an 8-field 64-byte struct, AoS wastes 7/8 of every cache line fetched. Convert hot-path data to SoA when iterating over single fields (prices, quantities, timestamps). Keep AoS when accessing multiple fields per element. Measure with `perf stat -e L1-dcache-load-misses`.

Concrete example: order book with price/qty/timestamp/side/exchange/id/flags/seq — if scanning only prices, SoA reduces cache misses by ~7x.

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

### 1.2 Struct Layout Analysis with pahole

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

### 1.3 Hot/Cold Struct Splitting

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

---

## 2. False Sharing

**Impact: CRITICAL**

Detection and prevention of false sharing between threads. A single false-sharing site can degrade multi-threaded throughput by 10-100x.

### 2.1 False Sharing Detection

**Impact: CRITICAL (undetected false sharing degrades multi-threaded throughput by 10-100x)**

Detect false sharing with `perf c2c record -g ./prog && perf c2c report --stdio`. Look for "Shared Data Cache Line Table" entries with high HITM (Hit Modified) count. Also: `perf stat -e mem_load_l3_hit_retired.xsnp_hitm ./prog`. False sharing threshold: any HITM count > 0 on hot-path atomics is worth investigating.

**Detection commands:**

```bash
# Record cache line contention data
perf c2c record -g ./target/release/mybin

# Report shared cache lines with HITM counts
perf c2c report --stdio

# Quick check for cross-snoop hits (Intel)
perf stat -e mem_load_l3_hit_retired.xsnp_hitm ./target/release/mybin
```

**What to look for:**

```text
# perf c2c report output — high HITM = false sharing
=================================================
 Shared Data Cache Line Table
=================================================
  HITM    Rmt    Lcl   Total   Offset   Symbol
  78.5%  45.2%  33.3%   1842     0x40   Counters::a
  21.5%  12.1%   9.4%    504     0x48   Counters::b
# a and b are on the same cache line — false sharing confirmed
```

### 2.2 False Sharing Prevention

**Impact: CRITICAL (unpadded cross-thread atomics on the same cache line cause 10-100x slowdown)**

Use `crossbeam_utils::CachePadded<T>` to wrap cross-thread atomics with padding to 128 bytes (not 64 — Intel adjacent-line prefetch fetches cache line pairs). Manual alternative: `#[repr(align(128))]`. Only pad cross-thread atomics — padding intra-thread data wastes cache.

**Incorrect (two atomics share a cache line — every write invalidates both):**

```rust
use std::sync::atomic::AtomicU64;

struct Counters {
    producer_count: AtomicU64, // offset 0
    consumer_count: AtomicU64, // offset 8 — same cache line!
}
// Producer writes invalidate consumer's cache line and vice versa
// Both threads constantly reload from L3 or worse
```

**Correct (each atomic gets its own cache line pair):**

```rust
use crossbeam_utils::CachePadded;
use std::sync::atomic::AtomicU64;

struct Counters {
    producer_count: CachePadded<AtomicU64>, // 128-byte aligned
    consumer_count: CachePadded<AtomicU64>, // separate cache line pair
}
// Writer threads only invalidate their own cache line
// Readers see no false contention
```

---

## 3. Prefetching & Pages

**Impact: HIGH**

Hardware prefetch patterns, huge pages, and memory locking. Controls TLB miss rate, page fault latency, and prefetcher effectiveness on non-sequential access.

### 3.1 Hardware Prefetcher Patterns

**Impact: HIGH (pointer chasing and random access defeat hardware prefetch entirely)**

Hardware prefetcher detects: sequential access, constant stride up to 2KB (Intel). Cannot detect: pointer chasing, hash table lookups, random access, variable stride. Use manual prefetch for linked structures. Prefetch distance: 2-4 cache lines ahead for sequential, 1 element ahead for linked lists. Measure: if L1 miss rate drops, prefetch is helping; if it doesn't, remove (wasted instruction).

**Incorrect (pointer chasing with no prefetch — every node is a cache miss):**

```rust
fn sum_linked_list(mut node: Option<&Node>) -> u64 {
    let mut total = 0;
    while let Some(n) = node {
        total += n.value;     // cache miss — next pointer unknown to prefetcher
        node = n.next.as_ref();
    }
    total
}
```

**Correct (manual prefetch one node ahead):**

```rust
use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};

fn sum_linked_list(mut node: Option<&Node>) -> u64 {
    let mut total = 0;
    while let Some(n) = node {
        // Prefetch next node while processing current
        if let Some(ref next) = n.next {
            unsafe {
                _mm_prefetch(
                    (next.as_ref() as *const Node).cast::<i8>(),
                    _MM_HINT_T0,
                );
            }
        }
        total += n.value;
        node = n.next.as_ref();
    }
    total
}
```

### 3.2 Huge Pages for TLB Miss Reduction

**Impact: HIGH (TLB misses on large allocations add 10-100ns per access)**

Transparent Huge Pages (THP) use 2MB pages instead of 4KB, reducing TLB misses by up to 512x for large allocations. Check status: `cat /sys/kernel/mm/transparent_hugepage/enabled`. Use `madvise(MADV_HUGEPAGE)` on large allocations (>2MB) for up to 4.5x improvement on random access. For deterministic latency, prefer explicit huge pages over THP (THP can cause compaction stalls).

**Incorrect (large allocation with 4KB pages — TLB thrashing on random access):**

```rust
// 64MB buffer with 4KB pages = 16,384 TLB entries needed
// Most CPUs have ~1,500 dTLB entries — constant TLB misses
let buffer: Vec<u8> = vec![0u8; 64 * 1024 * 1024];
```

**Correct (advise kernel to use huge pages):**

```rust
let buffer: Vec<u8> = vec![0u8; 64 * 1024 * 1024];
unsafe {
    libc::madvise(
        buffer.as_ptr() as *mut libc::c_void,
        buffer.len(),
        libc::MADV_HUGEPAGE,
    );
}
// 64MB / 2MB = 32 TLB entries instead of 16,384

// Verify huge pages are active:
// grep -i huge /proc/$(pidof app)/smaps
```

### 3.3 mlockall for Page Fault Prevention

**Impact: HIGH (first-touch page faults add 1-5us per page on hot paths)**

`libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE)` at startup prevents page faults on hot paths. Every first access to a new page causes a minor fault (~1-5us). Pre-fault all buffers at startup by writing to every page, then mlockall to keep them resident. Verify: `perf stat -e page-faults ./prog` should show 0 faults during steady state. Downside: increases RSS — all mapped memory stays resident.

**Incorrect (lazy page allocation — first-touch faults during trading):**

```rust
fn main() {
    let ring_buffer = vec![0u8; 4 * 1024 * 1024]; // pages not yet mapped
    // ... start trading loop
    // First write to each page: minor fault, 1-5us stall
    ring_buffer[0] = 1;       // FAULT
    ring_buffer[4096] = 1;    // FAULT
    ring_buffer[8192] = 1;    // FAULT — 1,024 faults for 4MB
}
```

**Correct (pre-fault at startup, lock all pages):**

```rust
fn main() {
    // Lock all current and future pages
    unsafe {
        libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
    }

    // Allocate and pre-fault every page
    let mut ring_buffer = vec![0u8; 4 * 1024 * 1024];
    for page in ring_buffer.chunks_mut(4096) {
        page[0] = 0; // touch every page to force allocation
    }

    // Verify: perf stat -e page-faults ./prog
    // Should show 0 faults after startup phase

    // ... start trading loop — no page faults possible
}
```

---

## 4. Measurement

**Impact: HIGH**

Cache performance measurement tools, thresholds, and simulation. Without measurement, cache optimization is guesswork.

### 4.1 Cache Performance Thresholds

**Impact: HIGH (without concrete thresholds, cache metrics are unactionable)**

Concrete thresholds from hardware performance experts for interpreting `perf stat` output:

| Metric | Healthy | Investigate | Severe |
|--------|---------|-------------|--------|
| L1-dcache miss rate | <5% | 5-20% | >20% |
| LLC miss rate | <2% | 2-5% | >5% (memory-bound) |
| IPC (instructions per cycle) | >2.0 | 1.0-2.0 | <1.0 (memory-bound) |
| Branch miss rate | <2% | 2-5% | >5% |
| MPKI (misses per kilo-instructions) | <5 | 5-10 | >10 (memory-bound) |

MPKI formula: `LLC-load-misses / (instructions / 1000)`

**Measurement commands:**

```bash
# Comprehensive cache stats
perf stat -e cycles,instructions,L1-dcache-loads,L1-dcache-load-misses,\
LLC-loads,LLC-load-misses,branch-misses ./target/release/mybin

# Calculate from output:
# L1 miss rate = L1-dcache-load-misses / L1-dcache-loads
# IPC = instructions / cycles
# MPKI = LLC-load-misses / (instructions / 1000)
```

### 4.2 Cachegrind for Cache Simulation

**Impact: HIGH (without per-line miss counts, optimization targets are guesswork)**

`valgrind --tool=cachegrind ./prog` for per-line cache simulation without root/PMU access. `cg_annotate cachegrind.out.*` for source annotation. `cg_diff old.out new.out` for before/after comparison. 10-50x slower than native but gives exact miss counts per source line. Use when: no root access, CI environments, comparing optimization impact.

**Measurement commands:**

```bash
# Run cache simulation
valgrind --tool=cachegrind ./target/release/mybin

# Annotate source with miss counts
cg_annotate cachegrind.out.12345

# Compare before/after optimization
cg_diff cachegrind.out.before cachegrind.out.after | cg_annotate -
```

**Reading output:**

```text
# cg_annotate output — Dr = data reads, D1mr = L1 data read misses
#        Ir       Dr      D1mr      DLmr
# ------------------------------------------
     1,024    2,048       512        64  fn process_orders(book: &OrderBook)
       512    1,024         8         0      book.prices.iter()  // SoA: 8 misses
       512    1,024       504        64      book.orders.iter()  // AoS: 504 misses
# D1mr column identifies exact cache-miss hot spots
```
