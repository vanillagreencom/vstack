---
title: Heap Profiling Strategy
impact: MEDIUM
impactDescription: Undetected memory leaks or unexpected allocations degrade latency over time
tags: heap, valgrind, heaptrack, memory, allocations
---

## Heap Profiling Strategy

**Impact: MEDIUM (undetected memory leaks or unexpected allocations degrade latency over time)**

Choose the right heap profiler based on your needs: heaptrack for development (lighter), Valgrind massif for detailed analysis (heavier), AddressSanitizer for leak detection (compile-time).

```bash
# Heaptrack (lighter, preferred for development)
heaptrack ./my_app
heaptrack_print heaptrack.my_app.*.gz
heaptrack_gui heaptrack.my_app.*.gz    # GUI analysis

# Valgrind massif (detailed heap profiling)
valgrind --tool=massif ./my_app
ms_print massif.out.* > heap_report.txt

# Valgrind memcheck (leak detection)
valgrind --leak-check=full --show-leak-kinds=all \
    ./my_app 2>&1 | tee memcheck.log

# AddressSanitizer (Rust, compile-time, faster than Valgrind)
RUSTFLAGS="-Z sanitizer=address" cargo build --release
./target/release/my_app
```
