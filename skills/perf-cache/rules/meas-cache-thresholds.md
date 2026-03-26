---
title: Cache Performance Thresholds
impact: HIGH
impactDescription: Without concrete thresholds, cache metrics are unactionable
tags: measurement, thresholds, perf-stat, ipc, mpki, cache-miss
---

## Cache Performance Thresholds

**Impact: HIGH (without concrete thresholds, cache metrics are unactionable)**

Concrete thresholds from hardware performance experts for interpreting `perf stat` output:

| Metric | Healthy | Investigate | Severe |
|--------|---------|-------------|--------|
| L1-dcache miss rate | <5% | 5-20% | >20% |
| LLC miss rate | <2% | 2-5% | >5% (memory-bound) |
| IPC (instructions per cycle) | >2.0 | 1.0-2.0 | <1.0 (memory-bound) |
| Branch miss rate | <2% | 2-5% | >5% |
| MPKI (misses per kilo-instructions) | <5 | 5-10 | >10 (memory-bound) |

MPKI formula: `LLC-load-misses / (instructions / 1000)`

**Measurement commands:**

```bash
# Comprehensive cache stats
perf stat -e cycles,instructions,L1-dcache-loads,L1-dcache-load-misses,\
LLC-loads,LLC-load-misses,branch-misses ./target/release/mybin

# Calculate from output:
# L1 miss rate = L1-dcache-load-misses / L1-dcache-loads
# IPC = instructions / cycles
# MPKI = LLC-load-misses / (instructions / 1000)
```
