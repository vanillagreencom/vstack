---
title: Top-Down Microarchitecture Analysis
impact: HIGH
impactDescription: Random optimization without systematic bottleneck categorization wastes time
tags: tma, topdown, toplev, vtune, uprof, microarchitecture
---

## Top-Down Microarchitecture Analysis

**Impact: HIGH (random optimization without systematic bottleneck categorization wastes time)**

Use Top-Down Microarchitecture Analysis (TMA) as the systematic approach before random optimization. TMA categorizes all CPU pipeline slots into four buckets, revealing where cycles are lost.

**Four TMA categories and targets:**
- **Retiring** (>70% good) — useful work done
- **Bad Speculation** (<5% target) — wasted work from mispredictions
- **Frontend Bound** (<15% target) — instruction fetch/decode stalls
- **Backend Bound** (<30% target) — execution/memory stalls

```bash
# Linux 5.x+ built-in topdown support
perf stat --topdown ./target/release/my_app

# pmu-tools toplev.py for detailed drill-down
# Install: git clone https://github.com/andikleen/pmu-tools
toplev.py -l1 ./target/release/my_app          # Level 1: four categories
toplev.py -l2 ./target/release/my_app          # Level 2: drill down
toplev.py -l3 --no-desc ./target/release/my_app # Level 3: detailed

# Intel VTune microarchitecture exploration
vtune -collect microarchitecture-exploration ./target/release/my_app

# AMD uProf microarchitecture assessment
AMDuProfCLI collect --config assess ./target/release/my_app
```

**Drill-down paths:**
- Backend Bound → Memory Bound → L1/L2/L3/DRAM (cache hierarchy issue)
- Backend Bound → Core Bound → ALU/port contention (compute bottleneck)
- Frontend Bound → Fetch Latency → iTLB/iCache (code layout issue)
- Bad Speculation → Branch Mispredict → specific branch (add likely/unlikely hints)

Always start with TMA level 1 to identify which category dominates, then drill into that category. Do not optimize Frontend if Backend is the bottleneck.
