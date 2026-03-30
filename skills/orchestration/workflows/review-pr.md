# PR Review Workflow

> **Dependencies**: `$GIT_HOST_CLI`, `$WORKTREE_CLI`, `$ISSUE_CLI` (optional), `$VALIDATE_CMD`, `$DECISIONS_CMD` (optional), `$DIFF_SUMMARY_CMD`, `scripts/workflow-state`, `scripts/workflow-sections`

Pre-submission code review with fix handling, QA checks, and issue audit.

## Inputs

| Command | Behavior |
|---------|----------|
| `/review-pr` | Full review cycle: review, fix, QA, summary |
| `/review-pr [PR#]` | Get/create worktree for PR, full review cycle |
| (from start-worktree) | Managed lifecycle with caller context |

**Caller context parameters** (via `⤵`):
- `worktree`: worktree path
- `agents` (optional): list of review agent names. Default: all 5.
- `lifecycle` (optional): `"managed"` (return to caller at § 11) | `"self"` (default, standalone).
- `dev_agent` (optional): name of alive dev agent for fix delegation. If absent, fixes use sub-agent tasks.
- `issue_id` (optional): issue tracker ID. If absent, extracted from branch.

**If PR# provided:**
```bash
ISSUE=$($GIT_HOST_CLI pr-issue [PR_NUMBER] --format=text)
WT_PATH=$($WORKTREE_CLI path $ISSUE 2>/dev/null || $WORKTREE_CLI create $ISSUE --pr [PR_NUMBER])
```

**If no argument:** Set `WT_PATH` to current directory.

**Standalone init** (`lifecycle: "self"` only):
```bash
# Extract issue from branch if not provided
ISSUE_ID=$(git rev-parse --abbrev-ref HEAD | grep -oiP "$ISSUE_PATTERN")
# Init workflow state if not exists
if ! scripts/workflow-state exists $ISSUE_ID; then
  scripts/workflow-state init $ISSUE_ID --worktree "$WT_PATH" --branch "$(git -C $WT_PATH rev-parse --abbrev-ref HEAD)"
  QA_LABELS=$($ISSUE_CLI cache issues get $ISSUE_ID | jq '[.labels[] | select(startswith("needs-"))]')
  scripts/workflow-state set $ISSUE_ID qa_labels "$QA_LABELS"
fi
```

---

## 1. Identify Changes

```bash
BASE_BRANCH=${WORKTREE_DEFAULT_BRANCH:-$(git -C [WORKTREE_PATH] symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')}
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=main
git -C [WORKTREE_PATH] diff "origin/$BASE_BRANCH"...HEAD --stat
```

**If no changes**: Report "No changes to review" and **END**.

### 1.1 Gather Decision Context

Extract issue ID from branch name (e.g., `[BRANCH_NAME]` → `[ISSUE_ID]`). Use the decider skill's search workflow:

```bash
$DECISIONS_CMD search --issue [ISSUE_ID]
```

Collect decision IDs and summaries from the JSON output.

**If decisions found**: Include in delegation prompt below. Agents MUST read cited decisions/research before suggesting changes that could contradict them.

### 1.2 Check for Re-Review Context

```bash
CYCLES=$(scripts/workflow-state get [ISSUE_ID] '.cycles // 0')
FIXED=$(scripts/workflow-state get [ISSUE_ID] '.fixed_items // []')
ESCALATED=$(scripts/workflow-state get [ISSUE_ID] '.escalated_items // []')
```

If `CYCLES > 0`: This is a re-review. Include the "Previous review cycle context" section in the delegation prompt below, populated from `FIXED` and `ESCALATED`.

## 2. Launch Review Agents

**Detect team context**:
```bash
TEAM=$(scripts/workflow-state get [ISSUE_ID] '.team_name // empty')
```

**Determine agent list**: If `agents` context provided, use only those. Otherwise default to all 5: security-review, test-review, doc-review, error-review, structure-review (configurable per project).

**If in team session** (`$TEAM` set):

1. **Create agent tasks**:
   ```bash
   # For each agent in [AGENTS]:
   scripts/workflow-sections [ISSUE_LIFECYCLE_WORKFLOW]/pr-review.md --agent "[AGENT_NAME]" --emoji "🐞"
   ```
   Create task for each.

