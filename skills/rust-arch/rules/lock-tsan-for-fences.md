---
title: TSAN for Fence-Based Code
impact: HIGH
impactDescription: false confidence in correctness
tags: concurrency, tsan, fence, testing
---

## TSAN for Fence-Based Code

**Impact: HIGH (false confidence in correctness)**

Thread Sanitizer (TSAN) does not understand `std::sync::atomic::fence()` synchronization patterns. It will report false negatives (no warnings) for code that has real data races, and false positives for correct fence usage.

**Detection:** Using TSAN to validate code that relies on explicit fences for synchronization.

**Fix:** Use loom for verification of fence-based synchronization. Loom models the memory ordering rules correctly and explores possible interleavings.
