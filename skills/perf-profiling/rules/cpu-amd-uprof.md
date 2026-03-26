---
title: AMD uProf CLI for Zen Profiling
impact: HIGH
impactDescription: Generic perf events miss Zen-specific microarchitecture details and IBS accuracy
tags: amd, zen, uprof, ibs, infinity-fabric, uncore
---

## AMD uProf CLI for Zen Profiling

**Impact: HIGH (generic perf events miss Zen-specific microarchitecture details and IBS accuracy)**

AMD uProf provides Zen 4/5-specific profiling that generic `perf` cannot match. Key advantage: Instruction-Based Sampling (IBS) is more accurate than statistical sampling for Zen architectures.

```bash
# Time-based profiling (general hot path analysis)
AMDuProfCLI collect --config tbp ./target/release/my_app

# Microarchitecture assessment (TMA-equivalent for Zen)
AMDuProfCLI collect --config assess ./target/release/my_app

# Memory access profiling (cache hierarchy + bandwidth)
AMDuProfCLI collect --config memory ./target/release/my_app

# IBS (Instruction-Based Sampling) — more accurate than perf sampling on Zen
# Samples at instruction retirement, not at arbitrary intervals
AMDuProfCLI collect --ibs-op ./target/release/my_app

# Analyze collected data
AMDuProfCLI report -i /tmp/AMDuProf-<session>/ -o report.csv
```

**Zen-specific perf events (when uProf not available):**

```bash
# AMD L3 uncore events — L3 miss sourcing (local CCD vs remote CCD vs DRAM)
perf stat -e amd_l3/event=0x01/ ./target/release/my_app

# AMD Data Fabric events — Infinity Fabric bandwidth monitoring
perf stat -e amd_df/event=0x07e/ ./target/release/my_app

# Combined Zen-aware profiling
perf stat -e cycles,instructions,L1-dcache-load-misses,\
amd_l3/event=0x01/,amd_l3/event=0x06/ ./target/release/my_app
```

**When to use IBS over perf sampling:** IBS samples at instruction retirement (not arbitrary timer intervals), eliminating skid — the gap between where the event occurred and where it was attributed. For Zen architectures, IBS gives significantly more accurate attribution than `perf record -e cycles`.
