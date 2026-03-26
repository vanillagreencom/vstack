# Initialize Session

> **Dependencies**: workflow-state, workflow-sections, session-init, issue tracker CLI (`$ISSUE_CLI`), worktree CLI (`$WORKTREE_CLI`)
>
> **Requires**: issue tracker CLI (e.g., linear skill), worktree management CLI (e.g., worktree skill)

Set up team, auth, cache, and workflow state for a worktree session.

## Inputs

| Command | Flow |
|---------|------|
| `/initialize` | § 1 → § 2 |
| `/initialize [ISSUE_ID]` | § 1 → § 2 |
| (from start-worktree.md) | Managed lifecycle with caller context |

**Caller context parameters** (via `⤵`):
- `lifecycle` (optional): `"managed"` (return to caller at § 2) | `"self"` (default, standalone).
- `issue_id` (optional): Issue ID. If absent, extracted from branch.

---

## 1. Initialize

**Create team first — before any other steps.**

1. **Extract ISSUE_ID**:
   - From argument if provided
   - Otherwise from branch: `git rev-parse --abbrev-ref HEAD` → parse `$ISSUE_PATTERN` (case-insensitive, project-configurable)

2. **Create team** (delete existing first if already leading one):
   ```
   Delete agent team (ignore error if no team exists)
   Create agent team: [ISSUE_ID_LOWERCASE]
   ```

3. **Pre-create workflow tasks** (skip if `lifecycle: "self"`):
   ```bash
   scripts/workflow-sections workflows/start-worktree.md
   ```
   Create task for each section.

4. **Run**: `scripts/session-init`

5. **If `gh_auth` is false or issue tracker auth is false** → report error and fix before proceeding.

6. **Set `WORKTREE_PATH`** to current working directory.

7. **Sync cache**:
   ```bash
   $ISSUE_CLI sync --reconcile
   ```

8. **Init workflow state**:
   ```bash
   scripts/workflow-state init [ISSUE_ID] --team "[ISSUE_ID_LOWERCASE]" \
     --agent "[AGENT]" --worktree "[WORKTREE_PATH]" --branch "[BRANCH]"
   ```
   QA fields (`--qa-labels`, `--sub-issues`) set later via `scripts/workflow-state set` when known.

---

## 2. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Get task details** on last task → description shows return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — session initialized.
