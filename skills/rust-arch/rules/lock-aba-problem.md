---
title: ABA Problem
impact: HIGH
impactDescription: silent corruption in CAS loops
tags: concurrency, cas, aba, lock-free
---

## ABA Problem

**Impact: HIGH (silent corruption in CAS loops)**

Compare-and-swap (CAS) succeeds if the current value matches the expected value. If a value changes from A to B and back to A between the read and CAS, the CAS succeeds despite the intermediate mutation — potentially corrupting data structures.

**Detection:** CAS loops without generation counters or hazard pointers.

**Fix:** Use tagged pointers (pack a generation counter into the pointer) or hazard pointer schemes that detect intermediate modifications.
