---
title: Huge Pages for TLB Optimization
impact: HIGH
impactDescription: TLB misses add significant latency to memory-intensive workloads
tags: huge-pages, TLB, THP, madvise, memory
---

## Huge Pages for TLB Optimization

**Impact: HIGH (TLB misses add significant latency to memory-intensive workloads)**

TLB misses trigger expensive page walks. Huge pages (2MB/1GB vs 4KB) dramatically reduce TLB pressure. Expected improvement: up to 4.5x speedup for random access patterns (per HRT research).

```bash
# Check system huge page configuration
grep -i huge /proc/meminfo
# Look for: HugePages_Total, HugePages_Free, Hugepagesize

# Check transparent huge pages (THP) status
cat /sys/kernel/mm/transparent_hugepage/enabled
# [always] madvise never  <- "always" or "madvise" is good

# Verify process is using huge pages
grep -i huge /proc/$(pidof my_app)/smaps | head -20
```

In Rust, use `madvise(MADV_HUGEPAGE)` on large allocations when THP is in `madvise` mode. Profile TLB misses before and after to confirm improvement.
