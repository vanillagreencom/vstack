---
title: AMD Generic Event Mapping
impact: CRITICAL
impactDescription: Silently incorrect cache miss data on AMD CPUs
tags: amd, zen, cache, perf, hardware-counters
---

## AMD Generic Event Mapping

**Impact: CRITICAL (silently incorrect cache miss data on AMD CPUs)**

On AMD Zen 4/5, Linux perf's generic hardware events map differently than on Intel. Using generic events produces silently wrong data that leads to incorrect optimization decisions.

**Incorrect (generic events misreport on AMD):**

```bash
# cache-misses maps to L1 INSTRUCTION cache misses on AMD (not LLC!)
# cache-references maps to L1 instruction cache fetches (not LLC accesses!)
perf stat -e cache-misses,cache-references ./target/release/my_app
```

**Correct (use named events or vendor tools):**

```bash
# AMD-safe: named L1 data cache events
perf stat -e L1-dcache-loads,L1-dcache-load-misses ./target/release/my_app

# AMD uProf for L3/LLC analysis
AMDuProfCLI collect --config cache ./target/release/my_app

# AMD uncore events (if exposed by kernel)
perf stat -e amd_l3/event=0x01/ ./target/release/my_app
```

Always verify which CPU vendor you are profiling on before interpreting generic hardware counter results. Intel's generic event mappings generally match expectations (LLC), but AMD's do not.
