---
title: Hardware Counter Thresholds
impact: CRITICAL
impactDescription: Interpreting counter values without concrete thresholds leads to wrong conclusions
tags: ipc, cache, branch, mpki, thresholds, perf-stat
---

## Hardware Counter Thresholds

**Impact: CRITICAL (interpreting counter values without concrete thresholds leads to wrong conclusions)**

Use concrete thresholds to interpret hardware performance counters. Raw numbers are meaningless without reference points.

**IPC (Instructions Per Cycle):**
- >3.0 — Excellent, pipeline well-utilized
- >2.0 — Healthy, typical for well-optimized code
- 1.0-2.0 — Moderate, room for improvement
- <1.0 — Memory-bound or stall-heavy, investigate immediately

**Cache miss rates:**
- L1-dcache miss rate >5% — Investigate data layout
- L1-dcache miss rate >20% — Severe, likely random access pattern
- LLC (Last Level Cache) miss rate >5% — Memory-bound workload
- LLC miss rate >20% — Severe, data does not fit in cache

**Branch misprediction:**
- Branch miss rate >5% — Predictor struggling, consider branchless code
- Branch miss rate >10% — Severe, sort data or restructure control flow

```bash
# Full counter collection command
perf stat -e instructions,cycles,\
L1-dcache-loads,L1-dcache-load-misses,\
LLC-loads,LLC-load-misses,\
branch-instructions,branch-misses \
    ./target/release/my_app
```

**MPKI (Misses Per Kilo-Instruction) formula:**

MPKI normalizes miss counts by instruction count, enabling cross-workload comparison.

```
MPKI = LLC-load-misses / (instructions / 1000)
```

- MPKI <1 — Cache-friendly
- MPKI 1-10 — Moderate, profile specific access patterns
- MPKI >10 — Memory-bound, optimize data locality or prefetch

```bash
# Collect values for MPKI calculation
perf stat -e instructions,LLC-load-misses ./target/release/my_app
# Then: MPKI = LLC-load-misses / (instructions / 1000)
```

Always collect IPC first (`perf stat ./prog` shows it by default). IPC <1.0 means the bottleneck is almost certainly memory or stalls, not compute — do not optimize algorithms, optimize data access.
