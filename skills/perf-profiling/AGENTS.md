# Linux Profiling for Low-Latency Systems

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when profiling,
> diagnosing, or optimizing Linux systems for low latency. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `flamegraph` | Rust flamegraph generation from perf data | `cargo install flamegraph` |
| `samply` | Firefox Profiler UI for Rust/C++ | `cargo install samply` |
| `heaptrack` | Lightweight heap profiler | `sudo apt install heaptrack` / `sudo pacman -S heaptrack` |
| `bcc-tools` | eBPF-based profiling (production-safe) | `sudo apt install bpfcc-tools` / `sudo pacman -S bcc-tools` |
| `valgrind` | Heavyweight memory/heap profiler | `sudo apt install valgrind` / `sudo pacman -S valgrind` |
| `toplev.py` | TMA drill-down (pmu-tools) | `git clone https://github.com/andikleen/pmu-tools` |
| `AMDuProfCLI` | AMD uProf CLI for Zen profiling + IBS | [AMD uProf download](https://developer.amd.com/amd-uprof/) |
| `inferno` | Rust-native flamegraph + diff tooling | `cargo install inferno` |
| `strace` | Syscall tracing and audit | `sudo apt install strace` / `sudo pacman -S strace` |

## Abstract

Profiling patterns for sub-millisecond latency systems on Linux, covering CPU hot path analysis, cache and TLB optimization, NUMA locality, system jitter diagnosis, and memory profiling. Rules are prioritized from critical (hardware counter accuracy) through high-impact (profiling methodology) to incremental (memory verification). Each rule includes practical commands and, where applicable, incorrect vs. correct examples.

## Platform Notes

- **Linux only**: All commands assume Linux with perf_events support
- **Root required**: eBPF tools, some perf events, IRQ affinity changes
- **Kernel version**: eBPF profiling requires kernel 4.6+ with BPF stack trace support
- **Hardware counters**: Some perf events require specific CPU support

## Diagnostic Decision Tree

| Symptom | Likely Cause | Profile With | Solution |
|---------|--------------|--------------|----------|
| High P99.9, low P50 | System jitter | `perf sched latency` | Isolate cores, RT scheduling |
| Consistent high latency | Hot path inefficiency | flamegraph | Optimize wide functions |
| Latency under load | Cache thrashing | `perf stat` named events | Improve data locality |
| Cross-chiplet gap | Thread topology | vendor profiler | Pin threads to same chiplet |
| Memory growth | Memory leak | heaptrack | Fix allocations |
| Cross-socket latency | NUMA misconfig | `numastat` | Pin to correct node |
| Random spikes | TLB misses | `perf stat -e dTLB-*` | Enable huge pages |

---

## Table of Contents

1. [Hardware Event Accuracy](#1-hardware-event-accuracy) — **CRITICAL**
   - 1.1 [AMD Generic Event Mapping](#11-amd-generic-event-mapping)
   - 1.2 [Verify Event Mapping Before Profiling](#12-verify-event-mapping-before-profiling)
   - 1.3 [Hardware Counter Thresholds](#13-hardware-counter-thresholds)
2. [CPU Profiling](#2-cpu-profiling) — **HIGH**
   - 2.1 [Flamegraph First](#21-flamegraph-first)
   - 2.2 [eBPF for Production Profiling](#22-ebpf-for-production-profiling)
   - 2.3 [Off-CPU Analysis for Blocking](#23-off-cpu-analysis-for-blocking)
   - 2.4 [Top-Down Microarchitecture Analysis](#24-top-down-microarchitecture-analysis)
   - 2.5 [Differential Flamegraph Comparison](#25-differential-flamegraph-comparison)
   - 2.6 [AMD uProf CLI for Zen Profiling](#26-amd-uprof-cli-for-zen-profiling)
3. [Cache & TLB](#3-cache--tlb) — **HIGH**
   - 3.1 [Use Named Cache Events](#31-use-named-cache-events)
   - 3.2 [Huge Pages for TLB Optimization](#32-huge-pages-for-tlb-optimization)
4. [NUMA Locality](#4-numa-locality) — **HIGH**
   - 4.1 [NUMA Local Memory Access](#41-numa-local-memory-access)
5. [System Jitter](#5-system-jitter) — **HIGH**
   - 5.1 [IRQ Affinity Away from Critical Cores](#51-irq-affinity-away-from-critical-cores)
   - 5.2 [Core Isolation Verification](#52-core-isolation-verification)
   - 5.3 [Scheduler Latency Analysis](#53-scheduler-latency-analysis)
6. [Memory Profiling](#6-memory-profiling) — **MEDIUM**
   - 6.1 [Heap Profiling Strategy](#61-heap-profiling-strategy)
   - 6.2 [Allocation-Free Hot Path Verification](#62-allocation-free-hot-path-verification)
   - 6.3 [Heaptrack Allocation Profiling](#63-heaptrack-allocation-profiling)
   - 6.4 [Strace Syscall Audit for Hot Paths](#64-strace-syscall-audit-for-hot-paths)

---

## 1. Hardware Event Accuracy

**Impact: CRITICAL**

Hardware performance counter mappings vary across CPU vendors. Using generic events on the wrong architecture produces silently incorrect data, leading to wrong optimization decisions.

### 1.1 AMD Generic Event Mapping

**Impact: CRITICAL (silently incorrect cache miss data on AMD CPUs)**

On AMD Zen 4/5, Linux perf's generic hardware events map differently than on Intel. Using generic events produces silently wrong data that leads to incorrect optimization decisions.

- `cache-misses` maps to **L1 instruction cache misses** on AMD (NOT LLC misses!)
- `cache-references` maps to **L1 instruction cache fetches** on AMD (NOT LLC accesses!)

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

### 1.2 Verify Event Mapping Before Profiling

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

### 1.3 Hardware Counter Thresholds

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

---

## 2. CPU Profiling

**Impact: HIGH**

Patterns for identifying CPU hot paths using flamegraphs, eBPF, and off-CPU analysis. Choosing the right profiler and configuration avoids measurement overhead artifacts.

### 2.1 Flamegraph First

**Impact: HIGH (time wasted on wrong optimization targets without visual hot path analysis)**

Start CPU profiling with flamegraphs to identify the widest functions before diving into specific counters. Wide plateaus in flamegraphs are your optimization targets.

```bash
# cargo-flamegraph (easiest for Rust projects)
cargo install flamegraph
cargo flamegraph --bench my_benchmark
cargo flamegraph -- --release

# Manual perf + flamegraph.pl
perf record -F 99 -g -- ./target/release/my_app
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg

# With DWARF call graphs (more accurate for optimized binaries, slower)
perf record -F 99 --call-graph dwarf -- ./target/release/my_app
```

**Reading flamegraphs:**
- Width = time spent (wider = more CPU time)
- Y-axis = stack depth (bottom = entry point)
- Look for wide plateaus = optimization targets
- Narrow deep stacks are usually fine; wide shallow ones are the problem

### 2.2 eBPF for Production Profiling

**Impact: HIGH (perf overhead distorts measurements in production systems)**

Use eBPF-based profiling (bcc tools) for production or long-running profiles. eBPF does in-kernel aggregation, producing significantly lower overhead than `perf record`.

```bash
# bcc's profile tool - in-kernel stack aggregation
sudo profile -F 99 -p $(pidof my_app) 30 > profile.txt

# Direct to flamegraph
sudo profile -F 99 -f -p $(pidof my_app) 30 | flamegraph.pl > ebpf_flame.svg

# System-wide profiling
sudo profile -F 99 -a 30 | flamegraph.pl > system_flame.svg
```

**When to use eBPF over perf:**
- Production systems where overhead matters
- Long-running profiles (minutes to hours)
- When perf's overhead itself affects measurements
- Kernel 4.6+ with BPF stack trace support required

### 2.3 Off-CPU Analysis for Blocking

**Impact: HIGH (latency spikes from blocking invisible to on-CPU profilers)**

When latency is high but CPU utilization is low, the problem is off-CPU: threads blocked on I/O, locks, or scheduling. Standard flamegraphs only show on-CPU time and will miss these entirely.

```bash
# Find where threads are blocked/waiting
sudo offcputime -f -p $(pidof my_app) 30 | flamegraph.pl --color=io > offcpu.svg

# Kernel stack traces for blocking analysis
sudo offcputime -K -p $(pidof my_app) 30 > offcpu_kernel.txt
```

**Use when:** Latency spikes occur but CPU is not saturated (blocking I/O, lock contention, scheduling delays).

### 2.4 Top-Down Microarchitecture Analysis

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

### 2.5 Differential Flamegraph Comparison

**Impact: HIGH (before/after optimization comparison without visual diff misses regressions)**

Use differential flamegraphs to visually compare before/after profiles. Red frames show regressions (more time), blue frames show improvements (less time). Essential for validating that optimizations helped and nothing regressed.

```bash
# Step 1: Capture before profile
perf record -F 99 -g -- ./target/release/my_app_before
perf script > before.perf
stackcollapse-perf.pl before.perf > before.folded

# Step 2: Capture after profile (same workload, same duration, same CPU)
perf record -F 99 -g -- ./target/release/my_app_after
perf script > after.perf
stackcollapse-perf.pl after.perf > after.folded

# Step 3: Generate differential flamegraph
difffolded.pl before.folded after.folded | flamegraph.pl > diff.svg
# Red = regression (function takes more time)
# Blue = improvement (function takes less time)

# Consistent palette for visual comparison across separate flamegraphs
flamegraph.pl --cp before.folded > before.svg
flamegraph.pl --cp after.folded > after.svg
```

**Rust-native pipeline with inferno:**

```bash
# cargo flamegraph with inferno crate (no Perl dependency)
cargo install inferno
cargo install flamegraph

# Generate comparable profiles
cargo flamegraph --bin my_app -o before.svg -- <same_args>
# ... apply changes ...
cargo flamegraph --bin my_app -o after.svg -- <same_args>

# Diff with inferno
inferno-diff-folded before.folded after.folded | inferno-flamegraph > diff.svg
```

**Critical for valid comparisons:** same workload, same duration, same CPU pinning, same system load. Pin to specific cores with `taskset` to eliminate NUMA/scheduling variance.

### 2.6 AMD uProf CLI for Zen Profiling

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

---

## 3. Cache & TLB

**Impact: HIGH**

Cache miss and TLB analysis patterns. Poor data locality is the most common cause of consistent high latency in memory-intensive systems.

### 3.1 Use Named Cache Events

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

### 3.2 Huge Pages for TLB Optimization

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

---

## 4. NUMA Locality

**Impact: HIGH**

NUMA topology discovery and memory pinning. Cross-socket memory access adds 40-100ns per access, catastrophic for latency-sensitive paths.

### 4.1 NUMA Local Memory Access

**Impact: HIGH (cross-socket memory access adds 40-100ns per access)**

On multi-socket or multi-chiplet systems, memory access latency depends on which NUMA node owns the memory. Remote access is 2-5x slower. Target >95% local access ratio.

```bash
# Discover NUMA topology
numactl --hardware
lscpu | grep -i numa
cat /sys/devices/system/node/node*/cpulist

# Monitor local vs remote memory access
numastat -p $(pidof my_app)
# Target: >95% local (numa_hit / (numa_hit + numa_miss))

# Pin process to NUMA node 0
numactl --cpunodebind=0 --membind=0 ./my_app

# Pin to specific CPUs and memory node
numactl --physcpubind=0-3 --membind=0 ./my_app

# Check which NUMA node NIC is on (pin app to same node)
cat /sys/class/net/eth0/device/numa_node
```

Watch NUMA stats over time to detect drift: `watch -n 1 'numastat -p $(pidof my_app)'`

---

## 5. System Jitter

**Impact: HIGH**

Scheduler latency, IRQ affinity, core isolation, and real-time scheduling verification. System-level interference causes tail latency spikes invisible to application-level profiling.

### 5.1 IRQ Affinity Away from Critical Cores

**Impact: HIGH (interrupt handling on latency-critical cores causes unpredictable jitter spikes)**

Hardware interrupts preempt running threads. If IRQs fire on cores running latency-sensitive work, they cause unpredictable jitter. Move all IRQs to non-critical cores.

```bash
# Check IRQ distribution across CPUs
cat /proc/interrupts | head -20

# Check specific IRQ affinity masks
for irq in /proc/irq/*/smp_affinity; do
    echo "$irq: $(cat $irq)"
done

# Move IRQs away from critical cores (e.g., cores 0-3)
# Set affinity to cores 4+ (mask depends on core count)
echo "fff0" | sudo tee /proc/irq/*/smp_affinity
```

Combine with core isolation (`isolcpus` kernel parameter) for maximum effect.

### 5.2 Core Isolation Verification

**Impact: HIGH (unverified isolation allows OS tasks to preempt latency-critical threads)**

After configuring core isolation, verify that isolated cores only run your application threads. OS housekeeping on isolated cores defeats the purpose of isolation.

```bash
# Verify isolated cores (kernel parameter: isolcpus=0-3)
cat /sys/devices/system/cpu/isolated

# Check process CPU affinity
taskset -cp $(pidof my_app)
# Should show pinned to isolated cores

# Verify no other processes on isolated cores
ps -eo pid,comm,psr | awk '$3 ~ /^[0-3]$/'
# Should only show your application processes
```

### 5.3 Scheduler Latency Analysis

**Impact: HIGH (high tail latency with low P50 indicates OS scheduler interference)**

When P99.9 is much higher than P50, the problem is often OS scheduler interference rather than application code. Use `perf sched` to measure scheduling latency directly.

```bash
# Measure scheduler hiccups
perf sched latency -p $(pidof my_app)

# Record scheduling events for detailed analysis
perf sched record -p $(pidof my_app) -- sleep 10
perf sched latency

# Visual CPU usage timeline
perf sched map

# Set real-time scheduling (requires root or CAP_SYS_NICE)
sudo chrt -f 99 ./my_app

# Verify scheduling policy
chrt -p $(pidof my_app)
# Should show: SCHED_FIFO or SCHED_RR with appropriate priority
```

Combine with core isolation and IRQ affinity for lowest achievable jitter.

---

## 6. Memory Profiling

**Impact: MEDIUM**

Heap profiling, leak detection, and allocation-free verification. Unexpected allocations in hot paths cause latency spikes and GC-like pauses.

### 6.1 Heap Profiling Strategy

**Impact: MEDIUM (undetected memory leaks or unexpected allocations degrade latency over time)**

Choose the right heap profiler based on your needs: heaptrack for development (lighter), Valgrind massif for detailed analysis (heavier), AddressSanitizer for leak detection (compile-time).

```bash
# Heaptrack (lighter, preferred for development)
heaptrack ./my_app
heaptrack_print heaptrack.my_app.*.gz
heaptrack_gui heaptrack.my_app.*.gz    # GUI analysis

# Valgrind massif (detailed heap profiling)
valgrind --tool=massif ./my_app
ms_print massif.out.* > heap_report.txt

# Valgrind memcheck (leak detection)
valgrind --leak-check=full --show-leak-kinds=all \
    ./my_app 2>&1 | tee memcheck.log

# AddressSanitizer (Rust, compile-time, faster than Valgrind)
RUSTFLAGS="-Z sanitizer=address" cargo build --release
./target/release/my_app
```

### 6.2 Allocation-Free Hot Path Verification

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

### 6.3 Heaptrack Allocation Profiling

**Impact: MEDIUM (allocation churn and temporary allocations invisible without heap flamegraphs)**

Use heaptrack for detailed allocation profiling including allocation flamegraphs, temporary allocation detection, and allocator comparison. More actionable than simple leak detection for latency optimization.

```bash
# Basic heap profiling
heaptrack ./target/release/myapp
heaptrack_print -f heaptrack.myapp.*.zst

# Allocation flamegraph — shows where allocations originate
heaptrack_print -f heaptrack.myapp.*.zst | flamegraph.pl --title "Allocations" > heap.svg
```

**For Rust projects:** heaptrack intercepts libc malloc/free. Rust's default allocator routes through these, but if using jemalloc or mimalloc via `#[global_allocator]`, you must use the system allocator for heaptrack to intercept:

```rust
// Temporarily switch to system allocator for heaptrack profiling
use std::alloc::System;
#[global_allocator]
static A: System = System;
```

**Key metric — temporary allocations:**

Look at "temporary allocations" as a percentage of total. High percentage = allocation churn (allocate then immediately free). This is the signal to use arena or pool allocation.

Pattern: 5M allocations for 50MB peak memory = severe churn, use arena/pool.

**Compare allocators:**

```bash
# Profile with jemalloc to compare
LD_PRELOAD=/usr/lib/libjemalloc.so heaptrack ./target/release/myapp

# Profile with default allocator
heaptrack ./target/release/myapp

# Compare reports side-by-side
heaptrack_print -f heaptrack.myapp.*.zst > report_jemalloc.txt
heaptrack_print -f heaptrack.myapp.*.zst > report_system.txt
```

Focus on: peak memory, total allocations, temporary allocation percentage, and largest allocation call sites.

### 6.4 Strace Syscall Audit for Hot Paths

**Impact: MEDIUM (hidden syscalls in hot paths cause latency spikes invisible to CPU profilers)**

Use `strace -c` to audit syscalls in hot paths. Hidden syscalls from allocator growth, lock contention, or time queries add unpredictable latency invisible to CPU-level profilers.

```bash
# Syscall summary — shows count, time, and errors per syscall
strace -c ./target/release/my_app

# Memory syscalls only — find allocator growth
strace -e trace=memory -c ./target/release/my_app

# Filter to specific suspicious syscalls
strace -e trace=mmap,munmap,futex,clock_gettime -c ./target/release/my_app

# Attach to running process
strace -c -p $(pidof my_app)
```

**Syscalls that should be zero in hot paths:**

| Syscall | Indicates | Target |
|---------|-----------|--------|
| `mmap`/`munmap` | Allocator growth/shrink | Zero after startup |
| `futex` | Lock contention | Zero in lock-free paths |
| `clock_gettime` | Time queries | Batch or use TSC |
| `brk` | Heap expansion | Zero after warmup |
| `madvise` | THP faults | Zero in steady state |

**Combine with perf stat for full picture:**

```bash
# Syscall audit + hardware counters together
strace -c ./target/release/my_app 2> syscalls.txt &
perf stat -p $! -e cycles,instructions,cache-misses

# Or sequential: first syscalls, then counters
strace -c -o syscall_report.txt ./target/release/my_app
perf stat -e instructions,cycles,L1-dcache-load-misses ./target/release/my_app
```

**Warning:** `strace` adds significant overhead (ptrace-based). Use only for diagnosis, not benchmarking. For production syscall auditing, use eBPF-based `syscount` from bcc-tools.
