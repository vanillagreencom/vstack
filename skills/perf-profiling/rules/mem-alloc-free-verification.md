---
title: Allocation-Free Hot Path Verification
impact: MEDIUM
impactDescription: Hidden allocations in hot paths cause latency spikes
tags: allocation, hot-path, mtrace, zero-alloc
---

## Allocation-Free Hot Path Verification

**Impact: MEDIUM (hidden allocations in hot paths cause latency spikes)**

For latency-critical paths that must be allocation-free, verify with runtime allocation tracking. A single unexpected `malloc` in a hot loop can add microseconds of jitter.

```bash
# mtrace - verify zero allocations in hot path
MALLOC_TRACE=/tmp/mtrace.log ./my_app
mtrace ./my_app /tmp/mtrace.log
# Should show: "No memory leaks" and ideally no allocations during hot path

# LD_PRELOAD allocation counter (custom or third-party)
# Interposes malloc/free to count allocations per code path
```

In Rust, use Divan's `AllocProfiler` or a custom global allocator wrapper to assert zero allocations in benchmarked hot paths.
