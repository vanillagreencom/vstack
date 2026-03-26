---
name: perf-latency
description: Microsecond-precision latency measurement. Use when benchmarking hot paths, measuring P99 latency, tracking performance regressions, implementing Criterion or Divan benchmarks, or validating latency budgets.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Latency Measurement

Patterns for accurate latency measurement, percentile tracking, and regression detection in sub-millisecond systems.

## When to Apply

Reference these guidelines when:
- Benchmarking hot paths with Criterion, Divan, or iai-callgrind
- Measuring P50/P99/P99.9 latency for performance budgets
- Implementing runtime latency tracking with HdrHistogram
- Detecting performance regressions in CI
- Profiling CPU hot spots with samply or flamegraphs
- Validating zero-allocation constraints in measured paths

## Nomenclature

- **P50/P95/P99/P99.9** - Percentile latencies (50th, 95th, 99th, 99.9th)
- **Coordinated omission (CO)** - Measurement bias where closed-loop benchmarks hide latency spikes
- **TSC** - Time Stamp Counter; hardware clock on x86 for nanosecond-precision timing
- **HdrHistogram** - High Dynamic Range Histogram; constant-memory percentile tracking with CO correction
- **iai-callgrind** - Deterministic instruction-counting benchmarks via Valgrind (CI-stable)

## Dev Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `samply` | CPU profiler with Firefox Profiler UI | `cargo install samply` |
| `cargo-flamegraph` | Flamegraph generation from perf data | `cargo install flamegraph` |

## Rule Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Measurement Fundamentals | CRITICAL | `mf-` |
| 2 | Benchmarking | HIGH | `bench-` |
| 3 | Runtime Monitoring | HIGH | `mon-` |
| 4 | Profiling | MEDIUM | `prof-` |

## Quick Reference

### 1. Measurement Fundamentals (CRITICAL)

- `mf-percentiles-not-averages` - Report P50/P95/P99/P99.9, never averages alone
- `mf-coordinated-omission` - Use HdrHistogram CO correction for throughput benchmarks
- `mf-warmup-before-measuring` - Exclude cold-cache iterations from measurement data
- `mf-sufficient-samples` - 10,000+ measurements for reliable P99.9 estimates
- `mf-subtract-timer-overhead` - Measure and subtract timer cost from each sample
- `mf-hardware-timestamps` - Use quanta/TSC for nanosecond-precision timing

### 2. Benchmarking (HIGH)

- `bench-criterion-setup` - Configure sample size, measurement time, and noise thresholds
- `bench-divan-allocation-tracking` - Use Divan AllocProfiler to verify zero-alloc hot paths
- `bench-iai-callgrind-ci` - Use instruction counting for deterministic CI regression gates

### 3. Runtime Monitoring (HIGH)

- `mon-hdrhistogram-tracking` - Use HdrHistogram + quanta for continuous latency tracking
- `mon-percentile-validation` - Automate budget checks against percentile targets
- `mon-regression-detection` - Compare results against baselines with threshold alerts

### 4. Profiling (MEDIUM)

- `prof-profile-before-optimizing` - Always profile before optimizing; use flamegraphs

## How to Use

Read individual rule files for detailed explanations and code examples:

```
rules/mf-percentiles-not-averages.md
rules/bench-criterion-setup.md
```

Each rule file contains:
- Brief explanation of why it matters
- Incorrect code example with explanation (where applicable)
- Correct code example with explanation

## Resources

Documentation lookup order: local skill files -> ctx7 CLI -> web fallback.

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| criterion | `/criterion-rs/criterion.rs` | Benchmark setup, configuration, groups |
| tracing | `/websites/rs_tracing` | Structured logging, profiling instrumentation |

### Web

| Source | URL | Use For |
|--------|-----|---------|
| HdrHistogram | `http://hdrhistogram.org/` | Percentile calculation with CO correction |
| Criterion.rs | `https://github.com/bheisler/criterion.rs` | Established Rust benchmarking |
| Divan | `https://github.com/nvzqz/divan` | Modern Rust benchmarking with allocation tracking |
| iai-callgrind | `https://docs.rs/iai-callgrind/0.16.1/iai_callgrind/` | Deterministic CI regression gates |
| Flamegraphs | `https://www.brendangregg.com/flamegraphs.html` | CPU profiling visualization |

## Full Compiled Document

For the complete guide with all rules expanded, plus monitoring patterns, benchmark storage, and profiling methodology: `AGENTS.md`
