# Submit PR Workflow

> **Dependencies**: `$GIT_HOST_CLI`, `$WORKTREE_CLI`, `$ISSUE_CLI` (optional), `$VALIDATE_CMD`, `$DECISIONS_CMD` (optional), `$VISUAL_QA_BASELINE_CMD` (optional), `scripts/workflow-state`, `scripts/workflow-sections`, `scripts/bot-review-wait`, `scripts/ci-wait`

Push changes, create/update PR, handle bot review, triage PR comments, and trigger CI.

## Inputs

| Command | Behavior |
|---------|----------|
| `/submit-pr` | Submit current branch as PR |
| `/submit-pr [PR#]` | Manage existing PR |
| (from start-worktree) | Managed lifecycle with caller context |

**Caller context parameters** (via `⤵`):
- `worktree`: worktree path
- `lifecycle` (optional): `"managed"` (return to caller at § 7) | `"self"` (default, standalone).
- `issue_id` (optional): issue tracker ID. If absent, extracted from branch.

**If PR# provided:**
```bash
ISSUE_ID=$($GIT_HOST_CLI pr-issue [PR_NUMBER] --format=text)
WT_PATH=$($WORKTREE_CLI path $ISSUE_ID 2>/dev/null || echo ".")
```

**If no argument:** Set `WT_PATH` to current directory.

**Standalone init** (`lifecycle: "self"` only):
```bash
# Extract issue from branch if not provided
ISSUE_ID=$(git rev-parse --abbrev-ref HEAD | grep -oiP "$ISSUE_PATTERN")
WT_PATH=$($WORKTREE_CLI path $ISSUE_ID 2>/dev/null || echo ".")
# Init workflow state if not exists
if ! scripts/workflow-state exists $ISSUE_ID; then
  scripts/workflow-state init $ISSUE_ID --worktree "$WT_PATH" --branch "$(git rev-parse --abbrev-ref HEAD)"
fi
```

---

## 1. Push and Submit PR

1. **Push branch**:
   ```bash
   $WORKTREE_CLI push "[WORKTREE_PATH]" --set-upstream
   ```

2. **Check for existing PR**:
   ```bash
   PR_NUM=$($GIT_HOST_CLI -C "[WORKTREE_PATH]" pr-view --json number,state 2>/dev/null | jq -r .number)
   ```

3. **Build PR body** from current workflow state using template (omit empty sections):

   ```markdown
   ## Summary
   [1-3 bullets describing changes]

   ## Context
   [For each matching decision from `$DECISIONS_CMD search --issue [ISSUE_ID]` (decider skill):]
   - **[DECISION_ID]**: [ONE_LINE_SUMMARY] — `[DECISION_FILE_PATH]`
   [For each research file linked to the issue:]
   - **Research**: [TITLE] — `[RESEARCH_FILE_PATH]`

   ## Completed Issues
   - Closes [ISSUE_ID] - [TITLE]
     - Closes [SUB_ISSUE_1] - [SUB_TITLE]
     - Closes [SUB_ISSUE_2] - [SUB_TITLE]

   ## Created Issues
   - [ISSUE_ID] - [TITLE] — Project: [PROJECT]

   ## QA Metrics
   [QA_METRICS] — project-configurable. Include results from QA agents that ran during review.

   ## Test Plan
   [validation steps]
   ```

   - **Completed Issues**: Use `Closes` keyword for issue tracker linkage. Indent sub-issues.
   - **Created Issues**: Include if issues created during review.
   - **QA Metrics**: Include if QA agents ran. Format is project-configurable based on which QA agent types are active.

4. **Create or update PR**:

   **No existing PR** → create with `defer-ci` label:
   ```bash
   ISSUE_TITLE=$($ISSUE_CLI cache issues get [ISSUE_ID] | jq -r '.title')

   $GIT_HOST_CLI -C "[WORKTREE_PATH]" pr-create \
     --title "[PREFIX]([ISSUE_ID]): $ISSUE_TITLE" \
     --body "[PR_BODY]" \
     --label defer-ci
   ```

   **Existing PR** (`$PR_NUM` set) → update body and ensure label:
   ```bash
   gh pr edit "$PR_NUM" --body "[PR_BODY]" --add-label defer-ci 2>/dev/null || true
   ```

---

## 2. Wait for Bot Review

Wait for bot review to complete (sticky comment with verdict). CI is deferred via label.

```bash
WAIT_RESULT=$(scripts/bot-review-wait [PR_NUMBER] 15 600 --json --reviewers "$BOT_REVIEWERS")
BOT_STATUS=$(echo "$WAIT_RESULT" | jq -r '.status')
BOT_VERDICT=$(echo "$WAIT_RESULT" | jq -r '.verdict')
```

