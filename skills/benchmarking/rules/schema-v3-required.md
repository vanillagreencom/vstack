---
title: Schema v3 Required
impact: MEDIUM
tags: schema, format, version
---

## Schema v3 Required

**Impact: MEDIUM (older formats cause parsing failures)**

All benchmark results must use schema v3 format with `"schema_version": 3`. Legacy flat-file formats are auto-converted on `record` but should not be used for new results.
