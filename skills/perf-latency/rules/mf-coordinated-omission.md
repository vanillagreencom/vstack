---
title: Account for Coordinated Omission
impact: CRITICAL
impactDescription: Benchmarks hide true latency spikes under load
tags: coordinated-omission, hdrhistogram, throughput, gil-tene
---

## Account for Coordinated Omission

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