2. **Spawn review agents**:
   - **If re-review** (`CYCLES > 0` AND `review_agents` in state): DO NOT spawn. Agents already alive. Proceed to step 3.
   - **Otherwise** (first cycle, regardless of lifecycle): Spawn each agent:
     ```
     # For each agent in [AGENTS]:
     Spawn agent: type=[AGENT], name=[AGENT], team=[TEAM], prompt=SPAWN_PROMPT
     ```
     Copy spawn prompt **verbatim** from spawn prompt templates (project-level) (fill `[PLACEHOLDERS]` only). Agents go idle waiting for delegation.
   - Store in state:
     ```bash
     scripts/workflow-state set [ISSUE_ID] review_agents '[AGENT_LIST_JSON]'
     ```

3. **Delegate via messages** (one per agent, all in parallel):
   ```
   # For each agent in [AGENTS]:
   Send message to [AGENT]: content=DELEGATION, summary="Review changes"
   ```

**If standalone** (no team):

- **If re-review** (`CYCLES > 0`) AND `review_agent_ids` in state:
  Resume existing agents — they retain full prior review context:
  ```
  # For each agent in [AGENTS] with stored ID:
  Resume agent: id=[AGENT_ID], prompt=DELEGATION
  ```
- **Otherwise**: Launch new sub-agents:
  ```
  Launch sub-agent: type=[AGENT_TYPE], prompt=DELEGATION
  ```

**Delegation prompt:** Follow exactly, fill placeholders, add nothing else. Omit lines/sections with empty placeholders.

<delegation_format>
Task prefix: [TASK_PREFIX]

Workflow: issue-lifecycle skill pr-review workflow

Worktree: [WORKTREE_PATH]
Branch: [BRANCH]

Decisions:
[For each matching decision: "- [DECISION_ID]: [ONE_LINE_SUMMARY] — [DECISION_FILE_PATH]"]
[If none: "- No linked decisions found."]
<if re-review cycle>
Re-review cycle [N]. Already resolved — do NOT re-report:
- Fixed: [For each fixed_item: "[DESCRIPTION] — fixed in [COMMIT_SHA]"]
- Escalated: [For each escalated_item: "[DESCRIPTION] — [REASON]"]
</if>
</delegation_format>

## 3. Collect & Present Results

**Agents mark their own tasks completed.** Monitor via task list.

**If in team session**: Wait for completion messages from all [AGENTS]. Do NOT shutdown — agents needed for potential re-review in § 4.

**If standalone**: Wait for all [AGENTS] returns. Store each agent's returned `agent_id` in state for re-review resume:
```bash
scripts/workflow-state set [ISSUE_ID] review_agent_ids '{"[AGENT_NAME]":"[AGENT_ID]",...}'
```

Extract `Report` path and `Verdict` from each. If any agent fails to return expected format, halt and report error.

Overall verdict: `action_required` if any agent has blockers, `pass` otherwise.

**Update state**:
```bash
# For each agent JSON path:
scripts/workflow-state append [ISSUE_ID] json_paths "[PATH]"
```

<output_format>

### ✅ PR REVIEW COMPLETE

| Agent | Verdict | Path |
|-------|---------|------|
| **Overall** | `[pass\|action_required]` | |
| [For each agent in AGENTS:] |
| [AGENT] | `[verdict]` | `[path]` |
</output_format>

**Route by verdict + items:**

Read agent JSONs, check for items where `category == "fix"`.

| verdict | fix items? | Next |
|---------|-----------|------|
| any | yes (or `action_required`) | → § 4 |
| `pass` | none | → § 5 |

## 4. Handle PR Review Items

**Collect items** from agent JSONs:
- **Blockers**: items from agents with `action_required` verdict
- **Fix suggestions**: items where `category == "fix"` from any agent

**If no items** → § 5.

**Present to user:**

<output_format>

### PR Review Items — [ISSUE_ID]

**Blockers**

| # | Agent | Location | Description | Pri |
|---|-------|----------|-------------|-----|
| 1 | [agent] | [file:line] | [description] | 🔴 |

**Fix Suggestions**

| # | Agent | Location | Description | Pri | Est |
|---|-------|----------|-------------|-----|-----|
| 1 | [agent] | [file:line] | [description] | 🟤 | 1 |

</output_format>

**Omit empty categories.**

→ Ask user (omit categories with no items):

| Category | Question | Type |
|----------|----------|------|
| Blockers | `Fix blockers?` | `Fix now` \| `Ignore and proceed` |
| Fix suggestions | `Apply fix suggestions?` | Multi-select: `#N: [TITLE]`, `All`, `None` |

If >4 suggestion items: show first 3 + `All N fixes`. Refine via "Other".

| User Choice | Action |
|-------------|--------|
| No items selected | → § 5 |
| Items selected | → fix delegation below |

**Never fix as main agent.**

### Fix Delegation

