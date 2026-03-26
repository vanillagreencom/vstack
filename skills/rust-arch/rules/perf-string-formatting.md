---
title: String Formatting in Hot Path
impact: CRITICAL
impactDescription: +100-500ns per format call
tags: performance, hot-path, string, formatting
---

## String Formatting in Hot Path

**Impact: CRITICAL (+100-500ns per format call)**

`format!()`, `to_string()`, and string interpolation allocate heap memory and invoke the formatting machinery on every call.

**Detection:** `format!()`, `.to_string()`, string concatenation in latency-sensitive paths.

**Fix:** Pre-allocated buffers written at startup, or `write!()` into a reusable buffer. For logging, use compile-time disabled macros or sampling in hot paths.
