# Issue Lifecycle

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when implementing
> issues, processing review fixes, and performing PR/QA reviews.
> Humans may also find it useful, but guidance here is optimized for
> automation and consistency by AI-assisted workflows.

## Abstract

Agent workflows for issue implementation, review fix delegation, pre-submission PR review, and QA review. Designed for specialist agents receiving delegations from an orchestrator.

---

## Table of Contents

1. [Dependencies](#dependencies)
2. [Workflow: Dev Implement](#workflow-dev-implement) (§ 1-11)
3. [Workflow: Dev Fix](#workflow-dev-fix) (§ 1-6)
4. [Workflow: PR Review](#workflow-pr-review) (§ 1 + Constraints)
5. [Workflow: QA Review](#workflow-qa-review) (§ 1-3 + Constraints)

---

## Dependencies

Workflows reference these companion skills and tools. Install and configure per your project:

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments, labels | `$ISSUE_CLI` |
| Orchestration skill | Review-finding schema, recommendation-bias patterns | Referenced by name |
| Benchmarking skill (optional) | Baseline capture, regression classification, recording | `$BENCH_CLI`, `$BENCH_PARSER` |
| Visual QA skill (optional) | Screenshot capture, interactive testing | `$VISUAL_QA_CLI`, `$SCREENSHOT_CLI` |

Project-level configuration:

| Variable | Purpose |
|----------|---------|
| `$VALIDATE_CMD` | Build + test + lint command |
| `$DECISIONS_CMD` | Decision document lookup (optional) |
| `$DIFF_SUMMARY_CMD` | Diff summary with domain grouping (optional) |

---

## Workflow: Dev Implement

**File**: `workflows/dev-implement.md`
**Agent type**: Dev agents receiving `Issue: [ISSUE_ID]` delegations
**Dependencies**: `$ISSUE_CLI`, `$VALIDATE_CMD`, `$DECISIONS_CMD` (optional), `$VISUAL_QA_CLI` (optional), `$SCREENSHOT_CLI` (optional), `$BENCH_CLI` (optional), orchestration skill

**The workflow for all dev/QA agents receiving `Issue: [ISSUE_ID]` delegations.**

Skip issue tracker updates for ad-hoc requests (no issue reference).

### Delegation Types

| Type | Detection | Flow |
|------|-----------|------|
| Single | `Issue: [ISSUE_ID]` | § 1 → § 2 → § 4 → § 5 → § 6 → § 7 → § 8 → § 9 → § 10 → return |
| Bundled | `Parent: [ISSUE_ID]` + `Sub-Issues (tree): [...]` | § 1 → § 2 → [§ 4-10]×N → § 11 → return |

**If bundled**: Execute § 4-10 per **pending** sub-issue (one task each), then § 11 aggregates and returns.

**Nested sub-issues**: Sub-issues may have children (3-level hierarchy: parent → sub → nested). Blocking relations shown when present:
```
↳ [SUB_ISSUE_1]: [TITLE] | blocks: [SUB_ISSUE_2]
↳ [SUB_ISSUE_2]: [TITLE] | blocked by: [SUB_ISSUE_1]
   ↳ [SUB_ISSUE_3]: [TITLE]  ← child of [SUB_ISSUE_2]
   ↳ [SUB_ISSUE_4]: [TITLE]  ← child of [SUB_ISSUE_2]
```
Respect blocking order: complete blockers before blocked issues.

**Completed sub-issues**: Sub-issues marked `(completed)` in delegation are for context only — skip them in § 4 loop. They represent prior work in this PR.

### § 1. Environment Setup

- Bash: `git -C [WORKTREE_PATH] ...`
- Read/Write/Edit/Grep/Glob: `[WORKTREE_PATH]/...`

```bash
git -C [WORKTREE_PATH] fetch origin main
```

### § 2. Activate Issue

#### 2.1 Claim & Get Context

```bash
# Activate issue (or parent if bundled), replace [AGENT_TYPE] with your agent type
$ISSUE_CLI issues activate [ISSUE_ID] --agent [AGENT_TYPE]
$ISSUE_CLI cache issues get [ISSUE_ID]
$ISSUE_CLI cache comments list [ISSUE_ID]
```

**If bundled**: Activate parent only. Sub-issues activated individually during § 4 loop.

**If bundled with completed siblings**: Also read comments from completed sibling sub-issues listed in delegation to pick up handoff notes:
```bash
$ISSUE_CLI cache comments list [COMPLETED_SIBLING_ID]
```

#### 2.2 Check for Research Context

```bash
$ISSUE_CLI cache issues get [ISSUE_ID] | jq -r '.description'  # Look for Research/Decision/Context fields
```

**If bundled**: Also check each sub-issue for research refs. Aggregate unique paths.

**If sub-issue**: Also check the parent issue's description for research/decision references. Sub-issues inherit their parent's research context.

**If research/decision/context references found** (in issue or parent): Read the cited files — these are mandatory context, not optional. Follow § 2.2.1, then continue.

##### 2.2.1 Research-Informed Implementation

You have domain context the orchestrator lacks. You decide how research applies.

1. **Read and evaluate**: Read project research documents (e.g., `[ISSUE_ID]/findings.md`). Consider how it applies to existing patterns, which skills need updates, whether a new skill is needed.

2. **Check for existing decision**: `$DECISIONS_CMD search --issue [RESEARCH_ISSUE_ID]`. If a prior research-complete already recorded a decision, reference it — don't duplicate. Only create new decisions if evaluation reveals *additional* decisions.

3. **Update skills BEFORE implementing**: Add decision references, update patterns.

4. **Update architecture docs** if research changes documented patterns.

#### 2.3 Evaluate Feasibility

Before planning, check your domain's code (per your agent's Domain Setup):

- **Prior decisions?** `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` to find governing decision entries
- **Description contradicts a decision?** Read the full decision file, not just the index summary. Report back to orchestrator with decision reference — do not implement approaches a decision explicitly rejects
- **Can you proceed?** Do required APIs/types exist?
- **Cross-domain dependency?** Need work in another domain first?
- **Blocked by existing issue?**
- **Optimization work without `baseline` label?** If improving existing behavior, add label now (before any code changes)

**If blocked** → **Jump to § 3**, then STOP.

**If clear** → continue to § 2.4.

#### 2.4 Plan Approach

- Update estimate if scope differs: `$ISSUE_CLI issues update [ISSUE_ID] --estimate N`
  - Estimates: 1=hours, 2=half-day, 3=day, 4=2-3 days, 5=week+
- Tasks pre-created by orchestrator. Do not create duplicates.
- **If bundled**: Plan sub-issue order based on dependencies/overlap.

#### 2.5 Domain-Specific Setup

Follow your agent definition for architecture docs, code paths, skills to load.

#### 2.6 Capture Baseline (if `baseline` label)

**Check labels** from § 2.1. If `baseline` label present:

1. **Identify affected domain** — determine which component (backend, frontend, etc.) is affected
2. **Follow** the benchmarking skill's baseline workflow Phase 1 (backend or UI path)

The perf-qa agent uses the baseline file during QA review.

### § 3. Block Issue (if dependency discovered)

**Skip if** not blocked — § 2.3 routed to § 2.4 (normal flow skips § 3).

#### 3.1 Blocked by Existing Issue

```bash
$ISSUE_CLI issues block [ISSUE_ID] --by [BLOCKER_ID] --reason "Cannot proceed until [REASON]"
```

#### 3.2 Cross-Domain Dependency Discovery

When you discover work in another domain must happen first (prerequisite issue doesn't exist):

1. **Add blocked label**:
   ```bash
   $ISSUE_CLI issues update [ISSUE_ID] --labels "agent:[AGENT_TYPE],[COMPONENT],blocked"
   ```

2. **Post structured comment**:
   ```bash
   $ISSUE_CLI comments create [ISSUE_ID] --body "BLOCKED: Cross-domain prerequisite needed.

   **Required Domain**: [DOMAIN]
   **Suggested Labels**: agent:[DOMAIN], [COMPONENT]
   **Prerequisite Issue**: [One-line description]

   **Why Blocking**:
   [What this issue needs, why it can't proceed, what prerequisite must provide]

   **Suggested Scope**:
   - [Deliverable 1]
   - [Deliverable 2]

   Requesting orchestrator create prerequisite issue."
   ```

3. **Report to orchestrator**: Your final message must state:
   - Issue is blocked pending cross-domain work
   - Domain and labels for new issue
   - Issue description ready for creation

**Orchestrator**: Creates prerequisite, sets blocking relation, delegates.

#### 3.3 Unblocked

When blocker resolves:
```bash
$ISSUE_CLI issues unblock [ISSUE_ID]
```

### § 4. Implement Solution

**If bundled**: Each sub-issue is a separate task (§ 4-10). Work only the sub-issue named in your current task.

#### 4.1 Verify Branch

`git branch --show-current` — should be `[BRANCH_NAME]` (auto-links PR to issue tracker).

**If bundled**: Branch is parent's.

#### 4.2 Implement

**If bundled**: Before implementing this sub-issue:
```bash
$ISSUE_CLI issues activate [SUB_ISSUE_ID] --agent [AGENT_TYPE]
```

Implement per your agent's domain expertise. Run quality gates before completion.

**Scope growing?** Create sub-issues: `$ISSUE_CLI issues create --title "..." --parent [PARENT_ID]`

**Found work outside scope?** Note in completion summary under "Discovered Work".

**Need deeper research?** Add "needs-research" label. Pause. Report to orchestrator.

#### 4.3 Update Documentation

Update relevant docs if implementation changes documented APIs or architecture.

**If significant path choices made** during implementation:

1. Add row to the project decision index
2. Create a decision document with full content
3. Use `// REVISIT(DXXX):` in code where applicable
4. Include decision ID in § 9 completion comment

**Skip decision recording if** no alternatives were considered or trade-offs made.

**If bundled**: Complete § 5-10 (validate, commit, post summary, finalize) for this sub-issue before marking task done.

### § 5. Validate

```bash
# Choose ONE based on change scope:
$VALIDATE_CMD --quick              # Fast: lint, unit tests (comment/minor changes)
$VALIDATE_CMD --fail-fast          # Full but stops at first failure (recommended for first run)
$VALIDATE_CMD                      # Full: build, all tests, docs, benchmarks (significant changes)
# After fixing failures:
$VALIDATE_CMD --recheck            # Only re-runs previously failed checks (skip cached passes)
```

**On failure:**
- **First run**: Use `--fail-fast` to stop early, fix, then `--recheck`
- **Simple + related to your work** → fix it, `--recheck`
- **Complex or unrelated** → still commit your work, note failure in commit message, report in return
- **Stuck** (same failure 3+ times) → stop looping, commit, report details

Always report unresolved validation failures to orchestrator.

#### 5.1 Visual QA

**Skip if** the issue does not have the `design` label.

Run a targeted visual check using the visual QA skill:
- **Rendering change**: `$SCREENSHOT_CLI --no-build` → Read the PNG to verify
- **Interaction / layout change**:
  1. `$VISUAL_QA_CLI doctor`
  2. Start a visual QA session, preferably with the relevant fixture:
     `$VISUAL_QA_CLI start --build --layout [PROJECT_TEST_FIXTURE]`
  3. `$VISUAL_QA_CLI map`
  4. Use map-first high-level commands to test the affected behavior
  5. Use `locate` only for literal text targets or OCR sanity checks
  6. Capture a screenshot or short recording if it adds evidence
- **Broad interaction change**: Run the project-specific visual test battery

Focus on what your changes affect — not the full checklist. Do NOT capture golden baselines — that happens at submit-pr time.

### § 6. Reflect & Update Skills/Rules

**Skip if** implementation was straightforward with no repeated issues and no notable discoveries.

**Trigger**: Any of these during § 4-5:
- Fixed same problem 2+ times (lint, pattern, API usage, test approach)
- Discovered non-obvious gotcha worth remembering
- Spent multiple cycles on something a rule/skill could prevent
- Documentation in skill, rules, patterns, need changed based on discovered optimal approaches.

**Action**: Update the source directly.

- **Repeated mistake** → Add rule to project rules or agent definition
- **Reusable pattern** → Add to relevant skill
- **Missing context** → Update architecture doc or reference table
- **Wrong guidance** → Fix incorrect rule, skill, or pattern that caused the issue

Criteria: Would this save 5+ minutes in a future session? If yes, update. One surgical addition per lesson. No verbose examples.

**If you can't update directly** (wrong domain, needs discussion): note in § 9 Discovered Work with type `[process]`.

### § 7. Commit Changes

```bash
git -C [WORKTREE_PATH] add -A
git -C [WORKTREE_PATH] commit -m "[PREFIX]([ISSUE_ID]): [DESCRIPTION]"
```

**If bundled**: Use CURRENT sub-issue ID, not parent ID.

**Worktree caveat**: Never stage lock files listed in the project-specific gitignore. Stage specific files by name.

**If unresolved validation failures**: Append `[validate: FAILING_CHECK]` to commit message.

**Verify commit exists** before proceeding:
```bash
git -C [WORKTREE_PATH] log -1 --oneline
```

### § 8. Apply QA Labels

Based on FINAL validated code:

| Trigger | Label |
|---------|-------|
| Unsafe code, atomics, lock-free | `needs-safety-audit` |
| Hot path, latency-sensitive, or shared/main-build perf risk | `needs-perf-test` |
| New module, public API | `needs-review` |

Full triggers: see the project label application guide.
Development-only feature exception: do not apply `needs-perf-test` for work isolated behind a development-only feature gate. Run the feature-gated checks locally and only add the label if shared or feature-off paths are affected.

### § 9. Post Completion Summary

**Always required** — documents the FINAL state after all validation passes.

**Target issue**: Post to the issue you just implemented (for bundled work: current sub-issue, not parent).

```bash
$ISSUE_CLI comments create [ISSUE_ID] --body "## Completion Summary

**Agent**: [AGENT_NAME]
**Branch**: \`[BRANCH]\`

### Files Created/Modified
- \`path/to/file\` - Description

### Key Decisions
1. Decision and rationale
2. DXXX recorded (if research-informed)

### Skills/Docs/Rules Updated
- \`skill-name\`: Updated X
(Skip if none)

### Domain Metrics
[Your agent-specific metrics: frame time, latency, etc.]
(Skip if not applicable)

### Discovered Work
- [Type]: Description (estimate: N)
Future work beyond current scope. NOT for the next agent — for backlog/orchestrator.
(Skip if none)

### Handoff Notes
Context the next agent needs to complete its current-scope work (e.g., struct changes, API contracts, file locations). Do NOT put aspirational suggestions or future work here — those belong in Discovered Work.
(Skip if none)"
```

### § 10. Finalize Issue

**Verify complete:**

| Step | When | Ref |
|------|------|-----|
| Baseline captured | `baseline` label | § 2.6 |
| Research applied | Research in description | § 2.2.1 |
| Validation run | Always | § 5 |
| Skills/rules updated | Repeated issues in § 4-5 | § 6 |
| Changes committed | Always | § 7 |
| QA labels applied | Triggers present | § 8 |
| Summary posted | Always | § 9 |

**If single**: Return now with:
```
Branch: [BRANCH_NAME]
Commit: [SHA]
QA Labels: [labels or "none"]
Validate: [pass or "FAILING: check1, check2"]
Summary: [ISSUE_ID] ✓
```

**If bundled**: Mark task completed. Next sub-issue is a separate task, or proceed to § 11 if none remain.

**Sub-issue of a parent** → mark issue Done (`$ISSUE_CLI issues update [ISSUE_ID] --state "Done"`).
**Parent or standalone issue** → do NOT mark Done (handled by PR merge workflow and issue tracker sync).

Do NOT push or submit PR — orchestrator handles after review passes.

### § 11. Return to Orchestrator (If Bundled)

**Skip if** single issue — you returned at § 10.

1. **Update parent issue with aggregated QA labels** (issue tracker API):
   ```bash
   # Collect QA labels from all sub-issues (including nested), apply to parent
   $ISSUE_CLI issues update [PARENT_ID] --labels "[EXISTING_LABELS],[AGGREGATED_QA_LABELS]"
   ```

2. **Post parent summary** (tree format for sub-issues, blocking info shown):
   ```bash
   $ISSUE_CLI comments create [PARENT_ID] --body "## Bundle Complete
   **Agent**: [NAME] | **Branch**: [BRANCH]

   Sub-issues (tree):
   ↳ [SUB_ISSUE_1] ✓ | blocks: [SUB_ISSUE_2]
   ↳ [SUB_ISSUE_2] ✓ | blocked by: [SUB_ISSUE_1]
      ↳ [SUB_ISSUE_3] ✓  ← nested
   Files: N | Commits: N | QA: [LABELS]
   [Discovered work: ...]"
   ```

3. **Return exactly**:

   <output_format>
   Parent: [ISSUE_ID]
   Sub-Issues: [tree format with ✓]
   Branch: [BRANCH]
   Commits: [COUNT] ([SHAS])
   QA Labels: [AGGREGATED]
   Summaries: [all issue IDs ✓]
   </output_format>

---

## Workflow: Dev Fix

**File**: `workflows/dev-fix.md`
**Agent type**: Dev agents receiving review fix delegations
**Dependencies**: `$ISSUE_CLI`, `$VALIDATE_CMD`, `$DECISIONS_CMD` (optional), `$VISUAL_QA_CLI` (optional), `$SCREENSHOT_CLI` (optional)

**The workflow for dev agents receiving review fix delegations.**

### § 1. Environment Setup

### § 2. Read Issue Context

```bash
$ISSUE_CLI cache issues get [ISSUE_ID]
$ISSUE_CLI cache comments list [ISSUE_ID]
```

Understand prior work, decisions, and handoff notes before evaluating items.

### § 3. Process Review Items

For each item in `Review items:`:

1. **Evaluate independently** — each item stands alone

2. **Apply if**: related to parent issue, no new risks

3. **Skip if** pattern conflicts with existing architecture, would break other functionality, does not follow your defined rules or conventions.
   - **Before applying**: `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` for decisions governing the affected area → if match found, read the full decision file
   - If review item contradicts an active decision, skip with decision reference (e.g., "Skipped — contradicts D010")
   - Expanding scope is OK if it relates to the parent issue/PR

4. **Update docs/skills/patterns** if fix changes documented behavior

5. **For UI lifecycle/cache fixes**: If you introduce cached/mirrored UI state or change window/event handling, trace all invalidation and event-entry paths before returning. Prefer extending existing listeners over adding parallel subscriptions for the same event family, and add regression coverage for the non-obvious paths you touched.

6. **Note in return** if fix reveals deeper issues or if you skipped items — cite decision ID or rule

7. **Report as Blocked** if stuck on same fix 3+ times

Related improvements OK — unrelated changes should become separate issues.

### § 4. Validate

```bash
# Choose based on change scope:
$VALIDATE_CMD --quick              # Fast: lint, unit tests (comment/minor changes)
$VALIDATE_CMD --fail-fast          # Full but stops at first failure (recommended for first run)
$VALIDATE_CMD                      # Full: build, all tests, docs, benchmarks (significant changes)
# After fixing failures:
$VALIDATE_CMD --recheck            # Only re-runs previously failed checks (skip cached passes)
```

**On failure:**
- **First run**: Use `--fail-fast` to stop early, fix, then `--recheck`
- **Simple + related to your work** → fix it, `--recheck`
- **Complex or unrelated** → still commit your work, note failure in commit message, report in return

#### 4.1 Visual QA

**Skip if** the issue does not have the `design` label, or the fix does not touch UI code.

Run a targeted visual check using the visual QA skill:
- **Rendering fix**: `$SCREENSHOT_CLI --no-build` → Read the PNG to verify
- **Interaction / layout fix**:
  1. `$VISUAL_QA_CLI doctor`
  2. Start a visual QA session, preferably with the relevant fixture:
     `$VISUAL_QA_CLI start --layout [PROJECT_TEST_FIXTURE]`
  3. `$VISUAL_QA_CLI map`
  4. Use map-first high-level commands to test the affected behavior
  5. Use `locate` only for literal text targets or OCR sanity checks
  6. Capture a screenshot or short recording if it adds evidence
- **Broader regression risk**: Run the project-specific visual test battery

Focus on what the fix changes — not the full checklist.

#### 4.2 Commit

```bash
git add -A && git commit -m "[PREFIX]([ISSUE_ID]): [MESSAGE]"
```

| Source | Commit Message |
|--------|----------------|
| `pr-review` | "Address PR review - [brief description]" |
| `qa-review` | "Address QA review - [brief description]" |
| `suggestions` | "Address review suggestions" |

If validation failures exist, append: `[validate: FAILING_CHECK]`

### § 5. Reflect & Update Skills/Rules

**Skip if** all fixes were one-off issues unlikely to recur (e.g., typo, missing import).

**Trigger**: Any of these during § 3-4:
- Fixed same problem 2+ times (lint, pattern, API usage, test approach)
- Discovered non-obvious gotcha worth remembering
- Spent multiple cycles on something a rule/skill could prevent
- Documentation in skill, rules, patterns, need changed based on discovered optimal approaches

**Action**: Update the source directly.

- **Repeated mistake** → Add rule to project rules or agent definition
- **Reusable pattern** → Add to relevant skill
- **Missing context** → Update architecture doc or reference table
- **Wrong guidance** → Fix incorrect rule, skill, or pattern that caused the issue

Criteria: Would this save 5+ minutes in a future session? If yes, update. One surgical addition per lesson. No verbose examples.

**If you can't update directly** (wrong domain, needs discussion): note in § 6 return with type `[process]`.

### § 6. Return

**Return exactly**:

<output_format>
| # | Decision | Reasoning |
|---|----------|-----------|
| N | Applied/Skipped/Blocked | [EXPLANATION — cite DXXX or rule if Skipped] |

Commits: [SHAS or "none"]
Validate: [pass or "FAILING: check1, check2"]
</output_format>

Report decision and reasoning for each item. Include commit SHAs and validation status.

**Do NOT** push — orchestrator handles after review.

---

## Workflow: PR Review

**File**: `workflows/pr-review.md`
**Agent type**: Review agents (security-review, test-review, doc-review, error-review, structure-review)
**Dependencies**: orchestration skill (recommendation-bias patterns, review-finding schema)

**The workflow for PR review agents.** PR review agents are pre-submission reviewers. They run in parallel, each reviewing the same diff from their specialist perspective.

**Ownership**: You review ONE PR. Return verdict to orchestrator. No issue tracker state changes.

### § 1. Review Changes

Extract from delegation message:
- `Worktree` path
- `Branch` name
- `Decisions` to respect
- Re-review context (if any)

#### 1.1 Diff

```bash
git -C [WORKTREE_PATH] diff main...HEAD
```

Review for noteworthy findings only — skip minor style issues. Exclude research documents.
If a changed path was deleted, inspect it from the git diff or git history; do not try to `Read` the deleted working-tree path directly.

#### 1.2 Read Decisions

Read decision files listed in delegation. Do NOT suggest changes that contradict them.

#### 1.3 Classify Findings

Read the orchestration skill's recommendation-bias patterns. Apply its decision flow to ALL findings — a finding must pass actionability and relatedness checks before entering `blockers[]` or `suggestions[]`. Then use size to categorize suggestions as `fix` or `issue`.

#### 1.4 Handle Re-Review (if applicable)

**Skip if** not a re-review cycle (no "re-review" section in delegation).

Items listed as fixed or escalated are already resolved — do NOT re-report them. Only report NEW issues or regressions introduced by the fixes.

#### 1.5 Return JSON Report

Build JSON per the orchestration skill's review-finding schema. Save to `[WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json`.

**Verdict rules:**
- `action_required`: 1+ items in `blockers[]`
- `pass`: `blockers[]` empty

#### 1.6 Return

**Return exactly** (return to orchestrator):

<output_format>
Verdict: [pass|action_required]
File: [WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json
```json
{complete JSON object}
```
</output_format>

### Constraints

**Do NOT**:
- Modify issue tracker state (labels, status)
- Create commits or push changes
- Call other subagents

**Orchestrator handles**: All issue tracker updates, routing blockers back to dev agent, merging JSONs, presentation.

---

## Workflow: QA Review

**File**: `workflows/qa-review.md`
**Agent type**: QA agents (safety, perf-qa, arch-review) invoked via `needs-*` labels
**Dependencies**: `$ISSUE_CLI`, `$DECISIONS_CMD` (optional), `$DIFF_SUMMARY_CMD` (optional), `$BENCH_CLI` (optional), `$BENCH_PARSER` (optional), orchestration skill (review-finding schema)

**The workflow for QA agents.** QA agents are review-only. They are never assigned as issue owners.

**Ownership**: You review ONE PR. Return verdict to orchestrator. No issue tracker state changes.

### § 1. Set Up

#### 1.1 Read Context

```bash
$ISSUE_CLI cache issues get [ISSUE_ID]
$ISSUE_CLI cache comments list [ISSUE_ID]
```

Extract from delegation prompt:
- Dev agent's completion summary
- Which `needs-*` label triggered this review

### § 2. Execute Review

#### 2.1 Read Decision/Research Context

Before reviewing, run `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"` for decisions governing the changed areas. If matches found, read the full decision files — index summaries are insufficient for understanding scope and rejected alternatives. If the delegation prompt includes additional decision context, read those too.

**Suggestions that contradict active decisions are invalid** unless the decision itself is flawed (flag as blocker with justification, citing the specific decision and why it's wrong).

#### 2.2 Identify Changed Files

```bash
$DIFF_SUMMARY_CMD -C [WORKTREE_PATH]
```

Use domain grouping and risk flags to focus review on changed files relevant to your domain.
**Exclude**: Research documents — historical research artifacts, not reviewable code or docs.

#### 2.3 Run Agent Review

Run your agent-specific review. See your agent file for exact commands and Output section for blocker/suggestion mapping.

#### 2.4 Classify Regressions (perf-qa only)

**Skip if** not `perf-qa` or no regressions detected (exit code 0).

When `$BENCH_CLI regression` exits with code 1, classify every regressed operation per the benchmarking skill's regression classification rules. Populate `blockers[]` and `qa_metadata.perf_qa.regressions[]` per your agent's Output section.

#### 2.5 Record Benchmark Results (perf-qa only)

**Skip if** not `perf-qa`.

- **Backend changes**: Pipe benchmark output through `$BENCH_PARSER` for automatic recording
- **Frontend/UI changes**: Run a project-specific perf capture tool and pipe results to `$BENCH_CLI record`
- **Manual entry**: `$BENCH_CLI record <component> '<json>'`

See the benchmarking skill for full recording details.

**Note**: Benchmark results may be symlinked to the main repo in worktrees. Results are written directly to main's directory — no commit needed. Record the latest commit SHA from your worktree branch as the benchmark commit in your return output (§ 3).

#### 2.6 Return JSON Report

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

### § 3. Complete

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

### Constraints

**Do NOT**:
- Claim the issue (`$ISSUE_CLI issues activate`)
- Modify issue tracker state (labels, status)
- Mark issue done
- Create commits for code changes or push changes
- Call other subagents

**Note**: Benchmark results may be symlinked — writes go directly to main repo, no commit needed (§ 2.5).

**Orchestrator handles**: All issue tracker updates, routing blockers back to dev agent, merging JSONs, presentation.
