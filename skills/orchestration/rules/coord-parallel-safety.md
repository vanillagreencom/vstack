---
title: Parallel Work Safety Analysis
impact: MEDIUM
impactDescription: Parallel agents modify overlapping files, causing merge conflicts or broken state
tags: coordination, parallel, safety
---

## Parallel Work Safety Analysis

**Impact: MEDIUM (parallel agents modify overlapping files, causing merge conflicts or broken state)**

Before running issues in parallel, verify safety across five dimensions:
1. **Dependency resolution** — direct blocks/blockedBy, shared blockers, research blockers
2. **Agent overlap** — same agent working on multiple issues risks file conflicts
3. **Code scope** — analyze file paths, modules, and type/value flows for overlap
4. **Build config** — manifest file changes create hard separations
5. **Active work** — check for existing worktrees and open PRs on the same code

Grouping constraints: limit concurrent issues, limit same-agent issues per group, and treat manifest conflicts as hard separations.
