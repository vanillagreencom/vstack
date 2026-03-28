# Dev Implementation Workflow

> **Dependencies**: `$ISSUE_CLI`, `$WORKTREE_CLI`, `scripts/workflow-state`, `scripts/workflow-sections`, issue-lifecycle skill workflows

Delegate development work to specialist agent(s). Handles single issues and bundled multi-agent work with handoff.

## Inputs

| Command | Behavior |
|---------|----------|
| `/dev-start` | Implement current branch's issue |
| `/dev-start [ISSUE_ID]` | Implement specific issue (or sub-issue from start-new session) |
| (from start-worktree / review-pr workflows) | Managed lifecycle with caller context |

**Caller context parameters** (via `⤵`):
- `worktree`: worktree path
- `lifecycle` (optional): `"managed"` (return to caller at § 4) | `"self"` (default, standalone).
- `issue_id` (optional): Issue ID. If absent, extracted from branch.

**Standalone init** (`lifecycle: "self"` only):
```bash
# Use argument if provided, else extract from branch
ISSUE_ID=${ARG:-$(git rev-parse --abbrev-ref HEAD | grep -oiP "$ISSUE_PATTERN")}
WT_PATH=$($WORKTREE_CLI path $ISSUE_ID 2>/dev/null || pwd)

# Init workflow state if not exists
if ! scripts/workflow-state exists $ISSUE_ID; then
  # Check for parent context (start-new flow: sub-issue in parent's worktree)
  PARENT_ID=$($ISSUE_CLI cache issues get $ISSUE_ID --format=compact | jq -r '.parent.identifier // empty')
  if [[ -n "$PARENT_ID" ]] && scripts/workflow-state exists $PARENT_ID; then
    TEAM=$(scripts/workflow-state get $PARENT_ID '.team_name // empty')
    WT_PATH=$(scripts/workflow-state get $PARENT_ID '.worktree // empty')
    scripts/workflow-state init $ISSUE_ID --worktree "$WT_PATH" --branch "$(git rev-parse --abbrev-ref HEAD)" --team "$TEAM"
  else
    scripts/workflow-state init $ISSUE_ID --worktree "$WT_PATH" --branch "$(git rev-parse --abbrev-ref HEAD)"
  fi
fi
```

---

## 1. Determine Agent

`agent:X` label → X | No label → infer from component paths.

```bash
$ISSUE_CLI cache issues get [ISSUE_ID] --format=compact | jq -r '.labels[]'
```

---

## 2. Delegate to Specialist Agent(s)

**Dev agents persist for the entire session.** Never shutdown dev agents — they stay alive for re-delegation (fix cycles, pending children, PR review fixes). Only the caller's finalization step shuts them down.

**Detect team context**:
```bash
TEAM=$(scripts/workflow-state get [ISSUE_ID] '.team_name // empty')
```

### If Single Issue

**If in team session** (`$TEAM` set):

1. **Create agent tasks**:
   ```bash
   scripts/workflow-sections [path-to-issue-lifecycle-dev-implement-workflow] --agent "dev-implement" --emoji "🐲"
   ```
   Create task for each section (via harness task API).

2. **Spawn dev teammate** (if not alive) — check team config for existing agent:
   ```
   Spawn agent: type=[AGENT_TYPE], name=[AGENT], team=[TEAM]
   ```
   Copy spawn prompt **verbatim** from spawn prompt templates (project-level) (fill `[PLACEHOLDERS]` only). Agent goes idle waiting for delegation.

3. **Delegate via message**:
   ```
   Send delegation message to [AGENT]: content=DELEGATION, summary="Implement [ISSUE_ID]"
   ```

4. **Wait for completion message**. Parse: Branch, Commit, QA Labels, Summary.

**If standalone** (no team):

Launch sub-agent:
```
Launch sub-agent: type=[AGENT_TYPE], prompt=DELEGATION
```
Wait for return. Parse: Branch, Commit, QA Labels, Summary.

**Single issue delegation prompt:**

<delegation_format>
Ultrathink.

Task prefix: [TASK_PREFIX]

Workflow: issue-lifecycle skill — dev-implement workflow

Issue: [ISSUE_ID]
Worktree: [WORKTREE_PATH]
Labels: [LABELS]
Blocks: [BLOCKED_ISSUE_IDS or "none"]
</delegation_format>

### If Bundled Issue

**Agent grouping**: Group pending sub-issues by `agent:[TYPE]` label. Read [agent-sequencing.md](agent-sequencing.md) for ordering. Process sequentially: first group → wait for completion → validate (§ 3) → collect handoff notes → next group. Each agent receives its sub-issues + completed sub-issues from prior agents as context.

**Handoff collection** (between agent groups): After each agent group returns and passes § 3 validation, before delegating the next group:

a. For each sub-issue completed by any prior agent group (cumulative, not just the latest):
   ```bash
   $ISSUE_CLI cache comments list [COMPLETED_ISSUE_ID] | jq -r '.[] | select(.body | contains("Handoff Notes")) | .body'
   ```
b. Extract "Handoff Notes" sections. Combine into a single block.
c. Include in next delegation as the `Handoff from prior agents:` field (see delegation format below).

If no handoff notes found, omit the section.

**If in team session** (`$TEAM` set):

