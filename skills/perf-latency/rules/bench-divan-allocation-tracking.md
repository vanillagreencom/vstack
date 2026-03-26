---
title: Use Divan for Allocation Tracking
impact: HIGH
impactDescription: Hidden allocations in hot paths cause latency spikes
tags: divan, allocation, zero-alloc, benchmark
---

## Use Divan for Allocation Tracking

**Impact: HIGH (hidden allocations in hot paths cause latency spikes)**

Divan's `AllocProfiler` tracks heap allocations per benchmark iteration, making it ideal for verifying zero-allocation hot paths. It also supports built-in multi-threaded contention benchmarks.

```rust
use divan::{Bencher, AllocProfiler};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn process_message(bencher: Bencher) {
    bencher
        .with_inputs(|| create_test_message())
        .bench_values(|msg| engine.process(&msg));
}

// Multi-threaded contention benchmark
#[divan::bench(threads = [1, 2, 4, 8])]
fn concurrent_submit(bencher: Bencher) {
    bencher.bench(|| queue.submit(create_item()));
}

fn main() {
    divan::main();
}
```

**When to use Divan vs Criterion:**

| Use Divan | Use Criterion |
|-----------|---------------|
| Zero-allocation verification | Baseline comparisons |
| Thread contention testing | Historical regression detection |
| New projects | Existing Criterion setup |
