---
title: Early Return on No-Op
impact: HIGH
tags: performance, early_return, view
---

## Early Return on No-Op

**Impact: HIGH (wasted computation every frame)**

Functions called every frame (view, overlay builders) must early-return when their output is unused (e.g., no active drag → skip overlay construction).
