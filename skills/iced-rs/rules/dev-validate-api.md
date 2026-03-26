---
title: Validate API Before Use
impact: HIGH
impactDescription: Silent compilation failures or runtime panics from 0.13 API assumptions
tags: api, docs, breaking_changes
---

## Validate API Before Use

**Impact: HIGH (silent compilation failures or runtime panics from 0.13 API assumptions)**

Iced 0.14 has significant breaking changes from 0.13. Always verify widget APIs, entry points, and trait signatures against current docs before assuming API shape.
