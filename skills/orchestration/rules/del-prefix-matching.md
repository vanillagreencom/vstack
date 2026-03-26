---
title: Task Prefix Matching
impact: HIGH
impactDescription: Agents claim wrong tasks or miss their assignments
tags: delegation, prefix, task-list
---

## Task Prefix Matching

**Impact: HIGH (agents claim wrong tasks or miss their assignments)**

The task prefix from `workflow-sections` JSON output must be used exactly in delegation messages — never hand-written. Agents filter the task list by prefix + PENDING status, so the delegation message prefix must match the task subjects exactly.

Task prefix hierarchy:
- Top-level: `§ N: Title`
- Nested sub-workflow: `⏤⤵ /command § N: Title`
- Agent delegation: `⏤⏤🐲 agent-name § N: Title`
- Inline tracking: `⏤⏤🐲 agent-name: Description`

Agents only process PENDING tasks — completed and in-progress tasks from other agents or prior rounds are ignored.
