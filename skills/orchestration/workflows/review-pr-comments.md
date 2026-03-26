# PR Comment Triage Workflow

> **Dependencies**: `$GIT_HOST_CLI`, `$WORKTREE_CLI`, `$ISSUE_CLI` (optional), `$DECISIONS_CMD` (optional), `scripts/workflow-state`, `scripts/workflow-sections`

Route PR review comments to domain agents for analysis, auto-fix valid items, loop until stable.

## Inputs

| Command | Behavior |
|---------|----------|
| `/review-pr-comments` | Full triage: analyze, fix, create issues, reply |
| `/review-pr-comments [PR-number]` | Specific PR by number |
| `/review-pr-comments [BRANCH_NAME]` | Specific PR by branch |
| `/review-pr-comments --dry-run [N]` | Parse + analyze, stop before § 6 |
| (from submit-pr/start-worktree) | Managed lifecycle with caller context |

**Dry-run**: Runs § 1-5, presents triage report, stops before fixes. No side effects.

**Caller context parameters** (via `⤵`):
- `worktree`: worktree path
- `lifecycle` (optional): `"managed"` (return to caller at § 9) | `"self"` (default, standalone).
- `issue_id` (optional): issue tracker ID. If absent, extracted from branch.
- `pr_number` (optional): PR number. If absent, resolved from branch.

**Standalone init** (`lifecycle: "self"` only):
```bash
ISSUE_ID=$(git rev-parse --abbrev-ref HEAD | grep -oiP "$ISSUE_PATTERN")
PR_NUMBER=$(gh pr view --json number -q .number 2>/dev/null)
if ! scripts/workflow-state exists $ISSUE_ID; then
  WT_PATH=$($WORKTREE_CLI path $ISSUE_ID 2>/dev/null || echo ".")
  scripts/workflow-state init $ISSUE_ID --worktree "$WT_PATH" --branch "$(git rev-parse --abbrev-ref HEAD)"
fi
```

## API Error Handling

On any `gh` or `$GIT_HOST_CLI` command failure: halt, report error, ask user: `Retry` | `Skip step` | `Abort`.

---

## 1. Fetch & Parse PR Data

### 1.1 Wait for All Bot Reviews

Multiple bots may review (e.g., claude bot reviews fast, codex reviews arrive minutes later). Wait for all configured reviewers before triaging.

```bash
WAIT_RESULT=$(scripts/bot-review-wait [PR_NUMBER] 15 600 --json --reviewers "$BOT_REVIEWERS")
```

`$BOT_REVIEWERS` is a comma-separated list of bot usernames to wait for (e.g., `claude[bot],chatgpt-codex-connector[bot]`). Default: wait for any bot with a sticky/review comment.

**Polling behavior**: For each reviewer, wait until either a terminal verdict arrives or timeout. Proceed when all have reported or timeout is reached — late arrivals are caught in § 6.3.

### 1.2 Fetch Actionable Data

```bash
PR_DATA=$($GIT_HOST_CLI pr-data "[PR_NUMBER]" --actionable)
```

Output contains: `threads` (inline) + `comments` (PR-level).

### 1.3 Filter Comments

1. **Get baseline timestamp** for re-run filtering:
   ```bash
   mkdir -p tmp
   GH_USER=$(gh api user -q .login)
   $GIT_HOST_CLI find-comment [PR_NUMBER] --pattern "Recommendations.*Processed" --author "$GH_USER" > tmp/summary_comment_[PR_NUMBER].json
   SUMMARY_TS=$(jq -r '.updated_at // empty' tmp/summary_comment_[PR_NUMBER].json)
   ```

2. **Filter comments**. When `$SUMMARY_TS` is set, filter PR-level comments with `select(.created_at > $SUMMARY_TS)`.

   **Exclude (both sources):**
   - Noise bots: `dependabot[bot]`, `github-actions[bot]`, `renovate[bot]`, `linear`
   - **If re-run** (`SUMMARY_TS` set): Comments posted before `SUMMARY_TS`

   **Exclude (review threads only):**
   - Resolved threads (`isResolved: true`)
   - Outdated threads (`isOutdated: true`)

   **Exclude (PR-level comments only):**
   - Status updates with no actionable content

   **Keep**: All reviewer comments (human + bot) with actionable content + unresolved, current threads.

