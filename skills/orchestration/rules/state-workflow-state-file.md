---
title: Durable Workflow State Files
impact: HIGH
impactDescription: State lost on context compaction — cannot track cycles, fixes, or agent sessions
tags: state, compaction, persistence
---

## Durable Workflow State Files

**Impact: HIGH (state lost on context compaction — cannot track cycles, fixes, or agent sessions)**

Use workflow state files (`tmp/workflow-state-[ID].json`) for any data that must survive context compaction: issue tracking, sub-issues, agent persistence, cycle counts, fix/escalation tracking, and audit trails. Use the `workflow-state` CLI for atomic reads/writes with flock-based locking to prevent corruption from concurrent access.
