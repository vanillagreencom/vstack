---
title: Profile Before Optimizing
impact: MEDIUM
impactDescription: Optimizing without data wastes effort on non-bottleneck code
tags: profiling, flamegraph, samply, optimization
---

## Profile Before Optimizing

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
