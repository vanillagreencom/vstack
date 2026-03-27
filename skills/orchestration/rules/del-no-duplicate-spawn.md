---
title: No Duplicate Agent Spawns
impact: HIGH
impactDescription: Duplicate agents create conflicting work and waste resources
tags: delegation, spawn, lifecycle
---

## No Duplicate Agent Spawns

**Impact: HIGH (duplicate agents create conflicting work and waste resources)**

Never spawn a fresh agent when an existing one of the same type is alive — message it instead. Before creating agent tasks, check the task list for PENDING tasks with the same prefix. Completed tasks from prior rounds are not duplicates.

If an agent appears stuck: confirm the stall using session-level evidence per `life-wait-for-return` (quiet ≠ stalled — check activity signals, not just worktree state). Only after stall confirmed: shut down → respawn → re-create tasks → re-delegate. Never fix code yourself as the orchestrator.