1. **Create agent tasks** — bundled uses per-sub-issue tasks (NOT `workflow-sections`):

   **Setup tasks** (§ 1-3):
   ```
   Create task: "⏤⏤🐲 dev-implement § 1: Environment Setup"
     description="Execute section 1 from issue-lifecycle dev-implement workflow"
   Create task: "⏤⏤🐲 dev-implement § 2: Activate Issue"
     description="Execute section 2 from issue-lifecycle dev-implement workflow"
   Create task: "⏤⏤🐲 dev-implement § 3: Block Issue"
     description="Execute section 3 from issue-lifecycle dev-implement workflow"
   ```

   **Per-sub-issue tasks** (§ 4-10, one per pending sub-issue, in blocking order):
   ```
   Create task: "⏤⏤🐲 dev-implement § 4-10: [SUB_ISSUE_1] — [TITLE]"
     description="Execute sections 4-10 from issue-lifecycle dev-implement workflow for sub-issue [SUB_ISSUE_1]: [TITLE]"
   ```

   **Return task** (§ 11):
   ```
   Create task: "⏤⏤🐲 dev-implement § 11: Return to Orchestrator"
     description="Execute section 11 from issue-lifecycle dev-implement workflow"
   ```

2. **Spawn dev teammate** (if not alive) — check team config for existing agent:
   ```
   Spawn agent: type=[AGENT_TYPE], name=[AGENT], team=[TEAM]
   ```
   Copy spawn prompt **verbatim** from spawn prompt templates (project-level) (fill `[PLACEHOLDERS]` only). Agent goes idle waiting for delegation.

3. **Delegate via message**:
   ```
   Send delegation message to [AGENT]: content=DELEGATION, summary="Implement [ISSUE_ID] bundle"
   ```

4. **Wait for completion message**. Parse: Branch, Commit, QA Labels, Summary.

**If standalone** (no team):

Launch sub-agent:
```
Launch sub-agent: type=[AGENT_TYPE], prompt=DELEGATION
```
Wait for return. Parse: Branch, Commit, QA Labels, Summary.

**Bundled issue delegation prompt:**

<delegation_format>
Ultrathink.

Task prefix: [TASK_PREFIX]

Workflow: issue-lifecycle skill — dev-implement workflow

Parent: [ISSUE_ID]
Sub-Issues:
[For completed sub-issues:]
↳ [SUB_ISSUE_1] (completed): [TITLE]
[For pending sub-issues assigned to this agent:]
↳ [SUB_ISSUE_2]: [TITLE] | blocks: [SUB_ISSUE_3]
↳ [SUB_ISSUE_3]: [TITLE] | blocked by: [SUB_ISSUE_2]
   ↳ [SUB_ISSUE_4]: [TITLE]  ← nested child of [SUB_ISSUE_3]

Worktree: [WORKTREE_PATH]
Labels: [parent labels]
Blocks: [blocked-issue-ids or "none"]

**Work pending issues only** (completed listed for context). Respect blocking order: complete blockers before blocked issues.

**Scope**: Implement YOUR assigned sub-issues only. You may fix/connect prior agents' code if needed, but do not implement work belonging to other agents' pending sub-issues.

Current status of issue bundle: [Brief summary of what was already done from other agents.]

[If handoff notes collected from prior agent groups:]
Handoff from prior agents:
[[ISSUE_ID] (agent:[TYPE])]:
- [extracted handoff notes]
</delegation_format>

---

## 3. Validate Agent Return

**Expected format**: `Branch: ... | Commit: [SHA] | QA Labels: ... | Summary: Posted ✓`

1. **Run ALL checks** — do not proceed if ANY fails:
   ```bash
   # Check commit exists
   git -C "[WORKTREE_PATH]" log -1 --oneline

   # Check state + summary (auto-includes pending children from bundle)
   $ISSUE_CLI issues validate-completion [ISSUE_ID] --include-children-of [ISSUE_ID]
   ```

2. **Evaluate results**:

   | Field | Expected | Failure Action |
   |-------|----------|----------------|
   | commit | exists | Re-delegate § 7-10 |
   | `.all_ok` | `true` | Check `.results[]` below |
   | `.results[].state_ok` | `true` | Re-delegate § 2 |
   | `.results[].has_summary` | `true` | Re-delegate § 9 |

3. **On failure**: Do NOT proceed. Re-message existing agent (team) or resume agent (standalone) with retry instructions specifying the missing step(s). Never proceed with "may have a different format" or similar excuses.
   ```
   # Team: Send delegation message to [AGENT]: content="[RETRY_INSTRUCTIONS]", summary="Retry [STEP]"
   # Standalone: Resume agent: [AGENT_ID], prompt="[RETRY_INSTRUCTIONS]"
   ```

4. **Store QA state**:
   ```bash
   scripts/workflow-state set [ISSUE_ID] qa_labels '[QA_LABELS_ARRAY]'
   scripts/workflow-state set [ISSUE_ID] sub_issues '[SUB_ISSUE_IDS_ARRAY]'
   ```

5. **If validate failures reported**: Investigate, suggest sub-issue (summary, steps, agent). Ask user before creating.

---

## 4. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Check last task** → description shows return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — dev implementation complete.
