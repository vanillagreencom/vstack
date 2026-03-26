---
name: issue-lifecycle
description: Agent workflows for issue implementation, review fix delegation, pre-submission PR review, and QA review. Designed for specialist agents receiving delegations from an orchestrator.
license: MIT
user-invocable: true
dependencies:
  required: [linear, orchestration, decider]
  optional: [benchmarking, visual-qa]
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Issue Lifecycle

Agent workflows for issue implementation, review fix delegation, pre-submission PR review, and QA review. Designed for specialist agents receiving delegations from an orchestrator.

## When to Apply

Reference these workflows when:
- A dev agent receives an `Issue: [ISSUE_ID]` delegation (single or bundled)
- A dev agent receives review fix items to address
- A PR review agent is delegated a pre-submission review
- A QA agent is triggered via `needs-*` labels
- An agent needs to block/unblock issues due to cross-domain dependencies
- An agent completes implementation and must post structured completion summaries

## Skill Dependencies

Workflows reference these companion skills and tools. Install and configure per your project:

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments, labels | `$ISSUE_CLI` |
| Orchestration skill | Review-finding schema, recommendation-bias patterns | Referenced by name |
| Decider skill | Decision templates, search CLI, creation workflows | `$DECISIONS_CMD` |
| Benchmarking skill (optional) | Baseline capture, regression classification, recording | `$BENCH_CLI`, `$BENCH_PARSER` |
| Visual QA skill (optional) | Screenshot capture, interactive testing | `$VISUAL_QA_CLI`, `$SCREENSHOT_CLI` |

Project-level configuration:

| Variable | Purpose |
|----------|---------|
| `$VALIDATE_CMD` | Build + test + lint command |
| `$DECISIONS_CMD` | Decision document lookup (optional) |
| `$DIFF_SUMMARY_CMD` | Diff summary with domain grouping (optional) |

## Workflows

| Workflow | Agent Type | Purpose |
|----------|------------|---------|
| `workflows/dev-implement.md` | Dev agents | Full implementation lifecycle: activate → plan → implement → validate → commit → QA labels → summary → finalize (§ 1-11) |
| `workflows/dev-fix.md` | Dev agents | Process review fix items: evaluate → apply/skip → validate → commit → return |
| `workflows/pr-review.md` | Review agents | Pre-submission PR review: diff → classify findings → JSON report → verdict |
| `workflows/qa-review.md` | QA agents | QA label-triggered review: context → agent review → benchmark recording → JSON report → verdict |

## References

| Topic | Source |
|-------|--------|
| Review finding schema | Orchestration skill (`schemas/review-finding.md`) |
| Recommendation bias | Orchestration skill (`workflows/recommendation-bias.md`) |
| Label application | Project label application guide |
| Benchmark baselines | Benchmarking skill (`workflows/issue-baselines.md`) |
| Regression classification | Benchmarking skill |

## Configuration

This skill is workflow-based (no `rules/` directory). All behavior is defined in the workflow files.

Agent types referenced in workflows:
- **Dev agents**: `[AGENT_TYPE]` — specialist agents receiving implementation delegations
- **Review agents**: `security-review`, `test-review`, `doc-review`, `error-review`, `structure-review`
- **QA agents**: `safety`, `perf-qa`, `arch-review`

Commit format: `[PREFIX]([ISSUE_ID]): [DESCRIPTION]` — configurable per project conventions.

## Full Compiled Document

For the complete guide with all workflows expanded inline: `AGENTS.md`
