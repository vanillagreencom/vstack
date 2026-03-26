---
title: SPSC Channel Selection
impact: HIGH
impactDescription: Wrong channel type adds 100-200ns per message on hot path
tags: spsc, rtrb, ringbuf, crossbeam, channel, latency
---

## SPSC Channel Selection

**Impact: HIGH (wrong channel type adds 100-200ns per message on hot path)**

Decision table for channel selection:

| Channel | Latency | Pattern | Use Case |
|---------|---------|---------|----------|
| `rtrb` | ~50ns | Wait-free SPSC | Intra-process hot path (trading) |
| `ringbuf` | ~100ns | SPSC, more features | SPSC with need for custom blocking |
| `crossbeam::channel` | ~200ns | MPSC/MPMC | Backpressure, multiple producers |
| `tokio::sync::mpsc` | Higher | Async-aware | Async context only |

For trading hot path: always `rtrb`. For async-to-sync bridge: SPSC ring forward to tokio mpsc. Always use power-of-2 capacity for bitmask modulo (no division).

**Incorrect (using crossbeam for single-producer single-consumer hot path):**

```rust
// crossbeam MPMC channel on SPSC hot path — 4x slower than necessary
let (tx, rx) = crossbeam::channel::bounded(1024);
// Also: 1024 is power-of-2 (good), but crossbeam doesn't optimize for it
```

**Correct (rtrb for SPSC hot path with power-of-2 capacity):**

```rust
// rtrb: wait-free SPSC, cache-padded indices, ~50ns
let (mut producer, mut consumer) = rtrb::RingBuffer::new(1024); // power-of-2

// Producer (pinned to core 0)
producer.push(message).expect("ring full");

// Consumer (pinned to core 1, same CCD as producer)
if let Ok(msg) = consumer.pop() {
    process(msg);
}
```
