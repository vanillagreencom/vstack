---
title: Bundled Issue Task Structure
impact: MEDIUM
impactDescription: Per-section tasks break when multiple sub-issues share the same workflow sections
tags: coordination, bundled, multi-issue
---

## Bundled Issue Task Structure

**Impact: MEDIUM (per-section tasks break when multiple sub-issues share the same workflow sections)**

When a parent issue has sub-issues assigned to the same agent, create one composite task per sub-issue covering all relevant sections, not one task per section. The task system doesn't support looping — if you create per-section tasks, the first sub-issue completes all section tasks and subsequent sub-issues have no tasks to track.

The spawn prompt's "Multi-section tasks" instruction tells agents to execute all referenced sections for one sub-issue, then mark the single task complete. The "Do NOT loop to other sub-issues" instruction prevents agents from processing multiple sub-issues in a single task.
