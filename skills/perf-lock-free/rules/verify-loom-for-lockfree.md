---
title: Loom Tests Required for All Lock-Free Structures
impact: CRITICAL
impactDescription: Concurrency bugs undetectable by other tools
tags: loom, lock-free, spsc, atomics, fences
---

## Loom Tests Required for All Lock-Free Structures

**Impact: CRITICAL (concurrency bugs undetectable by other tools)**

Every lock-free structure (SPSC queues, ring buffers, atomic state machines) MUST have loom tests. Loom is the only tool that can verify correctness of atomic fence patterns by exploring all possible thread interleavings.

Use `LOOM_MAX_PREEMPTIONS=2` for CI (sufficient for 2-thread SPSC). Use 3 for deep runs on critical structures.

```bash
# CI standard
LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom --release

# Thorough (for critical structures)
LOOM_MAX_PREEMPTIONS=3 RUSTFLAGS="--cfg loom" cargo test --features loom --release

# Debug failures
LOOM_LOG=trace LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --features loom
```