Waits for all configured bot reviewers (`$BOT_REVIEWERS` — e.g., `review-bot-a[bot],review-bot-b[bot]`). Auto-detects if not configured. Max wait 600s.

**Route result**:

| `status` | `verdict` | Action |
|----------|-----------|--------|
| `complete` | any | → § 3 |
| `timeout` | `approved` or `changes` | → § 3 (terminal verdict, safe) |
| `timeout` | `pending` | Extended poll below, then → § 3 |
| `checklist_timeout` | `approved` or `changes` | Ask user (see below) |
| `checklist_timeout` | `pending` | Extended poll below, then → § 3 |

**`checklist_timeout` with terminal verdict** — the bot submitted its review but is still posting inline threads. Prompt the user:

> Ask user: "Bot review verdict is **[BOT_VERDICT]** but it is still posting inline threads (checklist items unchecked). Options:"
> - **Wait 5 min** — poll again for up to 300s, then re-route
> - **Proceed** — skip remaining threads and move to comment triage now (may miss late threads)

```bash
# "Wait 5 min" path: extend checklist wait
EXT_ELAPSED=0
while [ $EXT_ELAPSED -lt 300 ]; do
  CHECKLIST_DONE=$($GIT_HOST_CLI sticky-comment [PR_NUMBER] --body 2>/dev/null \
    | grep -c '^\s*- \[ \]' || true)
  if [ "$CHECKLIST_DONE" -eq 0 ]; then break; fi
  sleep 30
  EXT_ELAPSED=$((EXT_ELAPSED + 30))
done
# → § 3 regardless
```

**Extended poll** (timeout + pending only):
```bash
# Poll sticky verdict every 30s for up to 300s more
EXT_ELAPSED=0
while [ $EXT_ELAPSED -lt 300 ]; do
  BOT_VERDICT=$($GIT_HOST_CLI sticky-comment [PR_NUMBER] --verdict 2>/dev/null || echo "pending")
  if [[ "$BOT_VERDICT" == "approved" || "$BOT_VERDICT" == "changes" ]]; then
    break
  fi
  sleep 30
  EXT_ELAPSED=$((EXT_ELAPSED + 30))
done
# Proceed to § 3 regardless (with note if still pending)
```

---

## 3. Comment Triage

### 3.1 Initial Triage

1. **Bot completion pre-check** — ensure sticky verdict is terminal before triaging:
   ```bash
   VERDICT=$($GIT_HOST_CLI sticky-comment [PR_NUMBER] --verdict 2>/dev/null || echo "pending")
   if [[ "$VERDICT" == "pending" ]]; then
     # Poll every 30s for up to 180s
     PRE_ELAPSED=0
     while [ $PRE_ELAPSED -lt 180 ]; do
       sleep 30
       PRE_ELAPSED=$((PRE_ELAPSED + 30))
       VERDICT=$($GIT_HOST_CLI sticky-comment [PR_NUMBER] --verdict 2>/dev/null || echo "pending")
       if [[ "$VERDICT" != "pending" ]]; then break; fi
     done
   fi
   # Proceed regardless — terminal or timed out
   ```

2. **Run Skill**: `⤵ /review-pr-comments [PR_NUMBER] § 1-8 → § 3.1` with context:
   - `lifecycle`: `"managed"`
   - `issue_id`: `[ISSUE_ID]`
   - `worktree`: `[WORKTREE_PATH]`

3. **Update state**:
   ```bash
   # For each fixed item:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.fixes '{"description":"[DESC]","location":"[LOC]","commit":"[SHA]","source":"[SOURCE]"}'

   # For each issue created:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.issues_created "[CREATED_ISSUE_ID]"

   # For each skipped item:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.skipped '{"description":"[DESC]","reason":"[REASON]"}'

   # Increment iteration count
   scripts/workflow-state increment [ISSUE_ID] pr_comment_review.iterations
   ```

4. **Route**:

   **If issues created** → § 3.3

   **If fixes applied** (no issues) → § 3.2 (re-review loop)

   **If no items fixed** AND no issues created → § 4

### 3.2 Re-Review Loop

After fixes pushed, wait for bot re-review (CI still deferred). Re-run `/review-pr-comments` until approved or stable.

1. **Check iteration count**:
   ```bash
   ITERATIONS=$(scripts/workflow-state get [ISSUE_ID] .pr_comment_review.iterations)
   # Max 3 iterations
   if [ "$ITERATIONS" -ge 3 ]; then
     # → Max iterations exceeded → § 4
   fi
   ```

