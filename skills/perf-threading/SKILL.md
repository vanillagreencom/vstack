---
name: perf-threading
description: Topology-aware thread pinning, core isolation, SPSC patterns, and page fault prevention for Rust low-latency systems. Use when designing threading architecture, pinning threads, selecting channels, or eliminating jitter sources.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Perf Threading

Topology-aware thread pinning, core isolation, SPSC patterns, and page fault prevention for Rust low-latency systems.

## When to Apply

Reference these guidelines when:
- Designing thread placement for latency-critical Rust systems
- Pinning threads to cores or choosing SPSC channels
- Configuring kernel core isolation or IRQ affinity
- Eliminating page fault jitter from steady-state operation
- Benchmarking inter-thread communication latency

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | CPU Topology | CRITICAL | `topo-` |
| 2 | Core Isolation | HIGH | `isolate-` |
| 3 | SPSC Patterns | HIGH | `spsc-` |
| 4 | Page Fault Prevention | HIGH | `fault-` |

## Quick Reference

### 1. CPU Topology (CRITICAL)

- `topo-discovery` - Read sysfs for L3 sharing groups (CCD), physical core IDs, HT siblings; pin communicating threads to same CCD
- `topo-thread-pinning` - Pin with `pthread_setaffinity_np` or `core_affinity` crate; verify readback; pin at thread start not after warmup

### 2. Core Isolation (HIGH)

- `isolate-kernel-params` - `isolcpus` + `nohz_full` + `rcu_nocbs`; verify with `/sys/devices/system/cpu/isolated`; `taskset` + `chrt -f 99`
- `isolate-irq-affinity` - Move all IRQs to housekeeping cores; place NIC IRQ on same NUMA node, different core from trading thread

### 3. SPSC Patterns (HIGH)

- `spsc-channel-selection` - `rtrb` (~50ns) for hot path SPSC; `crossbeam` for MPSC/MPMC; power-of-2 capacity for bitmask modulo
- `spsc-latency-measurement` - Custom two-thread harness with HdrHistogram; Criterion can't measure cross-thread latency; rate-limit to realistic msg/s

### 4. Page Fault Prevention (HIGH)

- `fault-mlockall` - `mlockall(MCL_CURRENT | MCL_FUTURE)` at startup; pre-fault buffers; verify zero faults with `perf stat`
- `fault-stack-prefault` - Pre-fault thread stack by touching every page at thread start; or reduce stack size via `Builder::stack_size`

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/topo-discovery.md
rules/spsc-channel-selection.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| libc | `/rust-lang/libc` | pthread affinity, mlockall, cpu_set_t |
| hdrhistogram | `/hdrhistogram/hdrhistogram_py` | Latency percentile recording |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
