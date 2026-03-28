# Start Session Workflow

> **Dependencies**: issue tracker CLI (`$ISSUE_CLI`), git host CLI (`$GIT_HOST_CLI`), worktree CLI (`$WORKTREE_CLI`), workflow-state, workflow-sections, session-init, open-terminal, parallel-launch, parallel-groups, `$VALIDATE_CMD`, `$DECISIONS_CMD`
>
> **Requires**: issue tracker CLI (e.g., linear skill), git host CLI (e.g., github skill), worktree management CLI (e.g., worktree skill)

Initialize development session, display status, select work, evaluate research, create worktree, and hand off to worktree session.

## 1. Initialize Session

### 1.1 Sync Cache

```bash
$ISSUE_CLI sync --reconcile
```

### 1.2 Present Dashboard

1. **Run**: `scripts/session-init`

2. **Output the result exactly as shown** — no reformatting, no additions, no commentary before/after the dashboard. The script output IS the dashboard. Script output uses backticks and markdown syntax.

3. **If build errors or auth failures** → fix locally before proceeding to § 1.3.

4. **If dashboard shows "Worktree session"** → STOP. Wrong workflow. Use [start-worktree.md](start-worktree.md) instead.

### 1.3 Select Work

**Skip if** `/start [ISSUE_ID]` provided → § 1.4

1. **Check recommendation**: The `Recommended` line shows the priority action.

2. **Present options**:

   | Recommendation | Action |
   |----------------|--------|
   | `/ci-fix N` | `⤵ /ci-fix N § 1-7 → § 1` |
   | `/review-pr-comments N` | `⤵ /review-pr-comments N § 1-8 → § 1` |
   | `/merge-pr N` | `⤵ /merge-pr N § 1-7 → § 1` |
   | `/research-complete [ISSUE_ID]` | `⤵ /research-complete [ISSUE_ID] § 1-7 → § 1` |
   | Complete [Project]: /audit-issues project-order | Invoke workflow: `⤵ /audit-issues project-order § 1-9 → § 1` |
   | Activate project: /audit-issues project-order | Invoke workflow: `⤵ /audit-issues project-order § 1-9 → § 1` |
   | Plan cycle: /audit-issues → /cycle-plan | Invoke workflow: `⤵ /audit-issues project § 1-9`, then `⤵ /cycle-plan § 1-6 → § 1` |
   | `/parallel-check "Project"` | `⤵ /parallel-check "Project" § 1-11 → § 1` |
   | Start in parallel: [ISSUE_ID], ... | Ask user: `Start [ISSUE_ID] only` (→ § 1.4) \| `Launch parallel group` (capture `[ISSUE_IDS]` → § 1.4) |
   | Start [ISSUE_ID] | Capture issue ID → § 1.4 |

3. **Ask user** with recommended as first option. The `👉` line format is `👉 Label — reason`. Use text before `—` as the option label, text after `—` as the option description.

### 1.4 Handle Uncommitted Files

**Skip if** no uncommitted files warning in dashboard → § 2

1. **Present options**:
   - Stash first → `git stash -m "before [ISSUE_ID]"`
   - Commit first → `git add -A && git commit`
   - Commit and push first → `git add -A && git commit && git push`

2. **Ask user**

---

## 2. Prepare Issue

### 2.1 Parallel Group Iteration

**Skip if** single issue

**Pre-resolve**: For each issue in `[ISSUE_IDS]`, check `parent_id` (from `--with-bundle`). If set, replace with parent. Deduplicate. Update `[ISSUE_IDS]` with resolved top-level set.

For each issue in `[ISSUE_IDS]`, run § 2.2-2.5:
1. Per-issue exits (→ § 3) mean "done with this issue, continue loop"
2. Bundle completed (`pending_count == 0`) → remove from set, skip to next
3. § 2.4 auto-decompose (skip asking user)

After all issues processed → § 3.

### 2.2 Get Issue

1. **Fetch issue data** (from `/start [ISSUE_ID]` argument or § 1.3 selection):
   ```bash
   PARENT_ID=$($ISSUE_CLI cache issues get [ISSUE_ID] --with-bundle | jq -r '.parent_id // empty')
   ```

