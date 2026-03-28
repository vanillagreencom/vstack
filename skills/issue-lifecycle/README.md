# Issue Lifecycle

Agent workflows for issue implementation, review fix delegation, pre-submission PR review, and QA review. Designed for specialist agents receiving delegations from an orchestrator.

## Structure

```
skills/issue-lifecycle/
├── SKILL.md              # Quick-reference index for Claude Code and skill-aware harnesses
├── AGENTS.md             # Full compiled document for Codex, Copilot, Gemini CLI, and
│                         # 20+ harnesses. All workflows expanded inline.
├── README.md             # This file — human-facing docs
└── workflows/
    ├── dev-implement.md   # Main implementation lifecycle (§ 1-11)
    ├── dev-fix.md         # Review fix delegation workflow (§ 1-6)
    ├── pr-review.md       # Pre-submission PR review workflow (§ 1 + Constraints)
    └── qa-review.md       # QA label-triggered review workflow (§ 1-3 + Constraints)
```

This skill is workflow-based — there is no `rules/` directory. All behavior is defined in the workflow files.

## Workflows

### dev-implement.md

The main workflow for dev agents receiving `Issue: [ISSUE_ID]` delegations. Supports both single-issue and bundled (parent + sub-issues) flows. Covers the full lifecycle: environment setup, issue activation, research context, feasibility evaluation, implementation, validation, visual QA, skill reflection, commit, QA label application, completion summary, and finalization.

### dev-fix.md

The workflow for dev agents receiving review fix delegations. Each review item is evaluated independently against project decisions and conventions, then applied or skipped with reasoning. Includes validation, visual QA for UI fixes, and structured return with per-item decisions.

### pr-review.md

The workflow for pre-submission review agents (security-review, test-review, doc-review, error-review, structure-review). Agents review the diff, classify findings using the orchestration skill's recommendation-bias patterns, and return a structured JSON report with a pass/action_required verdict.

### qa-review.md

The workflow for QA agents (safety, perf-qa, arch-review) triggered via `needs-*` labels. Includes decision context checking, agent-specific review execution, benchmark regression classification and recording (perf-qa), and structured JSON report output.

## Skill Dependencies

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments, labels | `$ISSUE_CLI` |
| Orchestration skill | Review-finding schema, recommendation-bias patterns | Referenced by name |
| Decider skill | Decision templates, search CLI, creation workflows | `$DECISIONS_CMD` |
| Benchmarking skill (optional) | Baseline capture, regression classification, recording | `$BENCH_CLI`, `$BENCH_PARSER` |
| Visual QA skill (optional) | Screenshot capture, interactive testing, target routing | `$VISUAL_QA_CLI`, `$SCREENSHOT_CLI`, `$VISUAL_QA_TARGET_CMD` |

## Configuration

Set these in `.env.local` or export them in the shell that runs the workflow. `.env.local.example` in the repo root shows a minimal pattern.

### Project-level variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `$ISSUE_CLI` | Issue tracker CLI command | Yes |
| `$VALIDATE_CMD` | Build + test + lint command | Yes |
| `$DECISIONS_CMD` | Decision document lookup | Optional |
| `$DIFF_SUMMARY_CMD` | Diff summary with domain grouping | Optional |
| `$BENCH_CLI` | Benchmark CLI command | Optional |
| `$BENCH_PARSER` | Benchmark output parser | Optional |
| `$VISUAL_QA_CLI` | Visual QA CLI command | Optional |
| `$SCREENSHOT_CLI` | Screenshot capture CLI command | Optional |
| `$VISUAL_QA_TARGET_CMD` | Optional project helper to select a visual-QA target and companion validation commands | Optional |
| `$VISUAL_QA_FIXTURE` | Representative layout fixture path for map-capable targets | Optional |
| `$VISUAL_QA_SMOKE_CMD` | Runtime smoke-test command for screenshot/OCR-only targets | Optional |
| `$VISUAL_QA_SWEEP_CMD` | Representative capture sweep command for screenshot/OCR-only targets | Optional |
| `$VISUAL_QA_BATTERY_CMD` | Broad visual regression battery command | Optional |

### Agent types

- **Dev agents**: `[AGENT_TYPE]` — specialist agents receiving implementation delegations
- **Review agents**: `security-review`, `test-review`, `doc-review`, `error-review`, `structure-review`
- **QA agents**: `safety`, `perf-qa`, `arch-review`

### Commit format

`[PREFIX]([ISSUE_ID]): [DESCRIPTION]` — configurable per project conventions.

## License

MIT
