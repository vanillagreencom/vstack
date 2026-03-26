# Start New Issue Workflow

> **Dependencies**: issue tracker CLI (`$ISSUE_CLI`), worktree CLI (`$WORKTREE_CLI`), open-terminal
>
> **Requires**: issue tracker CLI (e.g., linear skill), worktree management CLI (e.g., worktree skill)

Create a new issue from scratch, set up worktree, and launch session.

## Inputs

| Command | Flow |
|---------|------|
| `/start-new` | § 1 → § 2 → § 3 |
| `/start-new [TITLE]` | Skip title prompt → § 1 step 2 → § 2 → § 3 |

---

## 1. Gather Intent

1. **Sync cache**:
   ```bash
   $ISSUE_CLI sync --reconcile
   ```

2. **If title provided** as argument → set `TITLE`, skip to step 3.

   **Otherwise** → Ask user: "What do you want to work on?" (free text)

   Parse response: first line = `TITLE`, rest = `DESCRIPTION_NOTES`.

3. **Ask user**: "Brief description? (or press enter to skip)"

   If response provided → set `DESCRIPTION_NOTES`.

---

## 2. Create Issue

### 2.1 Select Project

1. **List active projects**:
   ```bash
   $ISSUE_CLI cache projects list --state started --format=compact
   ```

2. **Suggest project** — infer from title/description keywords (project-configurable keyword-to-project mapping):

   | Keywords | Suggested Project |
   |----------|-------------------|
   | (domain-specific terms) | Matching project |
   | No match | Most recently active project |

3. **Ask user**: "Which project?" with options:
   - `[SUGGESTED_PROJECT]` (suggested, shown first)
   - Other active projects as additional options
   - `Other` (free text)

### 2.2 Determine Agent

Infer `agent:[TYPE]` label from title/description using project-configurable keyword-to-agent mapping:

| Keywords | Agent | Label |
|----------|-------|-------|
| (domain-specific terms) | [AGENT_TYPE] | `agent:[TYPE]` |
| No match | — | (no agent label) |

### 2.3 Create Bundle

Always create as a parent + sub-issue pair. Parent coordinates, child implements.

1. **Derive titles**:
   - `PARENT_TITLE`: High-level name (e.g., user says "add zoom" → "Feature: Zoom")
   - `CHILD_TITLE`: Implementation task (e.g., "Implement zoom feature") — keep bare unless user gave specific scope.

2. **Create parent issue**:
   ```bash
   $ISSUE_CLI issues create \
     --title "[PARENT_TITLE]" \
     --project "[PROJECT_ID]" \
     --description "## Sub-Issues\n\n- (pending creation)" \
     --state "Todo" \
     --labels "[AGENT_LABEL]"
   ```
   Capture `PARENT_ID`.

3. **Create sub-issue**:
   ```bash
   $ISSUE_CLI issues create \
     --title "[CHILD_TITLE]" \
     --project "[PROJECT_ID]" \
     --parent "[PARENT_ID]" \
     --description "[DESCRIPTION_NOTES if provided, otherwise: 'Scope TBD.']" \
     --state "Todo" \
     --labels "[AGENT_LABEL]"
   ```
   Capture `CHILD_ID`.

4. **Update parent description** with actual child ID:
   ```bash
   $ISSUE_CLI issues update [PARENT_ID] \
     --description "## Sub-Issues\n\n- [CHILD_ID]: [CHILD_TITLE]"
   ```

5. **Set `ISSUE_ID`** = `PARENT_ID` (worktree session orchestrates from parent, delegates sub-issues to agents).

6. **Output**:

   <output_format>

   Bundle: [PARENT_ID] — [PARENT_TITLE]
   └─ [CHILD_ID] — [CHILD_TITLE]
   Project: [PROJECT_NAME]
   Agent: [AGENT or "unassigned"]

   </output_format>

---

## 3. Create Worktree & Launch

1. **Run check**: `$WORKTREE_CLI check` — returns `{uncommitted, unpushed, unpushed_commits}`

2. **If uncommitted** → Ask user: `Stash` | `Commit` | `Continue anyway`

3. **If unpushed** → Ask user: `Push unpushed commits to main?` (show commits) → `git push origin main`

4. **Create worktree**: `WT_PATH=$($WORKTREE_CLI create [ISSUE_ID])`

5. **Build launch command** (harness-specific, e.g.):

   ```
   [HARNESS_CMD] --session [ISSUE_ID] '/start [ISSUE_ID]'
   ```

6. **Open terminal**:
   - **tmux** (`$TMUX` set) → launch directly, no prompt: `scripts/open-terminal [WT_PATH] --tmux [ISSUE_ID] --title [ISSUE_ID] --cmd [LAUNCH_CMD]`
   - **Otherwise** → Ask user: `Auto-open terminal` | `I'll open it myself`
     - **Auto**: `scripts/open-terminal [WT_PATH] --title [ISSUE_ID] --cmd [LAUNCH_CMD]`
     - **Manual**: Output "Worktree ready at `[WT_PATH]`." and the launch command for copy-paste.

   Output: "Opened [tmux window / terminal] for [ISSUE_ID]. This session is complete."

→ end
