---
title: Re-baseline Before Treating Environment Changes as Code Regressions
impact: CRITICAL
tags: regression, environment, baseline, platform
---

## Re-baseline Before Treating Environment Changes as Code Regressions

**Impact: CRITICAL (false regression reports from kernel/CPU/governor changes)**

When regression coincides with environment change warnings (kernel, CPU model, or governor differ between baseline and latest), re-baseline on the current environment before treating as a code regression. Report with env diff.
