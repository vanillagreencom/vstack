---
title: Always Capture Environment
impact: HIGH
impactDescription: Can't detect environment-caused regressions
tags: recording, environment, platform
---

## Always Capture Environment

**Impact: HIGH (can't detect environment-caused regressions)**

Every recorded result must include `environment` with `platform`, `kernel`, `cpu_model`, and `cpu_governor`. This is auto-populated by `bench.sh record` but must be verified for manually recorded results.