2. **Wait for bot re-review** after fixes pushed:
   ```bash
   # 1. Wait for bot to update review
   scripts/bot-review-wait [PR_NUMBER]

   # 2. Read baseline from state
   LAST_TS=$(scripts/workflow-state get [ISSUE_ID] '.pr_review_baseline.last_ts // empty')
   LAST_THREADS=$(scripts/workflow-state get [ISSUE_ID] '.pr_review_baseline.last_threads // 0')

   # 3. Check status against baseline
   $GIT_HOST_CLI pr-review-status [PR_NUMBER] --baseline-ts "$LAST_TS" --baseline-threads "$LAST_THREADS" > tmp/pr_status_[PR_NUMBER].json
   ```

3. **Route based on status**:

   | `needs_action` | `reason` | Action |
   |----------------|----------|--------|
   | `false` | `no_sticky` | Ask user: `Wait` \| `Skip` |
   | `false` | `no_change` | → § 4 (nothing new) |
   | `false` | `approved_clean` | → § 4 (success) |
   | `true` | `has_threads` | `⤵ /review-pr-comments [PR_NUMBER] § 1-8 → § 3.2` with managed context, then update state, repeat |
   | `true` | `verdict_not_approved` | `⤵ /review-pr-comments [PR_NUMBER] § 1-8 → § 3.2` with managed context, then update state, repeat |

4. **Update state** after `/review-pr-comments` — if no fixes applied → § 4. Otherwise:
   ```bash
   # Increment iteration count
   scripts/workflow-state increment [ISSUE_ID] pr_comment_review.iterations

   # Add fixes/issues/skipped (same as § 3.1 step 3)

   # Update baseline
   NEW_TS=$(jq -r '.sticky_updated_at' tmp/pr_status_[PR_NUMBER].json)
   NEW_THREADS=$(jq -r '.unresolved_threads' tmp/pr_status_[PR_NUMBER].json)
   scripts/workflow-state set [ISSUE_ID] pr_review_baseline "{\"last_ts\":\"$NEW_TS\",\"last_threads\":$NEW_THREADS}"
   ```

5. **Max iterations exceeded**: Report to user with status, recommendation, and proceed to § 4.

### 3.3 Implement Created Issues

Sub-issues created during comment triage need implementation before CI.

1. **Check cycle count**:
   ```bash
   SUBMIT_CYCLES=$(scripts/workflow-state get [ISSUE_ID] '.submit_cycles // 0')
   ```
   **If** `SUBMIT_CYCLES >= 2` → § 4 with note: "Max re-submit cycles reached, created issues may need manual implementation."

2. **Increment**:
   ```bash
   scripts/workflow-state increment [ISSUE_ID] submit_cycles
   ```

3. **Implement**: `⤵ /dev-start § 1-4 → § 3.3 step 4` with context:
   - `worktree`: [WORKTREE_PATH]
   - `lifecycle`: `"managed"`
   - `issue_id`: [ISSUE_ID]

4. **Review**: `⤵ /review-pr § 1-11 → § 3.3 step 5` with context:
   - `worktree`: [WORKTREE_PATH]
   - `lifecycle`: `"managed"`
   - `dev_agent`: from dev-start return
   - `issue_id`: [ISSUE_ID]

5. **Re-submit** → § 1 (push updated code, update PR body with new `Closes` lines, re-trigger bot review)

---

## 3.5. Update Golden Baselines

**Skip if** the issue does not have the `design` label.

```bash
LABELS=$($ISSUE_CLI cache issues get "$ISSUE_ID" --format=compact 2>/dev/null | jq -r '.labels[]' 2>/dev/null)
```

If `design` label present:

1. **Capture baselines in worktree**:
   - If the project defines `$VISUAL_QA_BASELINE_CMD`, use it. This command may set a target, run extra preparation, or skip capture for projects whose current visual-QA target does not support baselines.
   - Otherwise, run the visual-qa skill's baseline capture on the default baseline-capable target.
   ```bash
   (cd [WT_PATH] && visual-qa skill baseline capture)
   ```
   The visual-qa skill builds the binary and writes baseline artifacts to the worktree's test data directory. If the project has no baseline-capable target, skip this step and report why.

2. **Commit and push** (without retriggering CI). Baselines are platform-specific:
   ```bash
   git -C [WT_PATH] add [BASELINE_PATH]/
   git -C [WT_PATH] commit -m "chore(visual-qa): update golden baselines [skip ci]"
   $WORKTREE_CLI push [WT_PATH] --no-rebase
   ```

