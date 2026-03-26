---
title: Severity Classification
impact: MEDIUM
impactDescription: Inconsistent classification delays critical fixes
tags: severity, classification, audit, process
---

## Severity Classification

**Impact: MEDIUM (inconsistent classification delays critical fixes)**

Classify every audit finding using these severity levels:

| Severity | Definition | Action |
|----------|------------|--------|
| CRITICAL | UB in production code path, exploitable | BLOCK merge, immediate fix |
| HIGH | UB in edge case, potential crash | BLOCK merge, fix required |
| MEDIUM | Unsafe code without SAFETY comment, unclear invariants | May merge with follow-up issue |
| LOW | Style issues, missing docs, non-blocking | Create follow-up issue |

CRITICAL and HIGH findings must block merge. MEDIUM findings may proceed with a tracked follow-up. LOW findings are advisory.
