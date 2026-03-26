---
title: Create Tasks Before Spawning Agents
impact: CRITICAL
impactDescription: Agents wake to empty task list, waste turns, may produce incorrect work
tags: delegation, spawn, ordering
---

## Create Tasks Before Spawning Agents

**Impact: CRITICAL (agents wake to empty task list, waste turns, may produce incorrect work)**

Always create tasks before spawning an agent. The pattern is: create tasks (no owner assignment) → spawn agent (behavioral prompt only, goes idle) → send delegation message (includes task prefix). The agent wakes on the delegation message, checks the task list, and finds PENDING tasks matching the prefix.

For re-delegation to an existing agent: create new tasks → send delegation message. The agent wakes, finds NEW PENDING tasks by prefix (prior round's tasks are already completed — not matched).
