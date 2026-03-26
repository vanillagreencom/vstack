---
name: perf-cache
description: CPU cache optimization for Rust — data layout, false sharing, prefetching, huge pages, and measurement. Use when writing hot-path code, analyzing cache misses, or optimizing struct layout for L1/L2/LLC efficiency.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# CPU Cache Optimization

Data layout, false sharing prevention, prefetching, memory locking, and cache measurement for Rust hot paths.

## When to Apply

Reference these guidelines when:
- Designing structs that live on hot paths (order books, tick stores, ring buffers)
- Profiling and seeing high L1-dcache miss rates or low IPC
- Writing multi-threaded code with shared atomics
- Allocating large buffers (>2MB) or latency-sensitive memory
- Interpreting `perf stat`, `perf c2c`, or `cachegrind` output

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Data Layout | CRITICAL | `layout-` |
| 2 | False Sharing | CRITICAL | `sharing-` |
| 3 | Prefetching & Pages | HIGH | `mem-` |
| 4 | Measurement | HIGH | `meas-` |

## Quick Reference

### 1. Data Layout (CRITICAL)

- `layout-aos-vs-soa` - SoA when iterating single fields; AoS when accessing multiple fields per element
- `layout-struct-analysis` - `pahole -C MyStruct` to verify field offsets, padding, cache-line fit
- `layout-hot-cold-splitting` - Hot fields in 64-byte aligned struct; cold fields separate

### 2. False Sharing (CRITICAL)

- `sharing-detection` - `perf c2c record/report` for HITM counts on shared cache lines
- `sharing-prevention` - `CachePadded<T>` (128-byte) for cross-thread atomics only

### 3. Prefetching & Pages (HIGH)

- `mem-prefetch-patterns` - Manual `_mm_prefetch` for pointer chasing; hardware handles sequential/stride
- `mem-huge-pages` - `madvise(MADV_HUGEPAGE)` on >2MB allocations for TLB miss reduction
- `mem-mlockall` - Pre-fault + `mlockall` at startup to eliminate hot-path page faults

### 4. Measurement (HIGH)

- `meas-cache-thresholds` - L1 miss >5% investigate, >20% severe; IPC <1.0 = memory-bound; MPKI >10 = memory-bound
- `meas-cachegrind` - `valgrind --tool=cachegrind` for per-line miss counts without root access

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/layout-aos-vs-soa.md
rules/sharing-prevention.md
rules/meas-cache-thresholds.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation
- Correct code example with explanation

## Resources

### Tools

| Tool | Command | Use For |
|------|---------|---------|
| pahole | `pahole -C Struct ./binary` | Struct layout, padding, field offsets |
| perf stat | `perf stat -e L1-dcache-load-misses` | Cache miss rates, IPC, branch misses |
| perf c2c | `perf c2c record && perf c2c report` | False sharing detection (HITM counts) |
| cachegrind | `valgrind --tool=cachegrind` | Per-line cache simulation (no root) |

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| crossbeam | `/crossbeam-rs/crossbeam` | CachePadded, epoch-based GC |
| libc | `/rust-lang/libc` | madvise, mlockall, mmap |

## Full Compiled Document

For the complete guide with all rules expanded: `AGENTS.md`
