# Zero-Allocation Rust Patterns

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when building,
> maintaining, or optimizing zero-allocation Rust hot paths. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Core Principle

**Never allocate after startup in hot paths.** Pre-allocate all memory during initialization. Every heap allocation in a hot path is a latency spike waiting to happen.

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `heaptrack` | Full-program heap profiling with GUI | System package manager |
| `dhat` | Rust-native allocation attribution | `cargo add --dev dhat` |
| `assert_no_alloc` | CI-gate allocation assertions | `cargo add --dev assert_no_alloc` |

## Abstract

Patterns for eliminating heap allocations in performance-critical Rust hot paths. Covers object pools, bounded collections, arena allocators, preallocated buffers, string interning, static dispatch, queue selection, cache line padding, allocation verification, and common pitfalls. Rules are prioritized from critical (allocation elimination) through high (data structures, verification) to medium (common pitfalls).

---

## Table of Contents

1. [Allocation Elimination](#1-allocation-elimination) -- **CRITICAL**
   - 1.1 [Object Pools](#11-object-pools)
   - 1.2 [Preallocated Buffers](#12-preallocated-buffers)
   - 1.3 [Static Dispatch Over Dynamic](#13-static-dispatch-over-dynamic)
   - 1.4 [String Interning](#14-string-interning)
2. [Data Structures](#2-data-structures) -- **HIGH**
   - 2.1 [Bounded Collections](#21-bounded-collections)
   - 2.2 [Arena Allocators](#22-arena-allocators)
   - 2.3 [Queue Selection](#23-queue-selection)
   - 2.4 [Cache Line Padding](#24-cache-line-padding)
3. [Verification](#3-verification) -- **HIGH**
   - 3.1 [Allocation Assertions](#31-allocation-assertions)
   - 3.2 [Allocation Profiling](#32-allocation-profiling)
4. [Pitfalls](#4-pitfalls) -- **MEDIUM**
   - 4.1 [Hidden format! Allocations](#41-hidden-format-allocations)
   - 4.2 [Iterator Collect Allocations](#42-iterator-collect-allocations)
   - 4.3 [Recursive Box Allocations](#43-recursive-box-allocations)
   - 4.4 [String Operation Allocations](#44-string-operation-allocations)
   - 4.5 [Vec Push Beyond Capacity](#45-vec-push-beyond-capacity)

---

## 1. Allocation Elimination

**Impact: CRITICAL**

Core patterns for eliminating heap allocations in hot paths. Violations cause latency spikes from allocator contention and unpredictable GC-like pauses.

### 1.1 Object Pools

**Impact: CRITICAL (hot-path allocations cause latency spikes from allocator contention)**

Pre-allocate pools of reusable objects at startup. Acquire/release in the hot path without heap allocation.

#### Slab Pool (recommended for lifecycle-managed objects)

`slab` provides pre-allocated arena storage with stable keys:

```rust
use slab::Slab;

pub struct Pool<T: Default> {
    pool: Slab<T>,
    max_capacity: usize,
}

impl<T: Default> Pool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Slab::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    #[inline]
    pub fn create(&mut self) -> Option<usize> {
        if self.pool.len() < self.max_capacity {
            Some(self.pool.insert(T::default()))
        } else {
            None // Pool exhausted
        }
    }

    #[inline]
    pub fn get(&self, key: usize) -> Option<&T> {
        self.pool.get(key)
    }

    #[inline]
    pub fn release(&mut self, key: usize) {
        self.pool.remove(key);
    }
}
```

Slab keys (usize) are opaque handles, not raw pointers.

#### Free-List Pool (simpler, no lifecycle management)

```rust
pub struct ObjectPool<T> {
    objects: Vec<T>,
    free_indices: Vec<usize>,
}

impl<T: Default + Clone> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            objects: (0..capacity).map(|_| T::default()).collect(),
            free_indices: (0..capacity).rev().collect(),
        }
    }

    #[inline]
    pub fn acquire(&mut self) -> Option<(usize, &mut T)> {
        self.free_indices.pop().map(|idx| (idx, &mut self.objects[idx]))
    }

    #[inline]
    pub fn release(&mut self, idx: usize) {
        self.free_indices.push(idx);
    }
}
```

#### Thread-Safe Pool (SPSC ring buffer)

```rust
use ringbuf::{HeapRb, Producer, Consumer};
use std::sync::Arc;

pub struct ThreadSafePool<T> {
    producer: Producer<T, Arc<HeapRb<T>>>,
    consumer: Consumer<T, Arc<HeapRb<T>>>,
}

impl<T: Default> ThreadSafePool<T> {
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::new(capacity);
        let (mut producer, consumer) = rb.split();
        for _ in 0..capacity {
            producer.try_push(T::default()).expect("Pool init failed");
        }
        Self { producer, consumer }
    }

    #[inline]
    pub fn acquire(&mut self) -> Option<T> {
        self.consumer.try_pop()
    }

    #[inline]
    pub fn release(&mut self, obj: T) {
        let _ = self.producer.try_push(obj);
    }
}
```

### 1.2 Preallocated Buffers

**Impact: CRITICAL (Vec growth in hot path causes unpredictable latency from reallocation)**

Allocate buffers with known maximum capacity at startup. Reuse via `.clear()` which retains capacity.

```rust
pub struct DataProcessor {
    parse_buffer: Vec<u8>,
    level_buffer: Vec<PriceLevel>,
    output_buffer: Vec<Update>,
}

impl DataProcessor {
    pub fn new(max_message_size: usize, max_levels: usize) -> Self {
        Self {
            parse_buffer: Vec::with_capacity(max_message_size),
            level_buffer: Vec::with_capacity(max_levels),
            output_buffer: Vec::with_capacity(100),
        }
    }

    pub fn process(&mut self, raw_data: &[u8]) -> &[Update] {
        self.parse_buffer.clear();
        self.level_buffer.clear();
        self.output_buffer.clear();
        // Work with preallocated buffers...
        &self.output_buffer
    }
}
```

`.clear()` keeps capacity; `.truncate(0)` has the same effect. Never rely on `push()` without verifying remaining capacity.

### 1.3 Static Dispatch Over Dynamic

**Impact: CRITICAL (dynamic dispatch prevents inlining and adds vtable lookup overhead in hot paths)**

Use generics for static dispatch in hot paths. Reserve `dyn` and `Box<dyn ...>` for cold paths.

**Incorrect (dynamic dispatch with vtable overhead):**

```rust
pub fn process_feed(handler: &dyn FeedHandler, data: &[u8]) {
    handler.parse(data); // Runtime dispatch via vtable
}

pub fn map_transform(data: &mut [f64], f: Box<dyn Fn(f64) -> f64>) {
    for x in data { *x = f(*x); } // Heap-allocated closure
}
```

**Correct (static dispatch, zero overhead):**

```rust
pub fn process_feed<F: FeedHandler>(handler: &F, data: &[u8]) {
    handler.parse(data); // Compile-time dispatch, inlinable
}

pub fn map_transform<F: Fn(f64) -> f64>(data: &mut [f64], f: F) {
    for x in data { *x = f(*x); } // Stack-allocated closure
}
```

**When dynamic dispatch is acceptable:**
- Cold paths (configuration, setup)
- Plugin systems where extensibility matters
- Trait objects stored long-term (one allocation, many uses)

### 1.4 String Interning

**Impact: CRITICAL (repeated string allocations and comparisons in hot paths waste cycles)**

Intern strings during initialization. After interning, comparisons are integer comparisons and no further allocations occur.

```rust
use string_cache::DefaultAtom as Atom;

pub struct SymbolTable {
    symbols: DashMap<Atom, u32>,
    next_id: AtomicU32,
}

impl SymbolTable {
    pub fn get_or_intern(&self, symbol: &str) -> u32 {
        // Fast path: already interned
        if let Some(id) = self.symbols.get(&Atom::from(symbol)) {
            return *id;
        }
        // Slow path: intern new symbol (happens once per unique symbol)
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.symbols.insert(Atom::from(symbol), id);
        id
    }
}
```

**Benefits:**
- Symbol comparison becomes integer comparison
- No string allocations after initial interning
- Ideal for finite symbol sets (e.g., market tickers)

---

## 2. Data Structures

**Impact: HIGH**

Choosing and configuring bounded collections, queues, and arenas for predictable memory behavior. Wrong choices cause cache misses, false sharing, or silent capacity overflows.

### 2.1 Bounded Collections

**Impact: HIGH (unbounded collections cause unpredictable heap growth and latency spikes)**

Use `ArrayVec` for stack-allocated, fixed-capacity collections when the maximum size is known at compile time.

```rust
use arrayvec::ArrayVec;

pub struct BoundedBuffer<T, const N: usize> {
    items: ArrayVec<T, N>,
}

impl<T, const N: usize> BoundedBuffer<T, N> {
    pub fn new() -> Self {
        Self { items: ArrayVec::new() }
    }

    pub fn push(&mut self, item: T) -> Result<(), CapacityError> {
        self.items.try_push(item).map_err(|_| CapacityError)
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.items.drain(..)
    }
}
```

**When to use ArrayVec:**
- Known maximum size at compile time
- Total size < 10KB (stack limit consideration)
- Frequent creation/destruction where heap allocation is unacceptable

**When to use preallocated Vec instead:**
- Maximum size known only at runtime
- Size may exceed stack limits

### 2.2 Arena Allocators

**Impact: HIGH (per-object heap allocations for temporary data add allocator overhead per operation)**

Use arena allocators for groups of temporary allocations that share a lifetime. Allocate from the arena, then reset in bulk.

```rust
use bumpalo::Bump;

pub struct ScratchArena {
    arena: Bump,
}

impl ScratchArena {
    pub fn new() -> Self {
        Self {
            arena: Bump::with_capacity(64 * 1024), // 64KB arena
        }
    }

    pub fn alloc_slice<T: Default + Copy>(&self, len: usize) -> &mut [T] {
        self.arena.alloc_slice_fill_default(len)
    }

    /// Clear arena for reuse (cheap pointer reset, no per-object drop)
    pub fn reset(&mut self) {
        self.arena.reset();
    }
}
```

**Usage pattern:**
```rust
let mut scratch = ScratchArena::new(); // Startup

// Hot path
let temp = scratch.alloc_slice::<f64>(1000);
// Use temp...
scratch.reset(); // Reuse for next operation
```

Arenas are ideal when many small allocations are created and freed together (e.g., per-tick scratch data).

### 2.3 Queue Selection

**Impact: HIGH (wrong queue type adds unnecessary synchronization overhead or causes contention)**

#### SPSC Queues

| Crate | Use Case | Latency | Notes |
|-------|----------|---------|-------|
| `rtrb` | Realtime paths | ~50ns | Wait-free, no_std compatible |
| `ringbuf` | General SPSC | ~100ns | Mature, production-ready |
| `heapless::spsc` | Embedded/no_std | ~80ns | Stack-allocated, fixed size |

**Avoid for SPSC:**
- `crossbeam::ArrayQueue` -- MPMC overhead unnecessary for single producer/consumer
- `std::sync::mpsc` -- not designed for low-latency

#### MPSC Queues

| Crate | Use Case | Notes |
|-------|----------|-------|
| `crossbeam::channel` | General MPSC | Bounded/unbounded, good performance |
| `flume` | Drop-in replacement | Slightly faster than crossbeam in some cases |

#### MPMC Queues

| Crate | Use Case | Notes |
|-------|----------|-------|
| `crossbeam::ArrayQueue` | Bounded MPMC | Lock-free, good for work stealing |
| `disruptor-rs` | LMAX pattern | Batch processing, multi-consumer |

#### Decision Tree

```
Single producer, single consumer?
+-- YES -> SPSC
|   +-- Need no_std? -> rtrb or heapless::spsc
|   +-- Need max perf? -> rtrb or custom
|   +-- General use -> ringbuf
+-- NO
    +-- Multiple producers, single consumer? -> crossbeam::channel or flume
    +-- Multiple producers, multiple consumers? -> crossbeam::ArrayQueue
```

#### Performance Baselines

| Queue Type | Target ops/ms | Latency |
|------------|---------------|---------|
| SPSC Ring Buffer | 60,000+ | <160ns |
| MPSC Channel | 20,000+ | <500ns |
| Disruptor Pattern | 40M+ ops/sec | <52ns |

**Industry comparisons** (for context):
- boost::lockfree::spsc_queue: 11,345 ops/ms
- folly::ProducerConsumerQueue: 14,614 ops/ms
- moodycamel::ReaderWriterQueue: 21,815 ops/ms

If below these baselines, investigate: cache line alignment, false sharing, atomic ordering.

### 2.4 Cache Line Padding

**Impact: HIGH (false sharing between threads destroys lock-free performance)**

Use 128-byte cache padding for atomics shared across threads. Intel prefetcher pulls 64-byte pairs, so 128 bytes prevents false sharing even with adjacent prefetch.

```rust
use crossbeam::utils::CachePadded;
use std::sync::atomic::AtomicUsize;

pub struct SPSCQueue<T> {
    buffer: Box<[Option<T>]>,
    capacity: usize,
    head: CachePadded<AtomicUsize>,  // Writer only
    tail: CachePadded<AtomicUsize>,  // Reader only
}
```

**Why 128 bytes:** Head and tail are accessed by different threads. Without padding, they share a cache line, causing constant invalidation. Intel's spatial prefetcher fetches pairs of 64-byte lines, so 64-byte padding is insufficient.

#### Power-of-Two Capacity

Always use power-of-two capacity for fast modulo via bitmask:

```rust
impl<T> SPSCQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        // ...
    }

    #[inline]
    fn index(&self, pos: usize) -> usize {
        pos & (self.capacity - 1) // Fast modulo via bitmask
    }
}
```

Size ring buffers to fit within L2 cache (256-512KB typical) for optimal latency. Exceeding L2 causes measurable performance degradation.

---

## 3. Verification

**Impact: HIGH**

Techniques for detecting, asserting, and profiling allocations to enforce zero-alloc invariants. Without verification, hidden allocations go undetected until production.

### 3.1 Allocation Assertions

**Impact: HIGH (without runtime assertions, hidden allocations go undetected until production)**

Use `assert_no_alloc` as a global allocator in test binaries to enforce zero allocations in hot paths. This is the CI gate -- profiling tools like dhat are for local attribution only.

```toml
[dev-dependencies]
assert_no_alloc = "1.1"
```

```rust
use assert_no_alloc::*;

#[test]
fn hot_path_must_not_allocate() {
    let mut processor = DataProcessor::new(4096, 20);
    let data = create_test_data();

    assert_no_alloc(|| {
        processor.process(&data); // Aborts if any allocation occurs
    });
}
```

No feature flags required -- `assert_no_alloc` uses a global allocator override in each test binary.

Structure allocation tests as dedicated test binaries (one per hot path) for clear CI output.

### 3.2 Allocation Profiling

**Impact: HIGH (without profiling, allocation sources remain unknown even after detection)**

Use profiling tools to identify where allocations occur before optimizing.

#### dhat (Rust-native, source-level attribution)

```toml
[dev-dependencies]
dhat = "0.3"

[features]
dhat-heap = []

[profile.test]
opt-level = 1  # Required for accurate dhat results
```

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn profile_allocations() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut processor = DataProcessor::new(4096, 20);

    #[cfg(feature = "dhat-heap")]
    let stats_before = dhat::HeapStats::get();

    processor.process(&create_test_data());

    #[cfg(feature = "dhat-heap")]
    {
        let stats_after = dhat::HeapStats::get();
        assert_eq!(
            stats_after.total_blocks - stats_before.total_blocks,
            0,
            "Hot path must not allocate"
        );
    }
}
// Run with: cargo test --features dhat-heap
```

#### External Tools

```bash
# heaptrack (recommended for full-program profiling)
heaptrack ./target/release/my_binary
heaptrack_gui heaptrack.my_binary.*.gz

# Valgrind massif
valgrind --tool=massif --massif-out-file=massif.out ./target/release/my_binary
ms_print massif.out > heap_report.txt
```

#### Verification Checklist

1. **Profile first**: heaptrack, dhat, or Divan's AllocProfiler
2. **Add allocation tests**: assert zero allocations in hot path
3. **Check hidden allocations**: `format!`, `collect()`, `to_string()`
4. **Verify capacity**: ensure Vecs do not grow in hot path

---

## 4. Pitfalls

**Impact: MEDIUM**

Common Rust idioms that silently allocate. Awareness prevents accidental allocations in code that appears allocation-free.

### 4.1 Hidden format! Allocations

**Impact: MEDIUM (format! silently allocates a String on every call)**

`format!` creates a new `String` each invocation. In hot paths, write to a preallocated buffer instead.

**Incorrect (allocates on every call):**

```rust
let msg = format!("Price: {}", price);
```

**Correct (write to preallocated buffer):**

```rust
use std::fmt::Write;
let mut buf = String::with_capacity(100); // Preallocate once
write!(&mut buf, "Price: {}", price).unwrap();
```

### 4.2 Iterator Collect Allocations

**Impact: MEDIUM (.collect() allocates a new Vec on every call)**

`.collect()` creates a new collection. In hot paths, write filtered results to a preallocated buffer.

**Incorrect (allocates on every call):**

```rust
let evens: Vec<_> = data.iter().filter(|x| *x % 2 == 0).collect();
```

**Correct (write to preallocated buffer):**

```rust
let mut evens = Vec::with_capacity(data.len() / 2);
for x in data.iter().filter(|x| *x % 2 == 0) {
    evens.push(*x);
}
```

Alternatively, reuse the buffer across calls with `.clear()` before refilling.

### 4.3 Recursive Box Allocations

**Impact: MEDIUM (Box in recursive structures allocates on every insert)**

`Box<T>` in recursive data structures (trees, linked lists) allocates on every node creation. Use arena allocators instead.

**Incorrect (heap allocation per node):**

```rust
enum Tree<T> {
    Leaf,
    Node {
        value: T,
        left: Box<Tree<T>>,   // Allocates!
        right: Box<Tree<T>>,  // Allocates!
    }
}
```

**Correct (arena-allocated nodes):**

```rust
struct ArenaTree<'a, T> {
    nodes: &'a Bump,
}
```

Arena allocation amortizes the cost across all nodes and frees them in a single `reset()`.

### 4.4 String Operation Allocations

**Impact: MEDIUM (common string methods silently create new String allocations)**

Methods like `to_uppercase()`, `to_lowercase()`, and `to_string()` create new `String` allocations. Use in-place alternatives when possible.

**Incorrect (creates new String):**

```rust
let upper = s.to_uppercase();
```

**Correct (modify in place):**

```rust
s.make_ascii_uppercase();
```

Note: `make_ascii_uppercase()` only works for ASCII. For Unicode, the allocation from `to_uppercase()` is unavoidable -- keep it off the hot path.

### 4.5 Vec Push Beyond Capacity

**Impact: MEDIUM (Vec::push may silently reallocate when capacity is exhausted)**

`Vec::push()` will reallocate if `len == capacity`. In hot paths, always verify remaining capacity or use bounded alternatives.

**Incorrect (may reallocate):**

```rust
vec.push(item);
```

**Correct (verify capacity first):**

```rust
assert!(vec.len() < vec.capacity());
vec.push(item);
```

For production hot paths, prefer `ArrayVec::try_push()` which returns an error instead of reallocating, or pre-size the Vec and guard against overflow.