2. **If `PARENT_ID` set** (single-issue only; parallel groups pre-resolved in § 2.1): Use parent issue, restart § 2

### 2.3 Route by Bundle Status

1. **Check bundle status**:

   | Condition | Action |
   |-----------|--------|
   | `.children` empty | Single issue → § 3 |
   | `.pending_count == 0` | All done → mark parent complete, § 1 |
   | Otherwise | Bundle with pending work ↓ |

2. **If bundle**: If `.agent` empty, infer agent from component paths (project-configurable) or ask user

3. **Present bundle**:

   <output_format>

   📦 Bundle: [ISSUE_ID] | 📋 N remaining | 🐲 [AGENT]
   ↳ [SUB_ISSUE_1]: [TITLE]
   ↳ [SUB_ISSUE_2]: [TITLE] → blocks [SUB_ISSUE_4]
      ↳ [SUB_ISSUE_4]: [TITLE]  ← nested

   </output_format>

### 2.4 Validate Bundle Scope Coverage

1. **Parse parent description** for `## Requirements` section

2. **If parent has `## Sub-Issues`** or no `## Requirements` → § 3 (scope already decomposed)

3. **Present warning**: "Parent [ISSUE_ID] has implementation requirements not decomposed into sub-issues."

4. **Ask user**: `Decompose now` | `Proceed anyway`
   - **Decompose now** → § 2.5
   - **Proceed anyway** → § 3

### 2.5 Decompose Parent Scope

1. **Extract requirements** from parent `## Requirements`, tag each bullet with domain (project-configurable domain categories)

2. **Check existing children** (from § 2.3): Map each child's scope to parent requirements it covers. Mark covered requirements as satisfied.

3. **Create sub-issues for uncovered requirements only** — group by domain → one sub-issue per domain. Sub-issues must be in parent's project.

4. **Title format**: `[Domain verb]: [scope]`

5. **Set labels**: `agent:[TYPE]` per domain, appropriate stack labels

6. **Set blocking relations**: Read [agent-sequencing.md](agent-sequencing.md). Include relations to/from existing children.

7. **Rewrite parent description**: Replace `## Requirements` with `## Sub-Issues` listing ALL children (existing + new), add `## Context`, remove implementation-level detail.

8. **Set parent label** to `agent:multi` if 2+ domains across all children

9. **Re-present bundle** (§ 2.3 step 3) → § 3

---

## 3. Evaluate Research Requirements

### 3.1 Parallel Group Evaluation

**Skip if** single issue

1. Run § 3.2 per issue
2. Spawn § 3.3 consultations in parallel (one task per issue)
3. Present § 3.4 as combined table (skip asking user)
4. Research-needed → collect per-issue context (§ 3.2/3.3 fields) as `batch_issues`, run § 3.5 once, remove from `[ISSUE_IDS]`

### 3.2 Extract Research References

1. **Parse issue description(s)** for:
   - Research document paths → list as `[RESEARCH_PATHS]`
   - Decision references → list as `[DECISION_IDS]` — read from individual decision files
   - `$DECISIONS_CMD search --issue [ISSUE_ID]` → find linked decisions not explicitly referenced

2. **If bundled (from § 2.3)**: Parse parent + all sub-issue descriptions. Dedupe paths/refs.

### 3.3 Consult Development Agent(s)

1. **Determine consultation agent(s)**:
   - If parent has `agent:multi`: consult **all distinct `agent:[TYPE]` agents** from children (parallel agent launches, ephemeral)
   - Otherwise: consult the agent from `agent:[TYPE]` label

2. **Single agent — create consultation team** (skip for multi-agent):
   - Create agent team: `consult-[ISSUE_ID]`
   - Spawn agent: type=[AGENT_TYPE], name="consult-[AGENT_TYPE]", team="consult-[ISSUE_ID]", prompt=[Consultation Agent template from spawn-prompts.md]
   - Wait for agent to go idle

