---
title: Orchestrator Never Fixes Code
impact: HIGH
impactDescription: Orchestrator code changes bypass domain expertise and review pipeline
tags: lifecycle, delegation, orchestrator
---

## Orchestrator Never Fixes Code

**Impact: HIGH (orchestrator code changes bypass domain expertise and review pipeline)**

The orchestrator never edits or writes code in the worktree, uses bare `cd`, or fixes code directly. Always delegate to the domain agent. If the agent appears stuck, follow the re-delegation protocol: shut down → respawn → re-create tasks → re-delegate. The orchestrator may run read-only commands (git status, diff summaries) and invoke scripts.
