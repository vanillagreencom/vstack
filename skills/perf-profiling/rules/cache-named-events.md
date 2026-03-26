---
title: Use Named Cache Events
impact: HIGH
impactDescription: Generic cache events silently measure wrong cache level on some architectures
tags: cache, perf, L1, TLB, named-events
---

## Use Named Cache Events

**Impact: HIGH (generic cache events silently measure wrong cache level on some architectures)**

Always use explicitly named cache events (`L1-dcache-load-misses`, `dTLB-load-misses`) rather than generic aliases (`cache-misses`, `cache-references`). Named events have consistent semantics across CPU vendors.

```bash
# L1 data cache analysis
perf stat -e L1-dcache-loads,L1-dcache-load-misses \
    ./target/release/my_app

# TLB miss analysis (critical for huge pages verification)
perf stat -e dTLB-load-misses,dTLB-store-misses,iTLB-load-misses \
    ./target/release/my_app

# Page walk cycles (TLB miss penalty)
perf stat -e dtlb_load_misses.walk_completed \
    ./target/release/my_app
```
