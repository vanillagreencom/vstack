---
title: Test Quality
impact: HIGH
tags: testing, quality, naming, setup
---

## Test Quality

**Impact: HIGH (false confidence from misleading tests)**

- **Verify setup reaches target** — trace call chains before copying patterns. Early-throwing mocks can shadow later overrides.
- **Question existing patterns** — "parity" work propagates flaws silently. Verify originals are sound.
- **Names must match behavior** — `WhenXThrows` but X never runs = misleading test.
