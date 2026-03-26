---
title: Extract Shared Computation
impact: HIGH
tags: duplication, extraction, helpers
---

## Extract Shared Computation

**Impact: HIGH (duplication = guaranteed divergence on next edit)**

If two functions compute the same derived values (geometry, dimensions, layout), extract to a shared struct or helper.