3. **Report**: `Golden baselines: updated (N scenarios)` or if capture fails, include failure reason from baseline report.

---

## 4. Trigger CI

All bot review comments resolved (or max iterations). Verify no late-arriving threads, then remove `defer-ci` label to trigger CI.

1. **Thread propagation delay** — bot may still be posting inline threads after sticky verdict:
   ```bash
   # Wait for late-arriving threads (bot posts inline comments after sticky update)
   sleep 15
   UNRESOLVED=$($GIT_HOST_CLI pr-threads [PR_NUMBER] --unresolved | jq '.unresolved_count')
   if [ "$UNRESOLVED" -eq 0 ]; then
     # Double-check after additional delay to catch very late threads
     sleep 15
     UNRESOLVED=$($GIT_HOST_CLI pr-threads [PR_NUMBER] --unresolved | jq '.unresolved_count')
   fi
   CI_GATE_REROUTED=$(scripts/workflow-state get [ISSUE_ID] '.pr_comment_review.ci_gate_rerouted // false')
   ```

   | `UNRESOLVED` | `CI_GATE_REROUTED` | Action |
   |--------------|---------------------|--------|
   | `0` | any | → step 2 (remove label) |
   | `>0` | `false` | Set `ci_gate_rerouted=true`, → § 3.1 (one triage pass) |
   | `>0` | `true` | Ask user: "Bot posted N unresolved threads after iteration limit" — `Triage now` \| `Skip and trigger CI` \| `Abort` |

   ```bash
   if [ "$UNRESOLVED" -gt 0 ]; then
     if [ "$CI_GATE_REROUTED" = "false" ]; then
       scripts/workflow-state set [ISSUE_ID] pr_comment_review.ci_gate_rerouted true
       # → § 3.1
     else
       # Ask user with 3 options
     fi
   fi
   ```

2. **Remove label**:
   ```bash
   gh pr edit [PR_NUMBER] --remove-label defer-ci
   ```

3. **Wait for CI**:
   ```bash
   scripts/ci-wait [PR_NUMBER]
   ```

4. **Handle CI result**:

   | Result | Action |
   |--------|--------|
   | ✅ Pass | → § 6 |
   | ❌ Fail | → § 5 |

---

## 5. CI Failure Recovery

1. **Run Skill**: `⤵ /ci-fix [PR_NUMBER] § 1-7 → § 5`

2. **After ci-fix returns**:
   - If fix applied → add `defer-ci` label, push, wait for bot re-review (§ 3.2 with iteration check)
   - If fix not possible → Ask user: `Skip CI` | `Retry` | `Abort`

3. **Max 2 ci-fix cycles** per PR submission.

4. **After max cycles** → § 6 with note: "CI failing, may need manual intervention"

---

## 6. Standalone Summary

**If managed**: Skip → § 7

**If standalone**:

1. **Reconcile fixes**:

   Run Skill: `⤵ /fix-reconcile § 1-9 → § 6 step 2` with context:
   - `issue_id`: [ISSUE_ID]
   - `pr_number`: [PR_NUMBER]

2. **Post summary** — skip if no fixes AND no issues created:
   ```bash
   $GIT_HOST_CLI post-comment [PR_NUMBER] "[SUMMARY_CONTENT]"
   $ISSUE_CLI comments create [ISSUE_ID] --body "[SUMMARY_CONTENT]"
   ```

   **Summary content template** (omit empty sections):

   ```markdown
   ## Recommendations Processed

   ### Fixed in PR
   - [SOURCE]: [ITEM] — [SHA]

   ### Issues Created
   - [ISSUE_ID] - [TITLE] — [PROJECT]

   ### Skipped
   - [SOURCE]: [ITEM] — [REASON]
   ```

3. **Output result**:

   <output_format>

   ### ✅ PR SUBMITTED — #[PR_NUMBER]

   | Metric | Value |
   |--------|-------|
   | PR | #[PR_NUMBER] |
   | CI | ✅ passing / ❌ failing |
   | Bot | ✅ approved / ⚠️ changes |
   | Comment iterations | [N] |
   | Fixes applied | [N] |
   | Issues created | [N] |

   </output_format>

4. **Offer merge** — skip if CI not passing:

   → Ask user: `Run /merge-pr [PR_NUMBER]` | `Skip`

   | Choice | Action |
   |--------|--------|
   | Merge | `⤵ /merge-pr [PR_NUMBER] § 1-7 → end` |
   | Skip | → end |

---

## 7. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Check task** for return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — PR submitted. Summary presented in § 6.
</content>
</invoke>