1. **Capture pre-fix state**:
   ```bash
   scripts/workflow-state set [ISSUE_ID] pre_delegate_sha "$(git -C [WORKTREE_PATH] rev-parse HEAD)"
   ```

2. **Run Skill**: `⤵ /dev-fix § 1-3 → § 4 step 3` with context:
   - `worktree`: [WORKTREE_PATH]
   - `lifecycle`: `"managed"`
   - `dev_agent`: [DEV_AGENT] (if provided)
   - `issue_id`: [ISSUE_ID]
   - `items`: [SELECTED_ITEMS — format each as `#[N] | [Agent] | [Location]` with Description + Recommendation]
   - `source`: `pr-review`

3. **Route based on fix scope**:
   ```bash
   PRE_SHA=$(scripts/workflow-state get [ISSUE_ID] .pre_delegate_sha)
   $DIFF_SUMMARY_CMD -C [WORKTREE_PATH] $PRE_SHA
   ```

   | `files_changed` | `risk_flags` | `scope` | Route |
   |-----------------|--------------|---------|-------|
   | `0` | — | — | § 5 |
   | `>0` | non-empty | any | → § 2 (full re-review, all agents) |
   | `>0` | empty | `production` | Selective shutdown (below) → § 2 |
   | `>0` | empty | `support` | § 5 |

   **Selective shutdown** (row 3):
   a. Read review JSONs. Reporting agents = agents whose JSON contained items.
   b. **Team session**: Shutdown non-reporters: Send message type="shutdown_request" to [AGENT]
   c. **Standalone**: Remove non-reporter IDs from `review_agent_ids` (reporters kept for resume).
   d. Update state: `scripts/workflow-state set [ISSUE_ID] review_agents '[REPORTERS_ONLY]'`

## 5. Verdict Pass

1. **Shutdown review agents**:
   - **Team session**: Send shutdown_request to each agent in state `review_agents`.
   - **Standalone**: Agents already returned — clear stored IDs.
   ```bash
   scripts/workflow-state set [ISSUE_ID] review_agents '[]'
   scripts/workflow-state set [ISSUE_ID] review_agent_ids '{}'
   ```

2. **Check skip_qa flag**:
   ```bash
   SKIP_QA=$(scripts/workflow-state get [ISSUE_ID] '.skip_qa // false')
   ```
   If `true`: `scripts/workflow-state set [ISSUE_ID] skip_qa false` → § 8

3. **Read state**: `scripts/workflow-state get [ISSUE_ID] .qa_labels`

4. **Route**:
   - QA labels present → § 6
   - No QA labels → § 8

## 6. QA Checks

**Skip if** no QA labels. → § 8

1. **Check labels**. See issue tracker label configuration (project-level).

2. **Determine sequence**: QA agent types are configurable per project. Example mappings: `needs-safety-audit` → safety, `needs-perf-test` → perf-qa, `needs-review` → arch-review, `design` → visual QA (use visual QA skills as necessary to validate UI changes).

**For each QA agent, execute steps 3–8:**

3. **Create agent tasks**:
   ```bash
   scripts/workflow-sections [ISSUE_LIFECYCLE_WORKFLOW]/qa-review.md --agent "qa-review" --emoji "🪲"
   ```
   Create task for each.

4. **Spawn QA agent**:

   **If in team session**:
   ```
   Spawn agent: type=[QA_AGENT], name=[QA_AGENT], team=[TEAM], prompt=SPAWN_PROMPT
   ```

   **If standalone**:
   ```
   Spawn agent: type=[QA_AGENT], prompt=SPAWN_PROMPT
   ```

   Copy spawn prompt **verbatim** from spawn prompt templates (project-level) (fill `[PLACEHOLDERS]` only).

5. **Delegate**:

   **If in team session**: Send message to [QA_AGENT]: content=DELEGATION, summary="QA review [ISSUE_ID]"

   **If standalone**: Launch as sub-agent task with DELEGATION prompt.

   <delegation_format>
   Task prefix: [TASK_PREFIX]

   Workflow: issue-lifecycle skill qa-review workflow

   Issue: [ISSUE_ID]
   Branch: [BRANCH]
   Worktree: [WORKTREE_PATH]
   Trigger: [needs-* label]

   Dev summary:
   [paste completion summary from dev return or describe branch changes]

   [If re-review (CYCLES > 0) — include:]
   Previous review cycle context (cycle [CYCLES]):
   - Fixed since last review: [For each fixed_item with source "qa-review": "[DESCRIPTION] — fixed in [COMMIT_SHA]"]
   - Escalated (accepted): [For each escalated_item with source "qa-review": "[DESCRIPTION] — [REASON]"]
   - Do NOT re-report fixed or escalated items. Only report NEW issues or regressions introduced by the fixes.
   </delegation_format>