3. **Delegate consultation**: Follow exactly, fill placeholders, add nothing else. Omit lines/sections with empty placeholders.
   - **Single agent**: Send delegation message to "consult-[AGENT_TYPE]" with prompt below
   - **Multi-agent**: Embed prompt in agent launch per agent (parallel, ephemeral — no team)

   <delegation_format>
   CONSULTATION ONLY.

   Read issue: $ISSUE_CLI cache issues get [ISSUE_ID].

   [If bundle from § 2.3:]
   Sub-issues (tree):
   ↳ [SUB_ISSUE_1]: $ISSUE_CLI cache issues get [SUB_ISSUE_1]
   ↳ [SUB_ISSUE_2]: $ISSUE_CLI cache issues get [SUB_ISSUE_2]
      ↳ [SUB_ISSUE_3]: $ISSUE_CLI cache issues get [SUB_ISSUE_3]  ← nested child
   [End if]

   - Read: [RESEARCH_PATHS]
   - Run: $DECISIONS_CMD search --issue [ISSUE_ID] (find linked decisions)
   - Read: [DECISION_FILES]
   - Read: Relevant architecture docs
   - Read: Relevant in-project code

   Analyze implementation requirements. Check for these triggers:

   1. **Architectural gaps**: Missing infrastructure/patterns needed for implementation
   2. **Multiple approaches**: >1 valid design with trade-offs (latency/memory/complexity)
   3. **Cross-component integration**: New patterns bridging multiple subsystems
   4. **Performance-critical decisions**: Hot path choices affecting latency budgets
   5. **Unclear requirements**: Ambiguous behavior, edge cases, or constraints

   **Bias toward research**: If ANY trigger present OR uncertain → recommend research.

   Reply format (choose ONE):
   - `No research needed` — only if: trivial implementation, clear single approach, no gaps
   - `Existing sufficient` — only if: all approaches/trade-offs documented in research_paths or decision_refs
   - `Research: [QUESTIONS]` — architectural gaps, multiple approaches, performance trade-offs, unclear requirements
   </delegation_format>

4. **Capture for resume**: If single agent, store teammate name `consult-[AGENT_TYPE]` as `[CONSULTATION_AGENT_NAME]` for resume in § 3.5. If multi-agent, skip (research-issue uses fresh delegation for multi-domain).

### 3.4 Present Result and Confirm

1. **Present summary**:

   <output_format>

   | Field | Value |
   |-------|-------|
   | 🎯 Issue | [ISSUE_ID] - Title |
   | 📦 Bundle | N sub-issues (M nested) — if bundle |
   | | ↳ [SUB_ISSUE_1]: [TITLE] &#124; blocks: [SUB_ISSUE_2] |
   | | ↳ [SUB_ISSUE_2]: [TITLE] &#124; blocked by: [SUB_ISSUE_1] |
   | |    ↳ [SUB_ISSUE_4]: [TITLE]  ← nested |
   | 🐲 Agent | [AGENT] |
   | 📚 Research | [ref\|none] |
   | 💬 Consultation | [RESPONSE] |

   </output_format>

2. **Ask user**: `Continue` | `Create research issue`

3. **Route**: `Continue` → § 3.6 → § 4 | `Create research` → § 3.5

### 3.5 Create Research Issue(s) (if requested)

Invoke workflow: `⤵ /research-issue § 1-5 → § 1` with context:

**If single issue**:
- `topic`: from § 3.3 agent "Research:" response
- `questions`: from § 3.3 agent response
- `domains`: from children's `agent:[TYPE]` labels if `agent:multi`, else from issue's stack labels
- `project`: from `$ISSUE_CLI cache projects list --state started`
- `blocked_issue`: [ISSUE_ID] from § 2
- `type`: Targeted (1 domain) or Pervasive (2+ domains)
- `consultation_agent_name`: from § 3.3 step 4 if single agent (omit for multi-agent)
- `research_paths`: from § 3.2 [RESEARCH_PATHS]
- `decision_ids`: from § 3.2 [DECISION_IDS]

**If batch** (from § 3.1):
- `project`: from `$ISSUE_CLI cache projects list --state started`
- `batch_issues`: list of per-issue objects, each containing: `topic`, `questions`, `domains`, `blocked_issue`, `type`, `consultation_agent_name`, `research_paths`, `decision_ids` — all sourced from § 3.2/3.3 per issue

Research issues block their `blocked_issue`. After `/research-issue` completes, user executes research externally, then runs `/research-complete [ISSUE_ID]` to continue.

After `/research-issue` returns → § 3.6.

### 3.6 Clean Up Consultation Team