3. **Collect bot review comments** — from ALL review bots (not just one):
   ```bash
   # Get sticky/summary comments from each bot reviewer
   for BOT in $BOT_REVIEWERS; do
     $GIT_HOST_CLI find-comment [PR_NUMBER] --author "$BOT" --review-summary
   done
   ```
   If no bot reviews found: ask user `Wait` | `Skip triage`.

### 1.4 Extract Comment Data

1. **Extract from review threads** — fields per comment:
   - `thread_id`, `author`, `body`, `path`, `line`, `url`
   - `source`: `inline`

2. **Extract from PR-level comments** (human reviewers only):
   - `comment_id`, `author`, `body`, `path` (null), `line` (null), `url`
   - `source`: `pr-level`

3. **Parse bot review comments** — extract from ALL bot reviewers. For each bot's review comment or sticky, extract section headers and bullets. Categorize by keywords:

   | Keywords (case-insensitive) | Source Type |
   |-----------------------------|-------------|
   | "inline comment" | Skip (redundant with review threads) |
   | "architect", "design", "pattern" | `pr-level:architectural` |
   | "doc", "readme", "comment" | `pr-level:documentation` |
   | "security", "auth", "vulnerab", "inject" | `pr-level:security` |
   | "test", "coverage", "assert" | `pr-level:testing` |
   | "perf", "latency", "throughput" | `pr-level:performance` |
   | No match / "follow-up", "future", "todo" | `pr-level:suggestion` |

   Fields per extracted item:
   - `comment_id`, `author` (bot name), `body`, `section`, `path` (null), `line` (null), `url`
   - `source`: from keyword matching
   - `blocking`: `true` for security items, `false` if "non-blocking"/"optional", `false` otherwise

   **Bot inline threads**: Bot review threads (e.g., codex inline suggestions with priority badges) are already captured in step 1 as regular review threads. They carry the bot's username as `author` — do NOT filter them out.

### 1.5 Resolve Issue Context

1. **Identify parent issue**: Extract `[ISSUE_ID]` from branch name. If not found, ask user.

2. **Get worktree context**:
   ```bash
   WT_PATH=$($WORKTREE_CLI path [ISSUE_ID] 2>/dev/null || echo ".")
   ```

3. **Gather decision context** (decider skill): `$DECISIONS_CMD search --issue [ISSUE_ID]`. Collect matching decision IDs and summaries for § 3 delegation prompt.

---

## 2. Detect Domains

Map each comment to a domain based on source and file path. Domain-to-agent mapping is configurable per project. Example:

| Source / Path Pattern | Domain Agent |
|-----------------------|--------------|
| `pr-level:architectural` | arch-review |
| `pr-level:documentation` | doc-review |
| `pr-level:security` | security-review |
| `pr-level:testing` | test-review |
| `pr-level:performance` | perf-qa |
| `pr-level:suggestion` | infer from keywords, default arch-review |
| Path inference | infer from component paths (project-configurable) |
| `docs/**` | doc-review |
| No file path (general comment) | arch-review |

---

## 3. Analyze via Domain Agents

### 3.1 Route to Domain Agents (Parallel if Multiple)

**Delegate to domain agents** from § 2 mapping (parallel task calls if multiple).

<delegation_format>
Analyze these PR review comments for your domain.

PR: #[PR_NUMBER] - [TITLE]
Parent Issue: [ISSUE_ID]
Worktree: [WORKTREE_PATH]

Decision context (read before classifying — do NOT suggest changes that contradict these):
[For each matching decision: "[DECISION_ID]: [ONE_LINE_SUMMARY] — [DECISION_FILE_PATH]"]
[If none: "No linked decisions found."]

Comments for your review:
[For each comment:]
---
Source ID: [THREAD_ID or COMMENT_ID]
Source Type: [inline or pr-level]
Author: @[AUTHOR]
File: [PATH]:[LINE] (or "general" if no file)
Comment: "[BODY]"
Blocking: [true/false]
URL: [URL]
---

1. Read `workflows/recommendation-bias.md`. Apply its decision flow to ALL findings.
2. Classify into arrays per `schemas/review-finding.md` schema:
   - `blockers[]`: Passed checks + blocking=true or P1/P2 priority
   - `suggestions[]`: Passed checks + blocking=false
   - `questions[]`: QUESTION type — include draft response
   - NOISE or failed checks: Omit entirely
