---
title: HashMap Lookup in Hot Path
impact: CRITICAL
impactDescription: +20-50ns per lookup
tags: performance, hot-path, hashmap, lookup
---

## HashMap Lookup in Hot Path

**Impact: CRITICAL (+20-50ns per lookup)**

`HashMap::get()` involves hashing, bucket traversal, and potential cache misses. In tight loops this adds measurable latency.

**Detection:** `HashMap` access patterns in data processing paths.

**Fix:** Array index with known-range keys, perfect hashing for static key sets, or pre-computed lookup tables populated at startup.
