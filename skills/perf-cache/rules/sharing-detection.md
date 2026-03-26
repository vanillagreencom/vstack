---
title: False Sharing Detection
impact: CRITICAL
impactDescription: Undetected false sharing degrades multi-threaded throughput by 10-100x
tags: false-sharing, perf-c2c, hitm, cache-line, threads
---

## False Sharing Detection

**Impact: CRITICAL (undetected false sharing degrades multi-threaded throughput by 10-100x)**

Detect false sharing with `perf c2c record -g ./prog && perf c2c report --stdio`. Look for "Shared Data Cache Line Table" entries with high HITM (Hit Modified) count. Also: `perf stat -e mem_load_l3_hit_retired.xsnp_hitm ./prog`. False sharing threshold: any HITM count > 0 on hot-path atomics is worth investigating.

**Detection commands:**

```bash
# Record cache line contention data
perf c2c record -g ./target/release/mybin

# Report shared cache lines with HITM counts
perf c2c report --stdio

# Quick check for cross-snoop hits (Intel)
perf stat -e mem_load_l3_hit_retired.xsnp_hitm ./target/release/mybin
```

**What to look for:**

```text
# perf c2c report output — high HITM = false sharing
=================================================
 Shared Data Cache Line Table
=================================================
  HITM    Rmt    Lcl   Total   Offset   Symbol
  78.5%  45.2%  33.3%   1842     0x40   Counters::a
  21.5%  12.1%   9.4%    504     0x48   Counters::b
# a and b are on the same cache line — false sharing confirmed
```
