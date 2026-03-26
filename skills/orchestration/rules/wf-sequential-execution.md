---
title: Sequential Section Execution
impact: CRITICAL
impactDescription: Skipped steps cause incomplete work and broken state
tags: workflow, execution, ordering
---

## Sequential Section Execution

**Impact: CRITICAL (skipped steps cause incomplete work and broken state)**

Process sections sequentially: mark in-progress, execute all sub-sections within the section, mark completed, then proceed to next. Never create tasks for sub-sections — they are steps within the parent task, not separate tasks. Never mark a parent section complete before all sub-sections are executed.

Never skip steps because the outcome seems predictable, or rationalize skipping based on change scope ("test-only", "small", "only N items", "already reviewed"). The workflow text is the decision authority, not the agent's assessment.
