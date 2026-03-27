# Orchestration

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when coordinating
> multi-agent sessions, delegating work, and managing workflow state.
> Humans may also find it useful, but guidance here is optimized for
> automation and consistency by AI-assisted workflows.

---

## Abstract

Multi-agent session coordination with delegation patterns, workflow state management, and review pipelines. Covers task pre-creation, spawn prompt design, delegation messaging, agent lifecycle management, compaction resilience, review finding schemas, and parallel work safety analysis. Designed to work across any AI coding harness that supports agent spawning and task management.

## Table of Contents

1. [Workflow Execution](#1-workflow-execution-critical)
2. [Delegation](#2-delegation-critical)
3. [Agent Lifecycle](#3-agent-lifecycle-high)
4. [State Management](#4-state-management-high)
5. [Coordination](#5-coordination-medium)
6. [Review Pipeline](#6-review-pipeline-medium)
7. [Scripts](#7-scripts)
8. [Schemas](#8-schemas)
9. [Known Platform Considerations](#9-known-platform-considerations)
10. [Templates](#10-templates)

---

## 1. Workflow Execution (CRITICAL)

### Pre-Create All Workflow Tasks

Run `workflow-sections` to extract section headers from workflow markdown, then create all tasks before executing any section. The task list serves as a durable position anchor that survives context compaction (conversation history is discarded but task state persists).

Two-step ordering: extract section data first, then create tasks. For delegation workflows with a team context, the team must exist before task creation or tasks go to the wrong scope and are invisible to teammates.

### Sequential Section Execution

Process sections sequentially: mark in-progress, execute all sub-sections within the section, mark completed, then proceed to next. Never create tasks for sub-sections — they are steps within the parent task, not separate tasks. Never mark a parent section complete before all sub-sections are executed.

Never skip steps because the outcome seems predictable, or rationalize skipping based on change scope ("test-only", "small", "only N items", "already reviewed"). The workflow text is the decision authority, not the agent's assessment.

### Skip-If Condition Evaluation

When a section starts with "Skip if [condition]", evaluate the condition literally. If true, append "(SKIPPED)" to the task subject and mark completed. The workflow decides what to skip, not the agent. This makes skipped steps visible to the orchestrator and to post-compaction recovery via the task list.

### Nested Workflow Invocation

Nested workflows (marked with `⤵`) must be invoked through the harness's workflow invocation mechanism — never inlined or substituted with ad-hoc commands. Pre-create tasks using `workflow-sections --subflow "/command"`. If the marker includes a return point (`→ § X`), pass `--return "§ X"` so task descriptions contain the return point for compaction resilience.

---

## 2. Delegation (CRITICAL)

### Delegation Patterns

| Pattern | When | Flow |
|---------|------|------|
| Spawn + message | Fresh agents (dev, QA, review) | Create tasks → spawn (behavioral prompt) → send delegation message (task prefix) | start-worktree, review-pr, cycle-plan |
| Message only | Re-delegation to existing agents | Create tasks → send delegation message (task prefix) | dev-fix, ci-fix, review-pr-comments |
| Self-create | Agent without team context | Embed `workflow-sections` in delegation prompt | audit-issues (TPM agent) |
| Consultation | One-off sub-agent | Full instructions in prompt, no task machinery | roadmap-plan, research-issue, start § 3 |

### Create Tasks Before Spawning Agents

Always create tasks before spawning an agent. The pattern is: create tasks (no owner assignment) → spawn agent (behavioral prompt only, goes idle) → send delegation message (includes task prefix). The agent wakes on the delegation message, checks the task list, and finds PENDING tasks matching the prefix.

For re-delegation to an existing agent: create new tasks → send delegation message. The agent wakes, finds NEW PENDING tasks by prefix (prior round's tasks are already completed — not matched).

### Message Gate Pattern

Every agent spawn prompt must include a mandatory message gate: for EVERY message, the agent scans for a `Task prefix:` line. No prefix found → go idle immediately. This positive gatekeeper (check for X before acting) is more robust than negative filtering (ignore Y).

Without it, agents process task-list notifications and other non-delegation messages, producing incorrect work. The delegation arrives separately via a message containing a `Task prefix:` line. The agent extracts the prefix, checks the task list, and finds PENDING tasks whose subject starts with that prefix.

The spawn prompt template must be copied verbatim (fill placeholders only). Paraphrasing has historically dropped the gate instruction.

### Task Prefix Hierarchy

| Context | Emoji | Example Subject | `taskPrefix` Value |
|---------|-------|-----------------|-------------------|
| Top-level workflow | (none) | `§ 1: Display Dashboard` | (none) |
| Nested sub-workflow (⤵) | `⤵` | `⏤⤵ /skill § 1: Identify Failures` | `⏤⤵ /skill` |
| Dev delegation | `🐲` | `⏤⏤🐲 dev-implement § 4: Implement` | `⏤⏤🐲 dev-implement` |
| TPM delegation | `🤹‍♂️` | `⏤⏤🤹‍♂️ tpm-roadmap § 1: Analyze` | `⏤⏤🤹‍♂️ tpm-roadmap` |
| Review delegation | `🐞` | `⏤⏤🐞 security-review § 1: Review` | `⏤⏤🐞 security-review` |
| QA delegation | `🪲` | `⏤⏤🪲 qa-review § 1: Set Up` | `⏤⏤🪲 qa-review` |
| Tracking (inline) | `🐲` | `⏤⏤🐲 backend: Fix CI lint` | `⏤⏤🐲 backend` |

The `taskPrefix` from `workflow-sections` JSON output must be used exactly in delegation messages — never hand-written. Agents filter by prefix + PENDING status.

### Task Layers

The shared task list contains three visually distinct layers:

```
§ N: [Title]                → orchestrator's own workflow steps
⏤⤵ /cmd § N: [Title]      → nested sub-workflows
⏤⏤🐲/🤹‍♂️/🐞/🪲 workflow § N: [Title] → agent tasks
```

Agents filter by their prefix + PENDING status — they never touch orchestrator or sub-workflow tasks.

### No Duplicate Agent Spawns

Never spawn a fresh agent when an existing one of the same type is alive — message it instead. Before creating agent tasks, check the task list for PENDING tasks with the same prefix. If an agent appears stuck: confirm the stall using session-level evidence per [Wait for Agent Return](#wait-for-agent-return-before-acting) (quiet ≠ stalled). Only after confirmed: shut down → respawn → re-create tasks → re-delegate.

### Single Return Message

The LAST task in an agent's assignment handles the return message. The spawn prompt must explicitly state "Do NOT send additional messages after it."

### Spawn Prompt Design Principles

Spawn prompts are harness-specific — each platform has its own agent spawning mechanism. The patterns below are universal regardless of harness:

**Message gate**: Every agent must check incoming messages for a delegation marker (e.g., `Task prefix:` line) before acting. No marker → wait for delegation. This prevents agents from processing system notifications as work directives.

**PENDING-only filtering**: Agents only process unclaimed tasks matching their prefix. Completed tasks from prior rounds or other agents are ignored.

**Task ID ordering**: Process tasks in creation order (lowest ID first). Tasks are created in section order so IDs naturally match workflow progression.

**Skip-if handling**: Evaluate conditions literally, mark skipped tasks visibly so the orchestrator can track what was skipped.

**Single return**: The last task sends the completion message. No additional messages after — prevents double-wakeups.

**Verbatim templates**: Spawn prompts must be copied exactly (fill placeholders only). LLM paraphrasing drops critical behavioral instructions.

Full verbatim templates are in `workflows/spawn-prompts.md` — copy exactly, fill placeholders only.

---

## 3. Agent Lifecycle (HIGH)

### Lifecycle Stages

```
1. TASKS        Orchestrator creates tasks via workflow-sections (no owner assignment)
2. SPAWN        Spawn agent with behavioral prompt → agent goes idle
3. DELEGATE     Send delegation message with task prefix
4. WORK         Agent wakes, finds PENDING tasks by prefix, sets in-progress, processes in ID order
5. RETURN       Last workflow section sends completion message to orchestrator
6. IDLE/REDEL   Agent goes idle — may receive new tasks + message for fix cycles
7. SHUTDOWN     Orchestrator sends shutdown request when all work complete
```

### Dev Agent Persistence

Dev agents persist for the entire session — never shut down except at finalization. After completing initial work, they may be re-delegated for review fix items, QA fix items, comment fixes, or CI failure fixes. Each re-delegation: create new tasks → send message with delegation.

### Review Agent Lifecycle Management

Review agents persist across fix → re-review cycles within the review workflow:
- Spawn if review agents state is empty, skip if already alive
- After fixes, selectively shut down non-reporting agents for low-risk changes; keep all alive if risk flags present
- Full shutdown when review passes, clear review agents state

QA agents spawn and shut down per-agent (sequential execution, less benefit from context preservation).

### Wait for Agent Return Before Acting

After delegation, wait for the agent's return message. Idle notifications are normal — agents go idle between turns while working. On idle notification, check the task list:
- Any in-progress → **do nothing, go idle** (agent is working)
- All completed → done, proceed
- All pending (none claimed) → re-send delegation ONCE, wait one full agent turn. If still all pending, respawn.

Never re-send or intervene while any task is in-progress.

**Quiet ≠ stalled.** Implementation agents routinely spend 5-15 minutes reading docs, planning, and analyzing code before producing any file changes. An agent making read/search calls with zero writes is in research/planning — real progress, not a stall.

**Minimum quiet window**: 10 minutes from delegation before entering escalation. No "simple task" exceptions.

**Invalid stall signals** (never sufficient alone or combined): return-message timeout, clean `git status`/`git diff`/`git log`, no modified files. These observe worktree state only — agents may be reading, planning, or working in a context not yet reflected in file changes.

**Stall confirmation required before shutdown.** Verify the agent is inactive using session-level evidence beyond worktree state:
- **Task-based harnesses** (e.g., Claude Code): task status unchanged across multiple idle cycles — no task claimed or progressed
- **Session-file harnesses** (e.g., Codex, OpenCode): no new entries in session log for 10+ minutes; check last timestamp and tool-call count in the session JSONL
- **Process-level**: agent process exited or consuming zero CPU for extended period

**Escalation sequence** (only after quiet window elapsed AND stall confirmed):
1. Re-message once with clarification specifying the missing step.
2. Wait 5 min. Re-check activity signals. New activity → go idle.
3. Still inactive → shut down → respawn → re-create tasks → re-delegate.

### Orchestrator Never Fixes Code

The orchestrator never edits or writes code in the worktree. Always delegate to the domain agent. If the agent appears stuck, confirm the stall using session-level evidence (see above — quiet ≠ stalled), then follow re-delegation: shut down → respawn → re-create tasks → re-delegate. The orchestrator may run read-only commands and invoke scripts.

---

## 4. State Management (HIGH)

### Durable Workflow State Files

Use workflow state files for any data that must survive context compaction: issue tracking, sub-issues, agent persistence, cycle counts, fix/escalation tracking, and audit trails. Use the `workflow-state` CLI for atomic reads/writes with flock-based locking.

State file location: `$ORCH_STATE_DIR/workflow-state-[ID].json` (default: `tmp/`)

### Compaction Recovery Protocol

After context compaction, conversation history is discarded but external state persists:
1. Check the task list — find last completed task, resume from next
2. Read workflow state file for persistent data (team name, cycles, agent IDs)
3. If team-based: re-read team config from disk to restore member list
4. Re-send delegation to existing agents. If no response after one idle cycle, only then respawn.

Never repeat completed actions.

---

## 5. Coordination (MEDIUM)

### Agent Sequencing by Data Dependency

When multiple agents work on related issues, determine blocking relations from data dependencies:
1. Infer agent from label or component path
2. Identify candidate pairs from sequential requirements
3. Confirm with Creates ↔ Consumes analysis — no data flow = no blocking
4. Set blocking relations on parent issues, not children, when bundled

Default sequential requirements:
- Backend → Frontend (if data dependency — UI needs backend types/APIs first)
- `*` → Generalist (runs last — may reference changes from any domain)

### Bundled Issue Task Structure

When a parent issue has sub-issues assigned to the same agent, create one composite task per sub-issue covering all relevant sections, not one task per section.

**Why per-sub-issue tasks**: Early designs used one task per section (§ 4, § 5, § 6...). Problem: first sub-issue completed all section tasks, second sub-issue had no tasks to track. The task system doesn't support looping. Fix: one composite task per sub-issue covering §§ 4-10. The spawn prompt's "Multi-section tasks" instruction tells agents to execute all referenced sections, then mark the single task complete.

```
§ 1: Environment Setup          (one task)
§ 2: Activate Issue              (one task)
§ 3: Block Issue                 (one task, usually SKIPPED)
§ 4-10: PROJ-001 — First sub    (composite — all sections for this sub-issue)
§ 4-10: PROJ-002 — Second sub   (composite — all sections for this sub-issue)
§ 11: Return to Orchestrator     (one task)
```

### Multi-Agent Bundles

When sub-issues span domains:
- Groups processed sequentially per agent-sequencing rules
- Orchestrator collects handoff notes between groups
- All dev agents persist until shutdown (enables cross-domain fix cycles)

### Parallel Work Safety Analysis

Before running issues in parallel, verify safety across five dimensions:
1. **Dependency resolution** — direct blocks/blockedBy, shared blockers
2. **Agent overlap** — same agent on multiple issues risks file conflicts
3. **Code scope** — analyze file paths, modules, type/value flows
4. **Build config** — manifest file changes create hard separations
5. **Active work** — check for existing worktrees and open PRs

Grouping constraints: limit concurrent issues, limit same-agent per group, manifest conflicts as hard separations.

---

## 6. Review Pipeline (MEDIUM)

### Review Finding Schema

All review/QA agents output JSON:

```json
{
  "agent": "agent-name",
  "timestamp": "2026-01-14T03:30:00Z",
  "verdict": "pass|action_required",
  "summary": "1-2 sentence summary",
  "blockers": [{
    "id": 1, "title": "Title (5-10 words)",
    "location": "src/file.rs (`function_name`)",
    "description": "What the issue is",
    "recommendation": "How to fix it",
    "priority": 1, "estimate": 2
  }],
  "suggestions": [{
    "id": 1, "title": "Title (5-10 words)",
    "location": "src/file.rs (`function_name`)",
    "description": "What could be improved",
    "recommendation": "How to improve it",
    "priority": 3, "estimate": 2,
    "category": "fix|issue"
  }],
  "questions": [{
    "id": 1, "location": "src/file.rs",
    "question": "Why is this async?",
    "draft_response": "Because...",
    "source": "@reviewer",
    "source_id": "PRRT_kwDO...",
    "source_type": "inline"
  }],
  "qa_metadata": {}
}
```

Verdict: `action_required` if blockers exist, `pass` otherwise. Location uses function/struct names, never line numbers.

### Recommendation Categorization

For each review suggestion, evaluate in order:

1. **Actionable?** Specific deliverable, observable impact, bounded scope. Vague → omit.
2. **Related?** Semantic relevance to issue/changes. Doc updates for changed code → always fix. Unrelated → issue.
3. **Size?** Small → fix. Needs delegation/tracking → issue.

Category signals:

| Signal | Category |
|--------|----------|
| Small, quick to apply | `fix` |
| Doc/reference updates for changed code | `fix` — always |
| Needs tracking, delegation, or history | `issue` |
| Architectural change, cross-component | `issue` |
| Test coverage (existing test) | `fix` |
| Test coverage (new suite) | `issue` |
| Error handling gaps | `issue` |
| Security vulnerabilities | `fix` if quick, else `issue` — never skip |

"Low priority" ≠ omit. Track if actionable.

### Issue Audit Pipeline

Review agents output JSON → orchestrator collects → transforms `category=issue` suggestions into audit input → TPM agent creates tracked issues. Sources: suggestions, escalated blockers, planned items, discovered work.

Audit item requires: index, title, location (no line numbers), description (2-3 sentences), recommendation (bullet-list), priority, estimate, found_by, origin (suggestion/escalated/planned/discovered). Dependency fields populated when implementation order is known.

---

## 7. Scripts

### workflow-sections

Parse `## N.` headers from workflow markdown → JSON array for task creation.

```bash
scripts/workflow-sections workflow.md                                    # Top-level
scripts/workflow-sections workflow.md --subflow "/ci-fix"                # Sub-workflow → ⤵ /ci-fix § N
scripts/workflow-sections workflow.md --agent "dev-fix" --emoji "🐲"     # Agent → 🐲 dev-fix § N
scripts/workflow-sections workflow.md --subflow "/cmd" --return "§ 5"    # With return point
```

Output JSON fields: `subject`, `description`, `activeForm`, `taskPrefix`, `metadata`.

### workflow-state

Read/write persistent state files with atomic flock-based locking.

```bash
scripts/workflow-state init PROJ-123 --agent backend --worktree /tmp/wt
scripts/workflow-state get PROJ-123 .cycles
scripts/workflow-state increment PROJ-123 cycles
scripts/workflow-state append PROJ-123 json_paths "review.json"
scripts/workflow-state append PROJ-123 fixed_items '{"description":"Fix","commit":"abc"}'
scripts/workflow-state set PROJ-123 pr_review_baseline '{"last_ts":"2026-01-28","last_threads":2}'
```

Environment: `ORCH_STATE_DIR` overrides state directory (default: `tmp`).

---

## 8. Schemas

### Workflow State

Persistent state across workflow steps. See `schemas/workflow-state.md` for full field definitions.

Key fields: `issue_id`, `sub_issues`, `agent`, `worktree`, `branch`, `team_name`, `child_sessions`, `review_agents`, `cycles`, `json_paths`, `fixed_items`, `escalated_items`, `audit_issues_created`.

### Review Finding

Review/QA agent output format. See `schemas/review-finding.md` for full field definitions.

Key structures: `blockers[]` (block merge), `suggestions[]` (fix or issue), `questions[]` (PR triage). Each item: id, title, location, description, recommendation, priority, estimate.

### Audit Issues Input

Input for issue audit workflows. See `schemas/audit-issues-input.md` for full field definitions.

Sources: review suggestions, escalated blockers, discovered work, roadmap items. Each item includes dependency tracking (blocks_items, blocked_by_items).

### Roadmap Plan Input

Input for roadmap planning. See `schemas/roadmap-plan-input.md` for full field definitions.

Proposed issues with dependency tracking, breaking changes, and doc update requirements.

## 9. Known Platform Considerations

Agent team implementations vary by harness. These are common behaviors to work around:

| Behavior | Impact | Mitigation |
|----------|--------|------------|
| Task status changes generate notifications | N trailing notifications after agent goes idle | Agent recognizes completed tasks, goes idle immediately |
| Idle notifications wake orchestrator on every agent turn boundary | Orchestrator may intervene prematurely | Rule: never intervene while any task is in-progress |
| Worktree appears clean during agent research/planning phase | Orchestrator misreads quiet-but-active agent as stalled | Rule: check session-level activity (task status changes, session log entries, process liveness) — not worktree state — before declaring stall. 10 min minimum quiet window. |
| Orchestrator loses team awareness after context compaction | Can't message alive teammates | Re-read team config from disk, re-send delegation, only respawn if no response |
| No session resumption for teammates after restart | Teammates lost on explicit session restart | Respawn + re-delegate pending tasks |
| Agent teams use more tokens than sub-agents | Higher cost per persistent agent | Consultation pattern uses sub-agents for one-off tasks |
| Spawn prompt paraphrasing | LLM drops critical instructions when paraphrasing | Copy templates verbatim, fill placeholders only |
| Task creation notifications | Idle agents wake prematurely on task creation | Create tasks before spawning, or create within existing team context |

**Sub-agent alternative**: If agent teams prove too costly, Patterns 1-3 can be converted to one-shot sub-agent calls. Trade-offs: no re-delegation (each call is one-shot), no inter-agent messaging, but zero notification overhead and lower cost. Pattern 4 already uses sub-agents.

## 10. Templates

Issue creation templates for structured output:

- `templates/issue-description-template.md` — Standard markdown for issue descriptions (research refs, requirements, context)
- `templates/parent-issue-template.md` — Parent/bundle issues with sub-issue coordination

## Dependencies

- `jq` for JSON processing
- `bash` 4+
- `flock` (util-linux) for atomic state updates
