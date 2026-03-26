---
title: Classify Every Regression
impact: CRITICAL
impactDescription: Missed hot-path blockers ship to production
tags: regression, classification, reporting
---

## Classify Every Regression

**Impact: CRITICAL (missed hot-path blockers ship to production)**

When `bench.sh regression` exits with code 1, every regressed operation must be classified as hot-path, cold-path, intentional, or environmental. Silent omission is forbidden. If uncertain about classification, default to hot-path (blocker).
