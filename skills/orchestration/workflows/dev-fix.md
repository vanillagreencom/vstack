# Dev Fix Workflow

> **Dependencies**: `$ISSUE_CLI`, `$WORKTREE_CLI`, `scripts/workflow-state`, `scripts/workflow-sections`, issue-lifecycle skill workflows

Delegate fix items to specialist dev agent. Works standalone (user-initiated) or managed (from review-pr).

## Inputs

| Command | Behavior |
|---------|----------|
| `/dev-fix` | Fix items from conversation context |
| `/dev-fix [ISSUE_ID]` | Fix items for specific issue |
| (from review-pr workflow) | Managed lifecycle with caller context |

**Caller context parameters** (via `⤵`):
- `worktree`: worktree path
- `lifecycle` (optional): `"managed"` (return to caller at § 3) | `"self"` (default, standalone).
- `dev_agent` (optional): name of alive dev agent for fix delegation. If absent, determine from state/labels.
- `issue_id` (optional): Issue ID. If absent, extracted from branch.
- `items` (optional): formatted review items. If absent, build from conversation context.
- `source` (optional): `pr-review` | `qa-review`. Default: `conversation`.
- `qa_agent` (optional): QA agent name (for qa-review source).

**Standalone init** (`lifecycle: "self"` only):
```bash
ISSUE_ID=${ARG:-$(git rev-parse --abbrev-ref HEAD | grep -oiP "$ISSUE_PATTERN")}
WT_PATH=$($WORKTREE_CLI path $ISSUE_ID 2>/dev/null || pwd)
```

---

## 1. Build Fix Items

**If `items` provided** (managed): Use directly → § 2.

**If standalone**: Synthesize from conversation context.

1. **Gather context**: From the conversation, identify what needs fixing. Read relevant files if needed.

2. **Format each fix item**:
   ```
   ---
   #[N] | [conversation] | [file:line or "TBD"]
   Description: "[WHAT IS WRONG]"
   Recommendation: "[HOW TO FIX]"
   ---
   ```

3. **Present to user**:

   <output_format>

   ### Fix Items — [ISSUE_ID]

   | # | Location | Description | Recommendation |
   |---|----------|-------------|----------------|
   | 1 | [file:line] | [description] | [recommendation] |

   </output_format>

4. **Ask user**: `Fix all` | Multi-select: `#N: [TITLE]` | `Cancel`

   | Choice | Action |
   |--------|--------|
   | Cancel | → END |
   | Items selected | → § 2 |

---

## 2. Delegate

1. **Determine agent**:
   - If `dev_agent` provided → use it (already alive)
   - Otherwise: from workflow state or issue labels
     ```bash
     AGENT=$(scripts/workflow-state get $ISSUE_ID '.agent // empty' 2>/dev/null)
     [[ -z "$AGENT" ]] && AGENT=$($ISSUE_CLI cache issues get $ISSUE_ID --format=compact | jq -r '[.labels[] | select(startswith("agent:"))] | first | split(":")[1] // empty')
     ```

2. **Group items by agent domain** if multi-domain. Sequential per [agent-sequencing.md](workflows/agent-sequencing.md).

3. **Detect team context**:
   ```bash
   TEAM=$(scripts/workflow-state get $ISSUE_ID '.team_name // empty')
   ```

4. **Create agent tasks** (team session only):
   ```bash
   scripts/workflow-sections [path-to-issue-lifecycle-dev-fix-workflow] --agent "dev-fix" --emoji "🐲"
   ```
   Create task for each section (via harness task API).

5. **Delegate:**

   **If in team session** and agent alive (`dev_agent` or team member):
   ```
   Send delegation message to [DEV_AGENT]: content=DELEGATION, summary="Fix items"
   ```

   **If standalone** (no team):
   ```
   Launch sub-agent: type=[AGENT_TYPE], prompt=DELEGATION
   ```

   <delegation_format>
   Ultrathink.

   Task prefix: [TASK_PREFIX]

   Workflow: issue-lifecycle skill — dev-fix workflow

   Source: [SOURCE]
   Issue: [ISSUE_ID]
   Worktree: [WORKTREE_PATH]
   [If qa_agent:] QA: [QA_AGENT]

   Review items:
   [FORMATTED_ITEMS]
   </delegation_format>

6. **Wait for completion.** Parse return: item decisions (Applied/Skipped/Blocked), commits, validation status.

7. **Update state**:
   ```bash
   # For each applied item:
   scripts/workflow-state append [ISSUE_ID] fixed_items '{"description":"[DESC]","location":"[LOC]","commit":"[SHA]","source":"[SOURCE]"}'

   # For each escalated/skipped item:
   scripts/workflow-state append [ISSUE_ID] escalated_items '{"description":"[DESC]","location":"[LOC]","reason":"[REASON]","source":"[SOURCE]"}'

   scripts/workflow-state increment [ISSUE_ID] cycles
   ```

---

## 3. Return

**If standalone** (`lifecycle: "self"`):

1. **Present results**:

   <output_format>

   ### Fix Results — [ISSUE_ID]

   | # | Decision | Reasoning |
   |---|----------|-----------|
   | N | Applied/Skipped/Blocked | [explanation] |

   Commits: [SHAs or "none"]
   Validate: [status]

   </output_format>

2. **END**

**If managed** (`lifecycle: "managed"`):

Return parsed results to caller: item decisions, commits, validation status.

1. **Check last task** → description shows return section.
2. **Continue there immediately**, do not stop.
