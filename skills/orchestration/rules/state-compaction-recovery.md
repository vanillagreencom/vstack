---
title: Compaction Recovery Protocol
impact: HIGH
impactDescription: Workflow stalls or restarts from scratch after context compaction
tags: state, compaction, recovery
---

## Compaction Recovery Protocol

**Impact: HIGH (workflow stalls or restarts from scratch after context compaction)**

After context compaction, conversation history is discarded but external state persists. Recovery protocol:
1. Check the task list — find last completed task, resume from next
2. Read workflow state file for persistent data (team name, cycles, agent IDs)
3. If team-based: re-read team config from disk to restore member list (compaction may drop team awareness while agents are still alive)
4. Re-send delegation to existing agents. If no response after one idle cycle, only then respawn.

Never repeat completed actions. Supplement with visible outputs captured in state files.
