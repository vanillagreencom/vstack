---
title: Wait for Agent Return Before Acting
impact: CRITICAL
impactDescription: Premature intervention corrupts agent work in progress
tags: lifecycle, idle, intervention, stall-detection
---

## Wait for Agent Return Before Acting

**Impact: CRITICAL (premature intervention corrupts agent work in progress)**

After delegation, wait for the agent's return message. Idle notifications are normal — agents go idle between turns while working. On idle notification, check the task list for tasks with the agent's prefix:
- Any in-progress → **do nothing, go idle** (agent is working)
- All completed → done, proceed (return message should have arrived)
- All pending (none claimed) → re-send delegation ONCE, then wait one full agent turn. Only then re-check — if still all pending, respawn.

Never re-send or intervene while any task is in-progress.

### Quiet ≠ Stalled

Implementation agents routinely spend 5-15 minutes reading docs, planning, and analyzing code before producing any file changes. An agent making read/search calls with zero writes is in research/planning — real progress, not a stall.

**Minimum quiet window**: 10 minutes from delegation before entering escalation. No "simple task" exceptions.

**Invalid stall signals** (never sufficient alone or combined): return-message timeout, clean `git status`/`git diff`/`git log`, no modified files. These observe worktree state only — agents may be reading, planning, or working in a context not yet reflected in file changes.

**Stall confirmation required before shutdown.** Verify the agent is inactive using session-level evidence beyond worktree state:
- **Task-based harnesses** (e.g., Claude Code): task status unchanged across multiple idle cycles — no task claimed or progressed
- **Session-file harnesses** (e.g., Codex, OpenCode): no new entries in session log for 10+ minutes; check last timestamp and tool-call count in the session JSONL
- **Process-level**: agent process exited or consuming zero CPU for extended period

**Escalation sequence** (only after quiet window elapsed AND stall confirmed):
1. Re-message once with clarification specifying the missing step.
2. Wait 5 min. Re-check activity signals. New activity → go idle.
3. Still inactive → shut down → respawn → re-create tasks → re-delegate.
