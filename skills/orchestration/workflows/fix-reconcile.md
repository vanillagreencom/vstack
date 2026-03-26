# Fix Reconciliation

> **Dependencies**: `$ISSUE_CLI`, `scripts/workflow-state`

Batch workflow to check if applied fixes address existing open issues. Processes **all fixes at once**.

**Entry points**: start-worktree § 5.1, submit-pr § 6, review-pr-comments § 7.2

**Always invoke** — do not skip based on judgment. § 1 gathers fixes from workflow-state and skips internally if none exist.

**Execute**: § 1 → § 2 → § 3 → § 4 → § 5 → § 6 → § 7 → § 8

## Inputs

| Context | Source | Required |
|---------|--------|----------|
| `issue_id` | Caller issue ID for workflow-state lookup | Yes |
| `pr_number` | Caller PR number for comments | Yes |

## 1. Gather Fixes

1. **Read state**:
   ```bash
   REVIEW_FIXES=$(scripts/workflow-state get [ISSUE_ID] '.fixed_items // []')
   PR_FIXES=$(scripts/workflow-state get [ISSUE_ID] '.pr_comment_review.fixes // []')
   ```

2. **Merge** both arrays into `fixes`. Deduplicate by description.

3. **Skip if empty** — if `fixes` has 0 items → § 9 (return, nothing to reconcile).

## 2. Extract Keywords

**Single pass** — extract searchable keywords from ALL fix descriptions:

| Fix Description | Keywords |
|-----------------|----------|
| "Use ex.ToString() for inner exception chain" | `inner exception`, `exception chain` |
| "Add fallback to stderr" | `fallback`, `stderr` |
| "Fix race condition in producer" | `race condition`, `producer` |

Combine into single query pattern: `keyword1|keyword2|keyword3|...`

## 3. Query Existing Issues

**Single batch query**:

```bash
KEYWORDS="inner exception|exception chain|fallback|stderr|race condition|producer"

$ISSUE_CLI cache issues list --state "Backlog,Todo" --project "[CURRENT_PROJECT]" --max --search "$KEYWORDS"
```

**Store results** as `existing_issues`.

## 4. Assess Matches

For **each** fix, check against `existing_issues`:

### 4.1 Decision Tree

```
Fix matches existing issue?
│
├─ Fully addressed → close (fix completely solves issue)
├─ Partially addressed → descope (main problem solved, edge cases remain)
└─ No match / Unrelated → skip (keyword match but different context)
```

### 4.2 Build Results List

```
results: [
  { index: 1, fix: "Inner exception logging", recommendation: close, issue: "[ISSUE_ID]", reason: "Fully addresses" },
  { index: 2, fix: "Fallback chain", recommendation: descope, issue: "[ISSUE_ID]", reason: "Main fix done, edge cases remain" },
  { index: 3, fix: "Race condition fix", recommendation: skip, issue: null, reason: "No matching issues" },
  ...
]
```
No user display needed in this step. Proceed to § 5 for preview display.

## 5. Present Preview

Show table (same format for 1 or many fixes):

<output_format>

### FIX RECONCILIATION — [N] fix(es) from PR #[PR_NUMBER]

| # | Fix | Rec | Issue | Reason |
|---|-----|-----|-------|--------|
| 1 | Inner exception logging | ✅ | [ISSUE_ID] | Fully addr. |
| 2 | Fallback chain | 🔄 | [ISSUE_ID] | Partial |
| 3 | Race condition fix | ⏭️ | - | No match |

Legend: ✅ CLOSE  🔄 DESCOPE  ⏭️ SKIP

**Show details** for items with matches:
```
#1: [ISSUE_ID] "Exception logging improvements" — fully addressed, recommend CLOSE
#2: [ISSUE_ID] "Error handling fallbacks" — main fix done, recommend DESCOPE (edge cases remain)
```
</output_format>

## 6. User Decision

Use **single** ask user with multi-select:

**Question 1**: "Which issues to CLOSE?" (multi-select, skip if none)
- Options for each `close` item: `#N: Close [ISSUE_ID]`
- Include: `All recommended`, `None`

**Question 2**: "Which issues to DESCOPE?" (multi-select, skip if none)
- Options for each `descope` item: `#N: Descope [ISSUE_ID]`
- Include: `All recommended`, `None`

Unselected items = skip (no action).

## 7. Execute Actions

Execute **all** selected actions:

### close

```bash
$ISSUE_CLI comments create [ISSUE_ID] --body "Addressed in PR #[PR_NUMBER] (commit [SHA]).

Fix applied: [DESCRIPTION]"
$ISSUE_CLI issues update [ISSUE_ID] --state "Done"
```

### descope

```bash
$ISSUE_CLI issues update [ISSUE_ID] --description "[UPDATED_DESCRIPTION]"
$ISSUE_CLI comments create [ISSUE_ID] --body "Partially addressed in PR #[PR_NUMBER].

**Done**: [what was fixed]
**Remaining**: [what's left]"
```

### skip

No action. Issue remains as-is.

## 8. Present Results

<output_format>

### ✅ FIX RECONCILIATION COMPLETE

| Action | Count | Issues |
|--------|-------|--------|
| Closed | N | [ISSUE_ID], ... |
| Descoped | N | [ISSUE_ID], ... |
| Skipped | N | — |
</output_format>

## 9. Return State

**If managed** (`lifecycle: "managed"`):
   1. **Get task** on last task → description shows return section.
   2. **Continue there immediately**, do not stop.

**If standalone** (`lifecycle: "self"`):

**END** — reconciliation complete. Results presented in § 8.
