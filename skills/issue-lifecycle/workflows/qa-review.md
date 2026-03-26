# QA Review Lifecycle

> **Dependencies**: `$ISSUE_CLI`, `$DECISIONS_CMD` (optional), `$DIFF_SUMMARY_CMD` (optional), `$BENCH_CLI` (optional), `$BENCH_PARSER` (optional), orchestration skill (review-finding schema)

**The workflow for QA agents (safety, perf-qa, arch-review) invoked via `needs-*` labels.**

QA agents are review-only. They are never assigned as issue owners.

**Ownership**: You review ONE PR. Return verdict to orchestrator. No issue tracker state changes.

---

## 1. Set Up

### 1.1 Read Context

```bash
$ISSUE_CLI cache issues get [ISSUE_ID]
$ISSUE_CLI cache comments list [ISSUE_ID]
```

Extract from delegation prompt:
- Dev agent's completion summary
- Which `needs-*` label triggered this review

---

## 2. Execute Review

### 2.1 Read Decision/Research Context

Before reviewing, use the decider skill's search workflow: `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` for decisions governing the changed areas. If matches found, read the full decision files — index summaries are insufficient for understanding scope and rejected alternatives. If the delegation prompt includes additional decision context, read those too.

**Suggestions that contradict active decisions are invalid** unless the decision itself is flawed (flag as blocker with justification, citing the specific decision and why it's wrong).

### 2.2 Identify Changed Files

```bash
$DIFF_SUMMARY_CMD -C [WORKTREE_PATH]
```

Use domain grouping and risk flags to focus review on changed files relevant to your domain.
**Exclude**: Research documents — historical research artifacts, not reviewable code or docs.

### 2.3 Run Agent Review

Run your agent-specific review. See your agent file for exact commands and Output section for blocker/suggestion mapping.

### 2.4 Classify Regressions (perf-qa only)

**Skip if** not `perf-qa` or no regressions detected (exit code 0).

When `$BENCH_CLI regression` exits with code 1, classify every regressed operation per the benchmarking skill's regression classification rules. Populate `blockers[]` and `qa_metadata.perf_qa.regressions[]` per your agent's Output section.

### 2.5 Record Benchmark Results (perf-qa only)

**Skip if** not `perf-qa`.

- **Backend changes**: Pipe benchmark output through `$BENCH_PARSER` for automatic recording
- **Frontend/UI changes**: Run a project-specific perf capture tool and pipe results to `$BENCH_CLI record`
- **Manual entry**: `$BENCH_CLI record <component> '<json>'`

See the benchmarking skill for full recording details.

**Note**: Benchmark results may be symlinked to the main repo in worktrees. Results are written directly to main's directory — no commit needed. Record the latest commit SHA from your worktree branch as the benchmark commit in your return output (§ 3).

### 2.6 Return JSON Report

1. **Build JSON** per the orchestration skill's review-finding schema, filename `[WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json`.
   - Standard fields: `agent`, `timestamp`, `verdict`, `summary`, `blockers[]`, `suggestions[]`
   - If `perf-qa`: include `benchmark_commit` from § 2.5
   - `qa_metadata.[agent_type]` populated per your agent (project-configurable):

   | Agent | qa_metadata key | Required fields |
   |-------|-----------------|-----------------|
   | safety | `safety` | `tool_results`, `unsafe_block_count`, `violations[]` |
   | perf-qa | `perf_qa` | `percentiles`, `regression_pct`, `regressions[]`, `platform`, `baseline_sha` |
   | arch-review | `arch_review` | `dimension_scores`, `overall_score`, `pass` |

   **Verdict rules:**
   - `action_required`: 1+ items in `blockers[]`
   - `pass`: `blockers[]` empty

2. **Return the JSON** in your response (the calling agent writes the file):

---

## 3. Complete

**Return exactly**:

<output_format>
QA_COMPLETE
verdict: [pass|action_required]
agent: [AGENT_NAME]
benchmark_commit: [SHA or "none"]
File: tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json
```json
{complete JSON object}
```
</output_format>

---

## Constraints

**Do NOT**:
- Claim the issue (`$ISSUE_CLI issues activate`)
- Modify issue tracker state (labels, status)
- Mark issue done
- Create commits for code changes or push changes
- Call other subagents

**Note**: Benchmark results may be symlinked — writes go directly to main repo, no commit needed (§ 2.5).

**Orchestrator handles**: All issue tracker updates, routing blockers back to dev agent, merging JSONs, presentation.
