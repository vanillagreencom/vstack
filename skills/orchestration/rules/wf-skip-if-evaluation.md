---
title: Skip-If Condition Evaluation
impact: HIGH
impactDescription: Skipped steps not visible in task list, blocking downstream detection
tags: workflow, skip, conditions
---

## Skip-If Condition Evaluation

**Impact: HIGH (skipped steps not visible in task list, blocking downstream detection)**

When a section starts with "Skip if [condition]", evaluate the condition literally. If true, append "(SKIPPED)" to the task subject and mark completed. The workflow decides what to skip, not the agent. This makes skipped steps visible to the orchestrator and to post-compaction recovery via the task list.
