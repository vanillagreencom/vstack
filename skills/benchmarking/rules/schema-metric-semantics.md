---
title: Metric Kind Semantics
impact: MEDIUM
tags: schema, metric, semantics, interpretation
---

## Metric Kind Semantics

**Impact: MEDIUM (misinterpreting mean as percentile)**

- **wall_time** (Criterion): `estimate.stat = "mean"` is the estimated mean, NOT a true percentile. CI bounds are confidence interval on the mean, not tail latency.
- **instruction_count** (iai-callgrind): Deterministic Ir count. Immune to timing noise. Use for CI gates.
- **estimated_cycles** (iai-callgrind): Simulated cycles from Callgrind simulator, not real hardware counters.
- **latency_distribution** (HdrHistogram): True percentiles. Use with coordinated omission correction for throughput benchmarks.
