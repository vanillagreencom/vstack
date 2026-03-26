---
title: Verify Event Mapping Before Profiling
impact: CRITICAL
impactDescription: Wrong optimization decisions from misinterpreted counters
tags: perf, hardware-counters, verification
---

## Verify Event Mapping Before Profiling

**Impact: CRITICAL (wrong optimization decisions from misinterpreted counters)**

Before drawing conclusions from `perf stat` results, verify what each event actually measures on your specific CPU. Event names like `cache-misses` are aliases whose underlying PMU event varies by architecture.

```bash
# List available events and their descriptions
perf list

# Check what a generic event maps to
perf stat -v -e cache-misses true 2>&1 | grep "config"

# Prefer explicit named events over generic aliases
perf stat -e L1-dcache-load-misses,L1-dcache-loads,dTLB-load-misses \
    ./target/release/my_app
```

When documenting profiling results, always record: CPU model, kernel version, and exact event names used. This ensures results are reproducible and correctly interpreted by others.
