---
title: Test Feature Gates
impact: HIGH
tags: testing, cfg, features, miri
---

## Test Feature Gates

**Impact: HIGH (tests run in wrong contexts or skip silently)**

- **Infrastructure features**: Module-level gating (`#[cfg(all(test, feature = "X"))]` on module declaration), not per-item `#[cfg]` on each test.
- **MIRI/sanitizer gating**: Test modules that don't exercise unsafe code → `#[cfg(all(test, not(miri)))]`. MIRI/ASAN only detect UB and memory errors in unsafe code.
- **Loom ordering models**: In-source `#[cfg(loom)] mod loom_tests` permitted in `#[path]` test files for simplified atomic ordering models co-located with the code they verify.
