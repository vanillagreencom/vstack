---
title: Wait for Agent Return Before Acting
impact: CRITICAL
impactDescription: Premature intervention corrupts agent work in progress
tags: lifecycle, idle, intervention
---

## Wait for Agent Return Before Acting

**Impact: CRITICAL (premature intervention corrupts agent work in progress)**

After delegation, wait for the agent's return message. Idle notifications are normal — agents go idle between turns while working. On idle notification, check the task list for tasks with the agent's prefix:
- Any in-progress → **do nothing, go idle** (agent is working)
- All completed → done, proceed (return message should have arrived)
- All pending (none claimed) → re-send delegation ONCE, then wait one full agent turn. Only then re-check — if still all pending, respawn.

Never re-send or intervene while any task is in-progress.
