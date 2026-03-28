# Orchestration

Multi-agent session coordination — front-to-back issue workflows, delegation patterns, workflow state management, review pipelines, and parallel work safety analysis.

## Structure

### Workflows (Session Lifecycle)
- `workflows/initialize.md` - Team setup, auth, cache, state init
- `workflows/start.md` - Dashboard, issue selection, research eval, worktree creation
- `workflows/start-worktree.md` - Full session: dev → review → submit → finalize
- `workflows/start-new.md` - Create new issue, spawn worktree session

### Workflows (Development)
- `workflows/dev-start.md` - Delegate implementation to specialist agents
- `workflows/dev-fix.md` - Delegate fix items to dev agents
- `workflows/ci-fix.md` - Analyze and fix CI failures

### Workflows (Review & Submission)
- `workflows/review-pr.md` - Pre-submission review with fix handling and QA
- `workflows/review-pr-comments.md` - Triage PR review comments via domain agents
- `workflows/submit-pr.md` - Push, create PR, bot review, comment triage, CI
- `workflows/merge-pr.md` - Verify conditions and merge PR(s)

### Workflows (Planning & Analysis)
- `workflows/audit-issues.md` - Audit issues for relations, hierarchy, gaps
- `workflows/fix-reconcile.md` - Check if fixes address existing open issues
- `workflows/post-summary.md` - Post summary and handoff comments
- `workflows/parallel-check.md` - Verify parallel work safety
- `workflows/cycle-plan.md` - Plan development cycles
- `workflows/roadmap-plan.md` - Consult specialists, analyze roadmap
- `workflows/roadmap-create.md` - Execute roadmap plan

### Workflows (Research)
- `workflows/research-issue.md` - Create research issue with assets
- `workflows/research-complete.md` - Route completed research to workflows
- `workflows/research-spike.md` - Quick research exploration

### Workflows (Reference)
- `workflows/agent-sequencing.md` - Cross-domain blocking relations
- `workflows/recommendation-bias.md` - Review finding categorization (fix vs issue)

### Scripts
- `scripts/workflow-sections` - Parse workflow markdown headers → JSON for task creation
- `scripts/workflow-state` - Read/write persistent state files with atomic locking

### Schemas
- `schemas/workflow-state.md` - Persistent state file schema
- `schemas/review-finding.md` - Review/QA agent JSON output format
- `schemas/audit-issues-input.md` - Input for issue audit workflows
- `schemas/roadmap-plan-input.md` - Input for roadmap planning

### Rules
- `rules/` - Individual rule files with frontmatter
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Skill Dependencies

| Dependency | Purpose |
|------------|---------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments |
| Git host CLI (e.g., `github` skill) | PR operations, CI status |
| Worktree CLI (e.g., `worktree` skill) | Create/remove git worktrees |
| Issue lifecycle skill | Dev implement/fix/review agent workflows |
| Project management skill | TPM audit/cycle/roadmap agent workflows |

## Configuration

Set these in `.env.local` or export them in the shell that runs the workflow. `.env.local.example` in the repo root includes optional Visual QA helpers plus launcher/review settings. The orchestration helper scripts source `.env.local` automatically when present.

| Variable | Purpose | Default |
|----------|---------|---------|
| `ORCH_STATE_DIR` | Override state file directory | `tmp` |
| `$ISSUE_CLI` | Issue tracker CLI path | — |
| `$GIT_HOST_CLI` | Git host CLI path | — |
| `$WORKTREE_CLI` | Worktree CLI path | — |
| `$VALIDATE_CMD` | Build + test + lint command | — |
| `$VISUAL_QA_BASELINE_CMD` | Optional helper to route baseline capture to a baseline-capable target | — |
| `$ISSUE_PATTERN` | Issue ID regex for branch names | — |
| `$BOT_REVIEWERS` | Comma-separated review bot usernames | — |
| `$BOT_CHECK_NAME` | Optional CI check name for early review detection | — |
| `$HARNESS_CMD` | Optional `parallel-launch` command template with `{issue}` placeholder | — |

## Creating a New Rule

1. Choose the category (see `rules/_sections.md`)
2. Use prefix: `wf-`, `del-`, `life-`, `state-`, `coord-`, `rev-`
3. Copy `rules/_template.md`
4. Fill frontmatter and rule body
5. Add to Quick Reference in `SKILL.md` and expand in `AGENTS.md`

## Rule File Structure

```markdown
---
title: Rule Title
impact: CRITICAL|HIGH|MEDIUM|LOW
impactDescription: One-line consequence of violation
tags: tag1, tag2
---

## Rule Title

**Impact: LEVEL (consequence)**

Explanation and why it matters.
```

## Impact Levels

- **CRITICAL** - Workflow failure, lost state, agents producing incorrect work
- **HIGH** - Degraded performance, context loss, wasted resources
- **MEDIUM** - Suboptimal coordination, noise in tracking, missed findings
- **LOW** - Style/convention deviation
