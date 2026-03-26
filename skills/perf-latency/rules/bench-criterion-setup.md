---
title: Configure Criterion for Reliable Results
impact: HIGH
impactDescription: Default Criterion settings produce noisy, non-representative measurements
tags: criterion, benchmark, sample-size, configuration
---

## Configure Criterion for Reliable Results

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
