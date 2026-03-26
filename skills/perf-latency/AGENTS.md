# Latency Measurement

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when benchmarking,
> measuring latency, or validating performance budgets. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows.

---

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

## Abstract

Patterns for accurate latency measurement, percentile tracking, and regression detection in sub-millisecond systems. Covers measurement fundamentals (percentiles, coordinated omission, hardware timestamps), benchmarking with Criterion/Divan/iai-callgrind, runtime monitoring with HdrHistogram, and profiling methodology. Each rule includes detailed explanations and, where applicable, incorrect vs. correct code examples.

---

## Table of Contents

1. [Measurement Fundamentals](#1-measurement-fundamentals) -- **CRITICAL**
   - 1.1 [Measure Percentiles, Not Averages](#11-measure-percentiles-not-averages)
   - 1.2 [Account for Coordinated Omission](#12-account-for-coordinated-omission)
   - 1.3 [Warm Up Before Measuring](#13-warm-up-before-measuring)
   - 1.4 [Collect Sufficient Samples](#14-collect-sufficient-samples)
   - 1.5 [Subtract Timer Overhead](#15-subtract-timer-overhead)
   - 1.6 [Use Hardware Timestamps](#16-use-hardware-timestamps)
2. [Benchmarking](#2-benchmarking) -- **HIGH**
   - 2.1 [Configure Criterion for Reliable Results](#21-configure-criterion-for-reliable-results)
   - 2.2 [Use Divan for Allocation Tracking](#22-use-divan-for-allocation-tracking)
   - 2.3 [Use iai-callgrind for Deterministic CI](#23-use-iai-callgrind-for-deterministic-ci)
3. [Runtime Monitoring](#3-runtime-monitoring) -- **HIGH**
   - 3.1 [Use HdrHistogram for Runtime Tracking](#31-use-hdrhistogram-for-runtime-tracking)
   - 3.2 [Validate Latency Against Budgets](#32-validate-latency-against-budgets)
   - 3.3 [Automate Regression Detection](#33-automate-regression-detection)
4. [Profiling](#4-profiling) -- **MEDIUM**
   - 4.1 [Profile Before Optimizing](#41-profile-before-optimizing)

---

## 1. Measurement Fundamentals

**Impact: CRITICAL**

Core principles for accurate latency measurement. Violations produce misleading data that leads to wrong optimization decisions.

### 1.1 Measure Percentiles, Not Averages

**Impact: CRITICAL (averages hide tail latency that causes user-visible stalls)**

Averages mask tail latency spikes. A system averaging 100us can have P99.9 at 50ms. Always report P50, P95, P99, and P99.9. Use HdrHistogram for correct percentile calculation.

**Incorrect (average hides tail latency):**

```rust
let avg = measurements.iter().sum::<u64>() / measurements.len() as u64;
println!("Average latency: {}ns", avg);
```

**Correct (report percentiles):**

```rust
use hdrhistogram::Histogram;

let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
for &m in &measurements {
    histogram.record(m).unwrap();
}
println!("P50:   {}ns", histogram.value_at_percentile(50.0));
println!("P99:   {}ns", histogram.value_at_percentile(99.0));
println!("P99.9: {}ns", histogram.value_at_percentile(99.9));
```

### 1.2 Account for Coordinated Omission

**Impact: CRITICAL (benchmarks hide true latency spikes under load)**

When a benchmark waits for each response before sending the next request, it "coordinates" with system slowdowns and hides true latency spikes. Example: target throughput 10,000 ops/sec (100us interval). One request takes 50ms -- without CO correction this shows 1 outlier at 50ms; with correction it shows 500 missed measurements (50ms / 100us).

**When to apply:**
- Throughput benchmarks (X requests/sec)
- Continuous stream processing

**When NOT to apply:**
- Single request-response microbenchmarks (Criterion and Divan handle this)

**Incorrect (closed-loop benchmark hides stalls):**

```rust
for _ in 0..10_000 {
    let start = clock.raw();
    let _result = process_message(&msg);
    let end = clock.raw();
    histogram.record(clock.delta(start, end)).unwrap();
}
```

**Correct (use HdrHistogram CO correction for throughput tests):**

```rust
for _ in 0..10_000 {
    let start = clock.raw();
    let _result = process_message(&msg);
    let end = clock.raw();
    histogram.record_correct(
        clock.delta(start, end),
        expected_interval_ns  // e.g., 100_000 for 10k ops/sec
    ).unwrap();
}
```

**Reference**: Gil Tene's "How NOT to Measure Latency" talk.

### 1.3 Warm Up Before Measuring

**Impact: CRITICAL (cold cache effects corrupt measurement data)**

The first iterations of any benchmark are always slower due to cold CPU caches, branch predictor training, and memory page faults. Include a warmup phase that is excluded from measurement.

**Incorrect (first measurement includes cold cache penalty):**

```rust
let start = Instant::now();
let result = operation();
let duration = start.elapsed();
```

**Correct (warm up before measuring):**

```rust
// Warm up (excluded from measurement)
for _ in 0..1_000 {
    black_box(operation());
}

// Measure
let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
for _ in 0..10_000 {
    let start = clock.raw();
    black_box(operation());
    let end = clock.raw();
    histogram.record(clock.delta(start, end)).unwrap();
}
```

### 1.4 Collect Sufficient Samples

**Impact: CRITICAL (too few samples make high-percentile estimates statistically meaningless)**

To estimate P99.9 you need at least 10,000 measurements (1 in 1,000 events). For P99, at least 1,000. Fewer samples produce unreliable tail estimates.

**Incorrect (100 samples cannot estimate P99.9):**

```rust
for _ in 0..100 {
    measure();
}
let p999 = histogram.value_at_percentile(99.9); // Statistically meaningless
```

**Correct (10,000+ samples for P99.9):**

```rust
for _ in 0..10_000 {
    measure();
}
let p999 = histogram.value_at_percentile(99.9); // Reliable estimate
```

### 1.5 Subtract Timer Overhead

**Impact: CRITICAL (timer cost inflates all measurements, especially sub-microsecond operations)**

Every timing call has overhead (~20ns for `Instant::now()`, ~0ns for quanta with upkeep thread). For sub-microsecond operations, this overhead is significant. Measure and subtract it.

**Correct (measure and subtract overhead):**

```rust
// Measure timer overhead
let empty_start = clock.raw();
let empty_end = clock.raw();
let overhead = clock.delta(empty_start, empty_end);

// Subtract from each measurement
let actual_duration = measured_duration.saturating_sub(overhead);
```

### 1.6 Use Hardware Timestamps

**Impact: CRITICAL (software timers lack precision for nanosecond-scale measurement)**

Use TSC/RDTSC via the `quanta` crate for nanosecond precision. `std::time::Instant` uses OS syscalls with higher overhead (~20ns vs ~0ns with quanta upkeep thread).

**Platform behavior:**

| Platform | Best Timer |
|----------|------------|
| Linux | `CLOCK_MONOTONIC_RAW` (via quanta TSC) |
| Windows | `QueryPerformanceCounter` (via quanta) |
| macOS | `mach_absolute_time` (via quanta) |

**Notes:**
- TSC is x86/x86_64 only; quanta falls back to stdlib on ARM
- First `Clock::new()` blocks ~10ms for TSC calibration -- create once at startup
- Use `Clock::recent()` with upkeep thread for ultra-low overhead reads

```rust
use quanta::Clock;

let clock = Clock::new(); // Create once, reuse
let start = clock.raw();
operation();
let end = clock.raw();
let duration_ns = clock.delta(start, end);
```

---

## 2. Benchmarking

**Impact: HIGH**

Patterns for Criterion, Divan, and iai-callgrind benchmarks in Rust. Incorrect setup produces noisy or non-representative results.

### 2.1 Configure Criterion for Reliable Results

**Impact: HIGH (default Criterion settings produce noisy, non-representative measurements)**

Override Criterion defaults for latency-sensitive benchmarks: increase sample size to 10,000, extend measurement time, tighten significance and noise thresholds.

**Criterion reports confidence intervals around the estimated mean -- not true percentiles.** Do not label Criterion CI bounds as P99 or P50. For true latency percentiles, use HdrHistogram.

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_hot_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("hot-path");
    group.sample_size(10_000)
         .measurement_time(std::time::Duration::from_secs(10))
         .significance_level(0.01)
         .noise_threshold(0.02);

    let engine = setup_engine();

    group.bench_function("process", |b| {
        b.iter(|| black_box(engine.process(black_box(&input))))
    });

    // Parameterized benchmarks
    for count in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("batch", count),
            &count,
            |b, &n| b.iter(|| { /* ... */ }),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_hot_path);
criterion_main!(benches);
```

**Cargo.toml entry:**

```toml
[[bench]]
name = "hot_path"
harness = false
```

### 2.2 Use Divan for Allocation Tracking

**Impact: HIGH (hidden allocations in hot paths cause latency spikes)**

Divan's `AllocProfiler` tracks heap allocations per benchmark iteration, making it ideal for verifying zero-allocation hot paths. It also supports built-in multi-threaded contention benchmarks.

```rust
use divan::{Bencher, AllocProfiler};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn process_message(bencher: Bencher) {
    bencher
        .with_inputs(|| create_test_message())
        .bench_values(|msg| engine.process(&msg));
}

// Multi-threaded contention benchmark
#[divan::bench(threads = [1, 2, 4, 8])]
fn concurrent_submit(bencher: Bencher) {
    bencher.bench(|| queue.submit(create_item()));
}

fn main() {
    divan::main();
}
```

**When to use Divan vs Criterion:**

| Use Divan | Use Criterion |
|-----------|---------------|
| Zero-allocation verification | Baseline comparisons |
| Thread contention testing | Historical regression detection |
| New projects | Existing Criterion setup |

### 2.3 Use iai-callgrind for Deterministic CI

**Impact: HIGH (time-based benchmarks are noisy in CI; instruction counts are deterministic)**

iai-callgrind counts CPU instructions instead of wall-clock time, making it immune to CI environment noise (shared runners, CPU throttling, load spikes). Use it for automated regression gates.

```bash
# In CI (Linux runners only -- requires valgrind)
cargo bench --bench iai_benchmarks --features iai -- --regress ir::count=0%
```

Time-based benchmarks (Criterion) should run nightly on dedicated hardware as advisory, non-blocking checks. iai-callgrind gates block the merge.

---

## 3. Runtime Monitoring

**Impact: HIGH**

Continuous latency tracking with HdrHistogram and percentile validation. Ensures production systems stay within performance budgets.

### 3.1 Use HdrHistogram for Runtime Tracking

**Impact: HIGH (ad-hoc percentile code has edge-case bugs and lacks coordinated omission support)**

Use HdrHistogram with quanta for runtime latency tracking. HdrHistogram handles percentile calculation correctly, supports coordinated omission correction, and has constant memory usage regardless of sample count.

```rust
use quanta::Clock;
use hdrhistogram::Histogram;

pub struct LatencyTracker {
    clock: Clock,
    histogram: Histogram<u64>,
}

impl LatencyTracker {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            histogram: Histogram::new(5).unwrap(), // 5 significant digits
        }
    }

    pub fn measure<F, R>(&mut self, operation: F) -> R
    where F: FnOnce() -> R
    {
        let start = self.clock.raw();
        let result = operation();
        let end = self.clock.raw();
        let duration_ns = self.clock.delta(start, end);
        self.histogram.record(duration_ns).unwrap();
        result
    }

    pub fn report(&self) {
        println!("=== Latency Report ===");
        println!("P50:   {:>8.2}us", self.histogram.value_at_percentile(50.0) as f64 / 1000.0);
        println!("P95:   {:>8.2}us", self.histogram.value_at_percentile(95.0) as f64 / 1000.0);
        println!("P99:   {:>8.2}us", self.histogram.value_at_percentile(99.0) as f64 / 1000.0);
        println!("P99.9: {:>8.2}us", self.histogram.value_at_percentile(99.9) as f64 / 1000.0);
        println!("Max:   {:>8.2}us", self.histogram.max() as f64 / 1000.0);
    }

    pub fn check_budget(&self, budget_us: f64) -> bool {
        let p999_us = self.histogram.value_at_percentile(99.9) as f64 / 1000.0;
        p999_us <= budget_us
    }
}
```

### 3.2 Validate Latency Against Budgets

**Impact: HIGH (performance budgets without automated validation are ignored)**

Every performance budget should have an automated check that fails when the budget is exceeded. Use HdrHistogram percentile queries against defined targets.

```rust
use hdrhistogram::Histogram;

pub struct LatencyTargets {
    pub p50_ns: u64,
    pub p999_ns: u64,
}

pub fn validate_latency(
    measurements: &[std::time::Duration],
    targets: &LatencyTargets,
) -> Result<(), String> {
    let mut histogram: Histogram<u64> = Histogram::new(3).unwrap();
    for d in measurements {
        histogram.record(d.as_nanos() as u64).unwrap();
    }

    let p50 = histogram.value_at_percentile(50.0);
    let p999 = histogram.value_at_percentile(99.9);

    if p50 > targets.p50_ns {
        return Err(format!("P50 {}ns exceeds budget {}ns", p50, targets.p50_ns));
    }
    if p999 > targets.p999_ns {
        return Err(format!("P99.9 {}ns exceeds budget {}ns", p999, targets.p999_ns));
    }

    Ok(())
}
```

### 3.3 Automate Regression Detection

**Impact: HIGH (manual performance comparisons miss gradual regressions)**

Compare current benchmark results against stored baselines automatically. Flag regressions that exceed a threshold percentage on P99.9 latency. Store results as structured data (JSON) for historical tracking.

```rust
pub struct BenchmarkResult {
    pub component: String,
    pub operation: String,
    pub timestamp: u64,
    pub commit_hash: String,
    pub p50_ns: u64,
    pub p99_ns: u64,
    pub p999_ns: u64,
    pub sample_count: usize,
}

pub fn detect_regression(
    current: &BenchmarkResult,
    baseline: &BenchmarkResult,
    threshold_pct: f64,
) -> Option<String> {
    let regression_pct = ((current.p999_ns as f64 - baseline.p999_ns as f64)
        / baseline.p999_ns as f64) * 100.0;

    if regression_pct > threshold_pct {
        Some(format!(
            "{}/{}: P99.9 regressed {:.1}% ({} -> {} ns)",
            current.component, current.operation,
            regression_pct, baseline.p999_ns, current.p999_ns
        ))
    } else {
        None
    }
}
```

---

## 4. Profiling

**Impact: MEDIUM**

CPU profiling and flamegraph analysis for identifying hot spots. Guides optimization effort to the right locations.

### 4.1 Profile Before Optimizing

**Impact: MEDIUM (optimizing without data wastes effort on non-bottleneck code)**

Never optimize based on assumptions. Profile first with a sampling profiler to identify actual hot spots. Use flamegraphs for visualization.

**Recommended profilers:**

| Tool | Platform | Use For |
|------|----------|---------|
| `samply` | Linux, macOS | CPU profiling with Firefox Profiler UI, multi-thread timeline |
| `perf` | Linux | Low-overhead sampling, hardware counters |
| `cargo-flamegraph` | Cross-platform | Quick flamegraph generation |

```bash
# Install samply
cargo install samply

# Profile a benchmark
samply record cargo bench --bench hot_path -- --profile-time=5

# Generate flamegraph from perf data
cargo install flamegraph
cargo flamegraph --bench hot_path -- --profile-time=5
```

**Reference**: Brendan Gregg's flamegraph methodology -- [brendangregg.com/flamegraphs.html](https://www.brendangregg.com/flamegraphs.html)
