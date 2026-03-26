---
title: Single Source of Initialization
impact: HIGH
tags: constructors, initialization, duplication
---

## Single Source of Initialization

**Impact: HIGH (duplicate field lists drift silently)**

When two constructors initialize the same fields, one must call the other or both call a shared helper.
