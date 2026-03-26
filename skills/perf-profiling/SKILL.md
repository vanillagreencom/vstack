---
name: perf-profiling
description: Linux performance profiling for low-latency systems. Use for CPU hot path analysis, cache optimization, NUMA locality, latency spikes, jitter diagnosis, or memory leaks.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Linux Profiling for Low-Latency Systems

Profiling patterns for sub-millisecond latency systems on Linux, prioritized by impact.

## When to Apply

Reference these guidelines when:
- Profiling CPU hot paths with flamegraphs or eBPF
- Analyzing cache misses or TLB pressure
- Diagnosing NUMA locality issues or cross-socket latency
- Investigating latency spikes or scheduler jitter
- Tracking memory leaks or verifying allocation-free hot paths
- Interpreting hardware performance counter results

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

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Hardware Event Accuracy | CRITICAL | `hw-` |
| 2 | CPU Profiling | HIGH | `cpu-` |
| 3 | Cache & TLB | HIGH | `cache-` |
| 4 | NUMA Locality | HIGH | `numa-` |
| 5 | System Jitter | HIGH | `jitter-` |
| 6 | Memory Profiling | MEDIUM | `mem-` |

## Quick Reference

### 1. Hardware Event Accuracy (CRITICAL)

- `hw-amd-generic-events` - Generic perf events map incorrectly on AMD; use named events or vendor tools
- `hw-verify-event-mapping` - Verify what each perf event measures on your CPU before interpreting results
- `hw-counter-thresholds` - Concrete thresholds for IPC, cache miss rates, branch misses, and MPKI formula

### 2. CPU Profiling (HIGH)

- `cpu-flamegraph-first` - Start with flamegraphs to identify widest functions before diving into counters
- `cpu-ebpf-production` - Use eBPF profiling for production systems; lower overhead than perf record
- `cpu-offcpu-analysis` - Use off-CPU analysis when latency is high but CPU utilization is low
- `cpu-tma-methodology` - Top-Down Microarchitecture Analysis for systematic bottleneck categorization
- `cpu-differential-flamegraph` - Before/after flamegraph comparison to validate optimizations and catch regressions
- `cpu-amd-uprof` - AMD uProf CLI with IBS and Zen-specific uncore events for Zen 4/5

### 3. Cache & TLB (HIGH)

- `cache-named-events` - Use named cache events over generic aliases for cross-architecture correctness
- `cache-huge-pages` - Use huge pages to reduce TLB pressure; up to 4.5x improvement for random access

### 4. NUMA Locality (HIGH)

- `numa-local-access` - Pin processes to NUMA node; target >95% local memory access ratio

### 5. System Jitter (HIGH)

- `jitter-irq-affinity` - Move IRQs away from latency-critical cores to prevent interrupt jitter
- `jitter-core-isolation` - Verify isolated cores only run application threads
- `jitter-scheduler-latency` - Use perf sched to diagnose high P99.9 with low P50

### 6. Memory Profiling (MEDIUM)

- `mem-heap-profiling` - Choose heaptrack (dev) or Valgrind massif (detailed) based on needs
- `mem-alloc-free-verification` - Verify zero allocations in hot paths with runtime tracking
- `mem-heaptrack` - Heap allocation flamegraphs, temporary allocation detection, and allocator comparison
- `mem-strace-syscall-audit` - Audit hidden syscalls (mmap, futex, clock_gettime) in hot paths

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/hw-amd-generic-events.md
rules/cpu-flamegraph-first.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect/correct examples where applicable
- Practical commands you can run immediately

## Resources

Documentation lookup order: local skill files → ctx7 CLI → web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| tracing | `/websites/rs_tracing` | Structured logging, instrumentation spans |

### Web

| Source | URL | Use For |
|--------|-----|---------|
| Brendan Gregg - perf Examples | `https://www.brendangregg.com/perf.html` | Comprehensive perf reference |
| Brendan Gregg - Flamegraphs | `https://www.brendangregg.com/flamegraphs.html` | Flamegraph methodology |
| Brendan Gregg - Off-CPU Analysis | `https://www.brendangregg.com/offcpuanalysis.html` | Off-CPU profiling |
| BCC Tools | `https://github.com/iovisor/bcc` | eBPF profiling toolkit |
| HRT Blog | `https://blog.hrt.tech/` | TLB optimization, huge pages research |

## Full Compiled Document

For the complete guide with all rules expanded inline: `AGENTS.md`
