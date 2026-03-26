---
title: Unsafe Block Checklist
impact: CRITICAL
impactDescription: Missed unsafe blocks are unaudited attack surface
tags: unsafe, checklist, audit, miri
---

## Unsafe Block Checklist

**Impact: CRITICAL (missed unsafe blocks are unaudited attack surface)**

For each `unsafe` block in the codebase:

- [ ] SAFETY comment present and complete (covers all applicable invariants)
- [ ] All pointer operations validated (null checks, bounds checks)
- [ ] No undefined behavior possible (verified by analysis or MIRI)
- [ ] MIRI test coverage exists (only meaningful if the block exercises unsafe operations)
- [ ] Panic paths don't leave invalid state (no partial writes visible after unwind)

Run an unsafe inventory tool to enumerate all unsafe blocks and verify none are missed during audit.