3. Preserve `source_id` and `source_type` from input on each item.
4. Save JSON to `[WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json`.
5. Return exactly:

   <output_format>
   Report: [WORKTREE_PATH]/tmp/review-[AGENT]-YYYYMMDD-HHMMSS.json
   Verdict: [pass|action_required]
   </output_format>
</delegation_format>

### 3.2 Collect Results

1. **Wait for all agents**. Extract `Report` path and `Verdict` from each.
2. **Store paths** in `JSON paths[]` for § 5.

---

## 4. Synthesize (if multi-domain)

**Skip if** comments from single domain only.

1. **Delegate to arch-review** for cross-cutting analysis:

   <delegation_format>
   Synthesize domain agent analyses of PR comments.

   PR: #[PR_NUMBER] - [TITLE]
   Parent Issue: [ISSUE_ID]
   Worktree: [WORKTREE_PATH]

   Domain Report JSONs:
   [List paths from § 3.2]

   Read each JSON. Identify cross-cutting concerns:
   1. Issues spanning multiple domains
   2. Dependencies between suggestions (output: `dependency: #A blocks #B (reason)`)
   3. Gaps at domain boundaries
   4. Conflicts between domain recommendations (flag both, don't resolve)

   **Do NOT** modify or overrule domain agent findings. Add your own only.

   1. Read `workflows/recommendation-bias.md`. Apply its decision flow.
   2. Build JSON per `schemas/review-finding.md` with YOUR cross-cutting findings only.
   3. Save to `[WORKTREE_PATH]/tmp/review-arch-synthesis-YYYYMMDD-HHMMSS.json` and return:

   <output_format>
   Report: [WORKTREE_PATH]/tmp/review-arch-synthesis-YYYYMMDD-HHMMSS.json
   Verdict: [pass|action_required]
   </output_format>
   </delegation_format>

2. **Add returned path** to `JSON paths[]`.

---

## 5. Present Triage Report

1. **Read all JSON files** from `JSON paths[]`

2. **Aggregate items** across all agents, preserving `agent` attribution:
   - `blockers[]` → Blockers
   - `suggestions[]` with `category: "fix"` → Fix Items
   - `suggestions[]` with `category: "issue"` → Issue Items (defer to § 6.2)
   - `questions[]` → Questions (auto-response in § 7.1)

3. **Deduplicate** by (location, description) — keep first, note all sources

4. **Decide action** for each fix item. Auto-fix all valid items — do NOT prompt for selection.

   | Item | Action | Reason |
   |------|--------|--------|
   | Blocker or fix with clear recommendation | **Fixing** | Valid, actionable |
   | Contradicts active decision | **Skipping** | Cite decision ID |
   | Vague, no clear deliverable | **Skipping** | Not actionable |
   | Unrelated to PR scope | **Skipping** | Out of scope → issue |

5. **Present table** showing what will be fixed and what won't:

<output_format>

### PR TRIAGE — #[PR_NUMBER] [TITLE] (pass [N])

| Field | Value |
|-------|-------|
| Branch | [headRefName] → Parent: [ISSUE_ID] |
| Reviewers | [BOT_1], [BOT_2], [HUMAN_1] |
| Summary | N blocker, N fix, N issue, N questions |

| Agent | Verdict | Blk | Fix | Issue | Q |
|-------|---------|-----|-----|-------|---|
| [AGENT] | ✅ pass | 0 | 1 | 0 | 0 |
| [AGENT] | ⚠️ action | 1 | 1 | 1 | 1 |

### 🔧 FIXING

| # | Agent | Author | Location | Description | Pri |
|---|-------|--------|----------|-------------|-----|
| 1 | [AGENT] | [BOT_1] | [file:line] | [description] | 🔴 |
| 2 | [AGENT] | [BOT_2] | [file:line] | [description] | 🟠 |

### ⏭️ SKIPPING

| # | Agent | Author | Location | Description | Reason |
|---|-------|--------|----------|-------------|--------|
| 1 | [agent] | codex[bot] | [file:line] | [description] | Contradicts D015 |

### 💬 QUESTIONS (auto-responding)

| # | Agent | Location | Question | Draft Response |
|---|-------|----------|----------|----------------|
| 1 | [agent] | [file:line] | [question] | [response] |

---
Pri: 🔴 P1  🟠 P2  🟡 P3  🟤 P4
Issue suggestions: [N] items → § 6.2 audit
</output_format>

**Omit empty sections.** Proceed immediately to § 6 — no user prompt.

---

## 6. Apply Fixes & Loop

### 6.1 Delegate Fixes

**Skip if** no items marked "Fixing" in § 5. → § 6.2

1. **Ensure worktree**: `WT_PATH=$($WORKTREE_CLI path [ISSUE_ID] 2>/dev/null || $WORKTREE_CLI create [ISSUE_ID] --pr [PR_NUMBER])`

2. **Group items** by `agent` field.

3. **Detect team context**: `scripts/workflow-state exists [ISSUE_ID] && TEAM=$(scripts/workflow-state get [ISSUE_ID] .team_name)`

4. **Delegate fixes** per agent group:

   **If in team** (`$TEAM` set): create tasks from issue-lifecycle dev-fix workflow, message existing dev teammate with delegation.

   **If standalone**: launch sub-agent with delegation.

   <delegation_format>
   Task prefix: [TASK_PREFIX]

   Workflow: issue-lifecycle skill dev-fix workflow

   Source: pr-comments
   Issue: [ISSUE_ID]
   PR: #[PR_NUMBER]
   Worktree: [WORKTREE_PATH]

   Review items:
   [For each item marked "Fixing":]
   ---
   #[N] | [AGENT] | [LOCATION]
   Title: "[TITLE]"
   Description: "[DESCRIPTION]"
   Recommendation: "[RECOMMENDATION]"
   ---
   </delegation_format>

5. **Wait for completion.**

6. **Handle results**:
   - Applied → mark for reply (§ 7.1)
   - Skipped by agent → add to skipped list with reason
   - Blocked → convert to issue (§ 6.2)

7. **Push**: `git -C "[WORKTREE_PATH]" push origin HEAD`

### 6.2 Create Issues

**Skip if** no issue suggestions AND no blocked items.

1. **Build audit-input file** from issue suggestions + blocked items
2. **Write file**: `[WORKTREE_PATH]/tmp/audit-pr-comments-YYYYMMDD-HHMMSS.json` per `schemas/audit-issues-input.md`
3. **Invoke workflow**: `⤵ /audit-issues --issues [FILE_PATH] § 1-9 → § 6.3`

### 6.3 Wait for New Comments & Re-Triage

After fixes are pushed, bots re-review. Wait for new comments, then loop.

1. **Update state**:
   ```bash
   scripts/workflow-state increment [ISSUE_ID] pr_comment_review.iterations
   ```

2. **Check iteration limit**:
   ```bash
   ITERATIONS=$(scripts/workflow-state get [ISSUE_ID] .pr_comment_review.iterations)
   ```
   **If** `ITERATIONS >= 5` → § 7 (max iterations, present skipped summary)

3. **Wait 5 minutes** for bot re-reviews to arrive:
   ```bash
   sleep 300
   ```

4. **Check for new comments**:
   ```bash
   # Count unresolved threads + new PR-level comments since last triage
   LAST_TS=$(scripts/workflow-state get [ISSUE_ID] '.pr_review_baseline.last_ts // empty')
   NEW_THREADS=$($GIT_HOST_CLI pr-threads [PR_NUMBER] --unresolved --since "$LAST_TS" | jq '.count')
   ```

5. **Route**:

   | `NEW_THREADS` | Action |
   |---------------|--------|
   | `0` | No new comments → § 7 |
   | `> 0` | New comments detected → update baseline, loop to § 1 |

6. **Update baseline** (before looping):
   ```bash
   NOW=$(date -u +%Y-%m-%dT%H:%M:%SZ)
   scripts/workflow-state set [ISSUE_ID] pr_review_baseline "{\"last_ts\":\"$NOW\",\"last_threads\":$NEW_THREADS}"
   ```

7. **Loop**: Return to § 1.2 (skip § 1.1 bot-wait on re-triage — comments already arrived).

---

## 7. Present Skipped Summary & Await User

**Always runs** after comment loop stabilizes (§ 6.3 exits with 0 new comments or max iterations).

### 7.1 Post Replies & Resolve Threads

1. **Post reply** to each comment from the final pass:

   | Outcome | Response |
   |---------|----------|
   | Applied | `Applied in [SHA]` |
   | Skipped (decision) | `Acknowledged — contradicts [DECISION_ID]` |
   | Skipped (not actionable) | `Acknowledged — not actionable` |
   | Blocked → issue | `Tracking in [ISSUE_ID]` |
   | Issue created | `Tracking in [ISSUE_ID]` |

   **For questions** (automatic): Post `draft_response` from JSON.

   **Posting mechanism:**
   - Inline threads: `$GIT_HOST_CLI post-reply "[THREAD_ID]" "[RESPONSE]" --pr "[PR_NUMBER]"`
   - PR-level comments: `$GIT_HOST_CLI post-comment "[PR_NUMBER]" "> Re: [QUOTE]\n\n[RESPONSE]"`
   - Use `1.` `2.` `3.` numbering, never `#N` (GitHub auto-links `#N` to PRs/issues)

   **Contested bot reviews** — when domain agent classifies a bot's blocking comment as noise:
   - Tag bot: `@[BOT_NAME] [RATIONALE]. Please re-review.`
   - Dismiss `CHANGES_REQUESTED`: `$GIT_HOST_CLI dismiss-review [PR_NUMBER] --bot --message "[RATIONALE]"`
   - Resolve the thread
   - **Human reviewers**: Tag `@[AUTHOR]` but do NOT dismiss

2. **Resolve threads**: Auto-resolve all threads where a reply was posted. Keep open only threads awaiting human response.

### 7.2 Present Final Summary

Aggregate all passes (§ 5 initial + § 6.3 re-triages). Show cumulative totals and all items NOT addressed.

<output_format>

### ✅ PR COMMENT TRIAGE COMPLETE

| Metric | Count |
|--------|-------|
| Triage passes | [N] |
| Fixed | [N] |
| Issues created | [N] |
| Replies posted | [N] |
| Threads resolved | [N] |

### ⏭️ ITEMS NOT ADDRESSED

| # | Author | Location | Description | Reason |
|---|--------|----------|-------------|--------|
| 1 | [BOT_1] | [file:fn] | [description] | Contradicts [DECISION_ID] — [reason] |
| 2 | [BOT_2] | [file:fn] | [description] | Not actionable — no specific deliverable |

(Empty if all items were addressed.)

Awaiting your response — ask questions, override skipped items, or confirm done.

</output_format>

**STOP and wait for user.** The user may:
- Ask about specific skipped items
- Override a skip: "fix #1" or "fix the refresh one"
- Ask follow-up questions about any comment
- Confirm done: → § 8

If user requests fixes for skipped items → delegate via § 6.1 (single item), push, then return here.

### 7.3 Reconcile & Post Summaries

**If managed** (`lifecycle: "managed"`): Skip → § 8

**If standalone** (`lifecycle: "self"`):

1. **Reconcile fixes** — skip if no fixes applied:
   Invoke: `⤵ /fix-reconcile § 1-9 → § 7.3 step 2`

2. **Post summary** — skip if no fixes AND no issues created:
   ```bash
   $GIT_HOST_CLI post-comment [PR_NUMBER] "[SUMMARY_CONTENT]"
   $ISSUE_CLI comments create [ISSUE_ID] --body "[SUMMARY_CONTENT]"
   ```

   ```markdown
   ## Recommendations Processed

   ### Fixed in PR
   - [SOURCE]: [ITEM] — [SHA]

   ### Issues Created
   - [ISSUE_ID] - [TITLE] — [PROJECT]

   ### Not Addressed
   - [SOURCE]: [ITEM] — [REASON]
   ```

---

## 8. Update State & Return

1. **Update state** with cumulative results:
   ```bash
   # For each fixed item:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.fixes '{"description":"[DESC]","location":"[LOC]","commit":"[SHA]","source":"[SOURCE]"}'

   # For each issue created:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.issues_created "[CREATED_ISSUE_ID]"

   # For each skipped item:
   scripts/workflow-state append [ISSUE_ID] pr_comment_review.skipped '{"description":"[DESC]","reason":"[REASON]"}'
   ```

2. **Return**:

   **If managed** (`lifecycle: "managed"`):
   - Check task for return section. Continue there immediately.

   **If standalone** (`lifecycle: "self"`):
   - **END** — triage complete. Results presented in § 7.2.
