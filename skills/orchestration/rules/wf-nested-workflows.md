---
title: Nested Workflow Invocation
impact: CRITICAL
impactDescription: Ad-hoc substitution breaks task tracking and recovery
tags: workflow, nested, subflow
---

## Nested Workflow Invocation

**Impact: CRITICAL (ad-hoc substitution breaks task tracking and recovery)**

Nested workflows (marked with `⤵`) must be invoked through the harness's workflow invocation mechanism — never inlined or substituted with ad-hoc commands. Pre-create tasks using `workflow-sections --subflow "/command"`. If the marker includes a return point (`→ § X`), pass `--return "§ X"` so task descriptions contain the return point for compaction resilience.
