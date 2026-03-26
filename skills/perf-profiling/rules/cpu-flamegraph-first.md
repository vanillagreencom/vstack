---
title: Flamegraph First
impact: HIGH
impactDescription: Time wasted on wrong optimization targets without visual hot path analysis
tags: flamegraph, perf, profiling, hot-path
---

## Flamegraph First

**Impact: HIGH (time wasted on wrong optimization targets without visual hot path analysis)**

Start CPU profiling with flamegraphs to identify the widest functions before diving into specific counters. Wide plateaus in flamegraphs are your optimization targets.

```bash
# cargo-flamegraph (easiest for Rust projects)
cargo install flamegraph
cargo flamegraph --bench my_benchmark
cargo flamegraph -- --release

# Manual perf + flamegraph.pl
perf record -F 99 -g -- ./target/release/my_app
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg

# With DWARF call graphs (more accurate for optimized binaries, slower)
perf record -F 99 --call-graph dwarf -- ./target/release/my_app
```

**Reading flamegraphs:**
- Width = time spent (wider = more CPU time)
- Y-axis = stack depth (bottom = entry point)
- Look for wide plateaus = optimization targets
- Narrow deep stacks are usually fine; wide shallow ones are the problem
