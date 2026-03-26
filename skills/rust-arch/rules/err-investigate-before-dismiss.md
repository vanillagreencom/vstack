---
title: Investigate Errors Before Dismissal
impact: HIGH
impactDescription: silently broken functionality goes unnoticed
tags: error-handling, debugging, investigation
---

## Investigate Errors Before Dismissal

**Impact: HIGH (silently broken functionality goes unnoticed)**

Never dismiss errors, warnings, or unexpected behavior without investigation. Errors that "seem harmless" often indicate silently broken functionality discovered too late.

Investigation checklist:
1. **Trace to source** — where is this coming from?
2. **Understand intent** — what was supposed to happen?
3. **Verify impact** — is functionality silently broken?
4. Only dismiss after confirming harmless (document why in a comment)
