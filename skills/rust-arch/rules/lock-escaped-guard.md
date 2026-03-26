---
title: Escaped Guard Lifetime
impact: HIGH
impactDescription: use-after-free in concurrent code
tags: concurrency, crossbeam, guard, lifetime
---

## Escaped Guard Lifetime

**Impact: HIGH (use-after-free in concurrent code)**

Crossbeam epoch-based reclamation guards protect memory from deallocation while the guard is alive. If a reference obtained under a guard escapes the guard's scope, the memory can be reclaimed while still referenced.

**Detection:** References derived from crossbeam `Guard`-protected loads that outlive the guard scope.

**Fix:** Process all data within the guard scope. Clone or copy needed values before dropping the guard.