6. **Wait for completion.**

7. **Shutdown QA agent**:

   **If in team session**: Send shutdown_request to [QA_AGENT]

   **If standalone**: Agent task already returned.

8. **Process agent return.** Agent returns `verdict`, `json_path`, and (for perf-qa) `benchmark_commit`.
   - **Update state**: `scripts/workflow-state append [ISSUE_ID] json_paths "[json_path]"`
   - If `benchmark_commit` is not "none", verify: `git -C [WORKTREE_PATH] log -1 --oneline [SHA]`.
   - **If perf-qa**: post benchmark report to issue tracker as issue comment:
     ```bash
     $ISSUE_CLI comments create [ISSUE_ID] --body "[PERF_REPORT]"
     ```
     Build PERF_REPORT from perf-qa JSON `qa_metadata.perf_qa`:
     ```markdown
     ## Benchmark Results — [BRANCH] ([benchmark_commit])

     **Platform**: [platform] | **Baseline**: [baseline_sha]

     ### Regressions
     [If regressions[] non-empty:]
     | Operation | Baseline | Current | Change | Classification | Notes |
     |-----------|----------|---------|--------|----------------|-------|
     | [op] | [baseline_ns] | [current_ns] | +[change_pct]% | [classification] | [justification/decision_ref] |

     [If regressions[] empty:]
     None detected.

     ### Budget Compliance
     | Component | Operation | P50 | P99 | Budget | Status |
     |-----------|-----------|-----|-----|--------|--------|
     [Key operations from benchmarks vs project performance budgets]

     ### Summary
     [N] benchmarks recorded | [N] regressions ([N] hot-path, [N] cold-path, [N] intentional) | All budgets [met/exceeded]
     ```
   - **Handle verdict:**

     | verdict | Action |
     |---------|--------|
     | `pass` | Continue to next QA agent |
     | `action_required` | → § 7 |

9. **After all QA agents complete** — check for accumulated fix suggestions:
   - Read all QA agent JSONs from state `json_paths`, filter items where `category == "fix"`
   - Exclude items already in `fixed_items` or `escalated_items`
   - Fix suggestions remain → § 7
   - No remaining items → § 8

## 7. Handle QA Review Items

**Skip if** all QA verdicts are `pass` AND no fix suggestions from QA agents. → § 8

**Never fix as main agent.**

Follow § 4 pattern (collect → present → ask user → delegate via `/dev-fix` → update state) with these overrides:

- **Items**: from QA agent JSONs. Exclude items already in `fixed_items` or `escalated_items`.
- **Table header**: `QA Agent` instead of `Agent`. Title: `QA Review Items — [ISSUE_ID]`.
- **Source**: `qa-review` in `/dev-fix` context.
- **`qa_agent`**: pass QA agent name (configurable, e.g. `safety|perf-qa|arch-review`) to `/dev-fix` context.
- **Route after fix**:

   | `files_changed` | `risk_flags` | `scope` | Route |
   |-----------------|--------------|---------|-------|
   | `0` | — | — | § 8 |
   | `>0` | non-empty | any | § 2 (full PR review) |
   | `>0` | empty | `production` | § 6 (focused QA re-check) |
   | `>0` | empty | `support` | § 8 |

## 8. Review Summary

**Read state**: `scripts/workflow-state get [ISSUE_ID] .json_paths`

**Skip if** json_paths empty (no reviews ran). Output: "No review items." → § 9

1. **Read all JSON files** from state `json_paths`

2. **Collect issue suggestions** — items where `category == "issue"` from review JSONs (defer to § 9 audit). Fix suggestions already handled in § 4 / § 7.

3. **Deduplicate** by (location, description) — keep first, note all sources

4. **Present summary**:

   <output_format>

### REVIEW SUMMARY — [ISSUE_ID]

| Agent | Verdict | Blockers | Fix | Issue |
|-------|---------|----------|-----|-------|
| [AGENT_NAME] | ✅ pass | 0 | 0 | 1 |
| [AGENT_NAME] | ⚠️ action_required → fixed | 2 | 1 | 0 |

### ✅ FIXED BLOCKERS

| # | Source | Location | Description | Commit |
|---|--------|----------|-------------|--------|
| 1 | [agent] | [file:line] | [description] | [sha] |

### ⚠️ ESCALATED BLOCKERS

| # | Source | Location | Description | Pri |
|---|--------|----------|-------------|-----|
| 1 | [agent] | [file:line] | [description] | 🟠 |

### 📊 QA METRICS

