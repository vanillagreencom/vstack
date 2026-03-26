---
title: Hot-Path Regressions Block Merge
impact: CRITICAL
impactDescription: Performance regression in critical path reaches production
tags: regression, hot_path, merge, blocker
---

## Hot-Path Regressions Block Merge

**Impact: CRITICAL (performance regression in critical path reaches production)**

Operations on the critical execution path that regress beyond threshold are merge blockers. Must be fixed before merge. Never dismiss as "measurement artifact" without evidence (e.g., cross-platform mismatch warning, known timer floor issue).
