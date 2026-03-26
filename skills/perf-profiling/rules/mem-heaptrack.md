---
title: Heaptrack Allocation Profiling
impact: MEDIUM
impactDescription: Allocation churn and temporary allocations invisible without heap flamegraphs
tags: heaptrack, heap, allocations, arena, jemalloc, flamegraph
---

## Heaptrack Allocation Profiling

**Impact: MEDIUM (allocation churn and temporary allocations invisible without heap flamegraphs)**

Use heaptrack for detailed allocation profiling including allocation flamegraphs, temporary allocation detection, and allocator comparison. More actionable than simple leak detection for latency optimization.

```bash
# Basic heap profiling
heaptrack ./target/release/myapp
heaptrack_print -f heaptrack.myapp.*.zst

# Allocation flamegraph — shows where allocations originate
heaptrack_print -f heaptrack.myapp.*.zst | flamegraph.pl --title "Allocations" > heap.svg
```

**For Rust projects:** heaptrack intercepts libc malloc/free. Rust's default allocator routes through these, but if using jemalloc or mimalloc via `#[global_allocator]`, you must use the system allocator for heaptrack to intercept:

```rust
// Temporarily switch to system allocator for heaptrack profiling
use std::alloc::System;
#[global_allocator]
static A: System = System;
```

**Key metric — temporary allocations:**

Look at "temporary allocations" as a percentage of total. High percentage = allocation churn (allocate then immediately free). This is the signal to use arena or pool allocation.

Pattern: 5M allocations for 50MB peak memory = severe churn, use arena/pool.

**Compare allocators:**

```bash
# Profile with jemalloc to compare
LD_PRELOAD=/usr/lib/libjemalloc.so heaptrack ./target/release/myapp

# Profile with default allocator
heaptrack ./target/release/myapp

# Compare reports side-by-side
heaptrack_print -f heaptrack.myapp.*.zst > report_jemalloc.txt
heaptrack_print -f heaptrack.myapp.*.zst > report_system.txt
```

Focus on: peak memory, total allocations, temporary allocation percentage, and largest allocation call sites.
