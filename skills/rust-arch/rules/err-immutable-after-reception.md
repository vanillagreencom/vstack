---
title: Data Immutability After Reception
impact: HIGH
impactDescription: mutation causes data corruption and replay inconsistency
tags: error-handling, immutability, data-integrity
---

## Data Immutability After Reception

**Impact: HIGH (mutation causes data corruption and replay inconsistency)**

Market data and other time-series inputs must be frozen after normalization. Mutating received data breaks audit trails, makes replays non-deterministic, and risks one consumer's transformation affecting another's view.

- Use `Copy` types for small value types (ticks, quotes, bars)
- Transformations create new instances; never mutate originals
- Freeze data after the normalization/parsing step