**Skip if** no consultation team (multi-agent path or no team created in § 3.3 step 2)

1. Send shutdown request to "consult-[AGENT_TYPE]"
2. Delete agent team: `consult-[ISSUE_ID]`

---

## 4. Delegate to Specialist Agent(s)

### 4.1 Check Issue Type

**Route**:
- **Parallel group** → § 4.4
- **Research issues** (`research` label) → § 4.2
- **All other issues** → § 4.3

### 4.2 Research Issues (human execution)

1. **Check assets**: research prompt file exists for [ISSUE_ID]

2. **If missing** → Invoke workflow: `⤵ /research-issue § 2 → § 4.2` (Prepare Assets only)

3. **If complete** → Present: `Research Ready: [ISSUE_ID] | Assets: ✓ | Run /research-complete after execution`

4. **User executes externally** → `/research-complete [ISSUE_ID]` → § 1

### 4.3 Create Worktree

Worktree creation is idempotent: existing worktrees are reused (rebased onto latest main), and PRs with matching branches are auto-detected. Fails on rebase conflicts.

1. **Run check**: `$WORKTREE_CLI check` — returns `{uncommitted, unpushed, unpushed_commits}`

2. **If uncommitted** → present uncommitted options (stash/commit/push)

3. **If unpushed** → Ask user: `Push unpushed commits to the default branch?` (show commits), then:
   ```bash
   DEFAULT_BRANCH=${WORKTREE_DEFAULT_BRANCH:-$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')}
   [ -n "$DEFAULT_BRANCH" ] || DEFAULT_BRANCH=main
   git push origin "$DEFAULT_BRANCH"
   ```

4. **Active work conflict scan**: Check for agent overlap with in-progress worktrees.
   ```bash
   $WORKTREE_CLI list
   ```
   For each active worktree issue: `$ISSUE_CLI cache issues get [WT_ISSUE] --format=compact` → compare `agent` with current issue.
   - **No overlap** → continue
   - **Same agent** → `scripts/parallel-groups needs-refresh [ISSUE_ID] [WT_ISSUE]`. If exit 1 (fresh, cached safe) → continue. Otherwise ask user: `Run /parallel-check [ISSUE_ID] [WT_ISSUE]` | `Continue anyway`. If check → `⤵ /parallel-check [ISSUE_ID] [WT_ISSUE] § 1-11 → § 4.3`. If conflicts verdict → warn with details, do not block.

5. **Create worktree**: `WT_PATH=$($WORKTREE_CLI create [ISSUE_ID])`

6. **Open terminal**: Detect environment:
   - **If `$TMUX` is set** → launch directly (no ask user): Run `scripts/open-terminal "$WT_PATH" --tmux [ISSUE_ID] --title [ISSUE_ID] --cmd "[LAUNCH_CMD]"`. Output: "Opened tmux window [ISSUE_ID] for worktree. This session is complete."
   - **Otherwise** → Ask user with: `Auto-open terminal` | `I'll open it myself`
     - **Auto**: Run `scripts/open-terminal "$WT_PATH" --title [ISSUE_ID] --cmd "[LAUNCH_CMD]"`. Output: "Opened worktree terminal for [ISSUE_ID]. This session is complete."
     - **Manual**: Output: "Worktree ready at `$WT_PATH`. Run the launch command in your new terminal. This session is complete."
   - **→ § 1** (restart dashboard in current session).

### 4.4 Launch Parallel Group

`[ISSUE_IDS]` contains only issues that passed § 2-3 (research-needed removed in § 3).

1. **If 0 issues in `[ISSUE_IDS]`** → § 1

2. **Detect terminal**: If `$TMUX` set → `FLAG=--tmux`. Otherwise → Ask user: `Auto-open terminal` | `I'll open them myself` → `FLAG=--auto` or skip launch.

4. **Ask user**: `Launch [N] issues` | `Select subset` | `Cancel`
   - **Launch**: `scripts/parallel-launch [ISSUE_IDS] $FLAG`
   - **Select subset**: Ask user with individual issues as options (multiSelect) → `scripts/parallel-launch [SELECTED_ISSUES] $FLAG`
   - **Cancel** → § 1

5. **→ § 1** (restart dashboard in current session).
