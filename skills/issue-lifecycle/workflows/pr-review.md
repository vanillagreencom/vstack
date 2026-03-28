# PR Review Lifecycle

> **Dependencies**: orchestration skill (recommendation-bias patterns, review-finding schema)

**The workflow for PR review agents (security-review, test-review, doc-review, error-review, structure-review).**

PR review agents are pre-submission reviewers. They run in parallel, each reviewing the same diff from their specialist perspective.

**Ownership**: You review ONE PR. Return verdict to orchestrator. No issue tracker state changes.

---

## 1. Review Changes

Extract from delegation message:
- `Worktree` path
- `Branch` name
- `Decisions` to respect
- Re-review context (if any)

### 1.1 Diff

```bash
BASE_BRANCH=${WORKTREE_DEFAULT_BRANCH:-$(git -C [WORKTREE_PATH] symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')}
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=main
git -C [WORKTREE_PATH] diff "origin/$BASE_BRANCH"...HEAD
```

Review for noteworthy findings only — skip minor style issues. Exclude research documents.
If a changed path was deleted, inspect it from the git diff or git history; do not try to `Read` the deleted working-tree path directly.

### 1.2 Read Decisions

Read decision files listed in delegation. Do NOT suggest changes that contradict them.

### 1.3 Classify Findings

Read the orchestration skill's recommendation-bias patterns. Apply its decision flow to ALL findings — a finding must pass actionability and relatedness checks before entering `blockers[]` or `suggestions[]`. Then use size to categorize suggestions as `fix` or `issue`.

### 1.4 Handle Re-Review (if applicable)

**Skip if** not a re-review cycle (no "re-review" section in delegation).

Items listed as fixed or escalated are already resolved — do NOT re-report them. Only report NEW issues or regressions introduced by the fixes.

### 1.5 Return JSON Report

Build JSON per the orchestration skill's review-finding schema. Save to `[WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json`.

**Verdict rules:**
- `action_required`: 1+ items in `blockers[]`
- `pass`: `blockers[]` empty

### 1.6 Return

**Return exactly** (return to orchestrator):

<output_format>
Verdict: [pass|action_required]
File: [WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json
```json
{complete JSON object}
```
</output_format>

---

## Constraints

**Do NOT**:
- Modify issue tracker state (labels, status)
- Create commits or push changes
- Call other subagents

**Orchestrator handles**: All issue tracker updates, routing blockers back to dev agent, merging JSONs, presentation.
