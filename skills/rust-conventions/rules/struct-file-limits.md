---
title: File Size Limits
impact: HIGH
tags: organization, splitting, file_size
---

## File Size Limits

**Impact: HIGH (files beyond tool limits can't be read in one pass)**

- Implementation: ≤1,500 lines per file. Approaching limit → split proactively by responsibility.
- Test files: ≤1,000 lines target, 1,500 hard limit.
- Split by contract, not by size: each module gets one clear responsibility and a minimal public API. Group by coupling (types + functions that change together stay together). Thin dispatchers stay in the parent; logic moves to focused modules.
- Never split types from the functions that exclusively operate on them.
