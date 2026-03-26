---
title: Benchmarks for Hot Paths
impact: HIGH
tags: benchmarks, criterion, performance, hot_path
---

## Benchmarks for Hot Paths

**Impact: HIGH (performance regressions go undetected)**

When adding performance-sensitive code, add a Criterion benchmark. Benchmarked types must be `pub` (benches/ is an external crate). If modifying an existing hot path, verify the existing benchmark covers your change. Integration tests go in `tests/`; benchmarks go in `benches/`.
