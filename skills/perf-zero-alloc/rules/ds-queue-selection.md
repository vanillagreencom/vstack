---
title: Queue Selection
impact: HIGH
impactDescription: Wrong queue type adds unnecessary synchronization overhead or causes contention
tags: spsc, mpsc, mpmc, queue, ringbuf, rtrb, crossbeam
---

## Queue Selection

**Impact: HIGH (wrong queue type adds unnecessary synchronization overhead or causes contention)**

### SPSC Queues

| Crate | Use Case | Latency | Notes |
|-------|----------|---------|-------|
| `rtrb` | Realtime paths | ~50ns | Wait-free, no_std compatible |
| `ringbuf` | General SPSC | ~100ns | Mature, production-ready |
| `heapless::spsc` | Embedded/no_std | ~80ns | Stack-allocated, fixed size |

**Avoid for SPSC:**
- `crossbeam::ArrayQueue` -- MPMC overhead unnecessary for single producer/consumer
- `std::sync::mpsc` -- not designed for low-latency

### MPSC Queues

| Crate | Use Case | Notes |
|-------|----------|-------|
| `crossbeam::channel` | General MPSC | Bounded/unbounded, good performance |
| `flume` | Drop-in replacement | Slightly faster than crossbeam in some cases |

### MPMC Queues

| Crate | Use Case | Notes |
|-------|----------|-------|
| `crossbeam::ArrayQueue` | Bounded MPMC | Lock-free, good for work stealing |
| `disruptor-rs` | LMAX pattern | Batch processing, multi-consumer |

### Decision Tree

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

### Performance Baselines

| Queue Type | Target ops/ms | Latency |
|------------|---------------|---------|
| SPSC Ring Buffer | 60,000+ | <160ns |
| MPSC Channel | 20,000+ | <500ns |
| Disruptor Pattern | 40M+ ops/sec | <52ns |

If below these baselines, investigate: cache line alignment, false sharing, atomic ordering.
