---
title: Cold-Path Regressions Must Be Reported
impact: CRITICAL
tags: regression, cold_path, reporting
---

## Cold-Path Regressions Must Be Reported

**Impact: CRITICAL (hidden regressions accumulate silently)**

Cold-path regressions (startup, teardown, configuration, initialization) must be reported with justification. They appear in the regression report but are blockers only if a performance budget exists for that operation.