[QA_METRICS] — project-configurable per QA agent type. Include agent-specific results as returned by each QA agent's JSON `qa_metadata` field. Example sections:

**[QA_AGENT_TYPE]**: [metric_1] [status] | [metric_2] [status] | ...

**Perf** (from `qa_metadata.perf_qa`, if perf-qa agent ran):

| Metric | Value |
|--------|-------|
| Percentiles | P50 [val] · P99 [val] · P99.9 [val] |
| Budget | [budget target] · Margin: [N]x |
| Platform | [platform] |
| Baseline | [baseline_sha] → [benchmark_commit] |
| Regressions | [N] hot-path ❌ · [N] cold-path ⚠️ · [N] intentional ℹ️ |

**If regressions[] non-empty**, expand each:

| Operation | Baseline | Current | Change | Class | Notes |
|-----------|----------|---------|--------|-------|-------|
| [op] | [val] | [val] | +X% | hot-path | ❌ BLOCKER |
| [op] | [val] | [val] | +X% | intentional | [decision_ref]: [reason] |

**Budget compliance** (key operations vs project performance budgets):

| Component | Operation | P50 | P99 | Budget | Status |
|-----------|-----------|-----|-----|--------|--------|
| [component] | [operation] | [val] | [val] | [budget] | ✅ |

---
Pri: 🔴 P1  🟠 P2  🟡 P3  🟤 P4
Est: 1 (hours) | 2 (half-day) | 3 (day) | 4 (2-3d) | 5 (week+)
Issue suggestions: [N] items → § 9 audit

   </output_format>

   **Omit empty sections.** Omit QA METRICS if no QA agents ran. Show issue suggestion count in legend if any exist.

## 9. Create Issues

1. **Read state**: `scripts/workflow-state get [ISSUE_ID] .escalated_items`

2. **Extract discovered work** from completion summaries:
   ```bash
   $ISSUE_CLI cache comments list [ISSUE_ID] | jq -r '.[] | select(.body | contains("Discovered Work")) | .body'
   ```
   If bundled: also extract from each sub-issue via `$ISSUE_CLI cache issues get [ISSUE_ID] --with-bundle | jq -r '.children[].id'`.
   Parse "Discovered Work" section bullets into audit items with `origin: "discovered"`, `found_by: [agent]`. Skip if section absent or "(Skip if none)".

3. **Skip if** no issue suggestions AND escalated_items empty AND no discovered work items. → § 10

4. **Build audit-input file** from:
   - Escalated items from state file
   - Issue suggestions (`category: "issue"` from review JSONs in state `json_paths`)
   - Discovered work items (from step 2, `origin: "discovered"`)

5. **Write file**: `[WORKTREE_PATH]/tmp/audit-start-YYYYMMDD-HHMMSS.json`
   - Schema: `schemas/audit-issues-input.md`

6. **Run Skill**: `⤵ /audit-issues --issues [FILE_PATH] § 1-9 → § 9 step 7`

7. **Update state** — for each created issue from audit output:
   ```bash
   scripts/workflow-state append [ISSUE_ID] audit_issues_created "[CREATED_ISSUE_ID]"
   ```

## 10. Delegate Pending Children

1. **Query pending children**:
   ```bash
   $ISSUE_CLI cache issues children [ISSUE_ID] --recursive --pending --format=safe
   ```

2. **Skip if** no pending children → § 11.

3. **Capture pre-delegate state**:
   ```bash
   scripts/workflow-state set [ISSUE_ID] pre_delegate_sha "$(git -C [WORKTREE_PATH] rev-parse HEAD)"
   ```

4. **Delegate immediately** — no exceptions, no asking user, no deferral. Delegate regardless of how sub-issues were created or their perceived scope.

   **Run Skill**: `⤵ /dev-start § 1-4 → § 10 step 5` with context:
   - `worktree`: [WORKTREE_PATH]
   - `lifecycle`: inherit current
   - `issue_id`: [ISSUE_ID]

5. **Assess re-review scope**:
   ```bash
   PRE_SHA=$(scripts/workflow-state get [ISSUE_ID] .pre_delegate_sha)
   $DIFF_SUMMARY_CMD -C [WORKTREE_PATH] $PRE_SHA
   ```

   | `risk_flags` | `scope` | Action | Route |
   |--------------|---------|--------|-------|
   | non-empty | any | — | → § 1 (full re-review) |
   | empty | `production` | `scripts/workflow-state set [ISSUE_ID] skip_qa true` | → § 1 |
   | empty | `support` | — | → § 11 |

## 11. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Check task** for return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — review cycle complete. Summary presented in § 8.
</content>
</invoke>
