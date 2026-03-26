---
title: Loom Test Coverage
impact: HIGH
impactDescription: Untested lock-free code has undetectable ordering bugs
tags: lock-free, loom, testing, concurrency
---

## Loom Test Coverage

**Impact: HIGH (untested lock-free code has undetectable ordering bugs)**

Every lock-free data structure (SPSC queues, ring buffers, atomic structures) must have loom tests that pass. Loom exhaustively explores thread interleavings to find ordering bugs that are nearly impossible to reproduce with standard testing.
