---
title: Flaky Test Avoidance
impact: HIGH
tags: testing, flaky, concurrency, timing
---

## Flaky Test Avoidance

**Impact: HIGH (CI failures that can't be reproduced locally)**

- **Use signals, not iteration counts** — `while !done.load()` not `for _ in 0..10000`
- **Startup barriers before concurrent work** — ensure all threads ready before test begins
- **spin_loop() is not synchronization** — use `yield_now()`, channels, or condition variables
- **No static mutable state in tests** — use thread_local or per-test instances
- **Parallel tests must be isolated** — shared global state = flaky failures in CI
- **Drain loops bounded by known quantities** — track actual counts, not arbitrary iterations
- **Never rely on timing** — `sleep()` for synchronization is a bug waiting to happen
- **No probabilistic aggregate assertions** — each iteration must be self-contained
