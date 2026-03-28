# Project Management — Full Compiled Document

**Version 1.1.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when managing
> project roadmaps and cycle planning. Humans may also find it useful,
> but guidance here is optimized for automation and consistency by
> AI-assisted workflows.

## Abstract

A methodology-based skill for technical program management (TPM). Covers roadmap planning, cycle planning, issue auditing, prioritization scoring, dependency management, label taxonomy, and project health tracking. All workflows return structured JSON recommendations — the orchestrator or user handles execution. Designed to be portable across any issue tracker that exposes a CLI (`$ISSUE_CLI`).

---

## Table of Contents

1. [Hierarchy](#hierarchy)
2. [Prioritization](#prioritization)
3. [Health Indicators](#health-indicators)
4. [Dependency Management](#dependency-management)
5. [Issue Management](#issue-management)
6. [Initiatives & Projects](#initiatives--projects)
7. [Label Management](#label-management)
8. [Workflows](#workflows)
9. [Schemas](#schemas)

---

## Hierarchy

```
Initiative → Project → Milestone → Issue → Sub-Issue
```

| Level | Duration | Example |
|-------|----------|---------|
| Initiative | Months | "Platform MVP" |
| Project | 2-6 weeks | "Phase 1: Foundation" |
| Milestone | Key checkpoint | "Data Pipeline Complete", "Alpha" |
| Issue | 1-5 days | "Implement message queue" |
| Sub-Issue | Breakdown | Child issue for parallel work |

**Max depth**: Initiative → Project → Issue → Sub-Issue (no deeper).

---

## Prioritization

### Scoring Formula

```
Score = (Critical Path x 3) + (Dependencies x 2) + (Risk x 2) + (Value x 1) - (Estimate x 0.5)
```

**Thresholds**: 8+ P1 | 5-7 P2 | 3-4 P3 | 0-2 P4

### Factor Definitions

#### Critical Path (x3)

| Score | Criteria |
|-------|----------|
| 3 | Blocks release |
| 2 | Blocks feature |
| 1 | Nice to have |
| 0 | Independent |

#### Dependencies (x2)

| Score | Criteria |
|-------|----------|
| 3 | 3+ issues blocked |
| 2 | 1-2 issues blocked |
| 1 | Soft dependency |
| 0 | None |

**Tip**: Query issue relations to count blocked issues:
```bash
$ISSUE_CLI cache issues list-relations [ISSUE_ID]
```

#### Risk (x2)

| Score | Criteria |
|-------|----------|
| 3 | Unknown technology |
| 2 | Complex integration |
| 1 | Straightforward |
| 0 | Trivial |

#### Value (x1)

| Score | Criteria |
|-------|----------|
| 3 | Core functionality |
| 2 | Enhancement |
| 1 | Polish |
| 0 | Optional |

#### Estimate (x0.5, subtracted)

| Score | Criteria |
|-------|----------|
| 3 | >1 week (5 pts) |
| 2 | 2-5 days (4 pts) |
| 1 | 1-2 days (2-3 pts) |
| 0 | <1 day (1 pt) |

### Example

**API Gateway Service**:
- Critical Path: 3 (blocks all downstream service communication)
- Dependencies: 3 (blocks auth, billing, notifications)
- Risk: 3 (first-time service mesh integration)
- Value: 3 (core infrastructure)
- Estimate: 2 (~3-4 days, 4 pts)

Score: (3x3) + (3x2) + (3x2) + (3x1) - (2x0.5) = **23** → P1

### Trade-offs

- **High risk + high value**: Spike first (1-2 days exploration)
- **Large issue blocking many**: Break into smaller deliverables
- **Multiple P1 items**: True dependencies first, then risk, then value

---

## Health Indicators

| Indicator | Green | Yellow | Red |
|-----------|-------|--------|-----|
| Blocked issues | 0 | 1-2 | 3+ |
| In Progress age | <3 days | 3-7 days | >7 days |
| Completion ratio (7d) | >0.8 | 0.5-0.8 | <0.5 |

---

## Dependency Management

### Core Rules

#### Same-Project Issue Blocking

**Issue blocking relations (`blocks`/`blocked-by`) must be within the same project.**

| Scenario | Correct Approach |
|----------|------------------|
| Issue A blocks Issue B, same project | Issue relation: `--blocked-by` |
| Issue in Project 1 blocks Issue in Project 2 | **Project** relation: Project 2 blocked-by Project 1 |
| Cross-project dependency needed | Move issue to same project OR use project-level blocking |

**Why**: Projects are the unit of planning and delivery. Cross-project issue blocking creates invisible dependencies that break project-level tracking.

**When you find cross-project issue blocking**:
1. **Check project ordering** — if Issue_A (Project_A) blocks Issue_B (Project_B) and Project_A already blocks Project_B at project level, the issue-level block aligns. Leave as redundant specificity.
2. **Relocate** — if the block contradicts project ordering or no project-level dep exists, move one issue to the correct project.
3. Use `related` only for informational links where no blocking dependency exists.
4. **Never infer project-level dependencies from individual issue relations.** Project deps come from project-order scope analysis (TPM audit), not bottom-up — which can create misleading or circular project dependencies.

#### Same-Project Parent-Child

**Sub-issues must be in the same project as their parent.**

| Scenario | Correct Approach |
|----------|------------------|
| Child belongs in parent's project | Normal: `--parent [ISSUE_ID]` |
| Child belongs in a different project | Detach (`--remove-parent`), move to correct project as standalone or re-parent in that project |

**Why**: Children in different projects break project-level tracking and make parents appear incomplete.

**When audit finds a cross-project parent-child**: Detach child, then either move it to parent's project or keep standalone with `related` link.

#### Blocking Level Rule

**Blocking relations go on bundle parents, not children.** A child issue must not block (or be blocked by) any issue outside its own parent bundle.

| Scenario | Correct Approach |
|----------|------------------|
| Child A (parent P1) should block Child B (parent P2) | P1 blocks P2; A `related` B |
| Child A (parent P1) should block Standalone B | P1 blocks B; A `related` B |
| Standalone A should block Child B (parent P2) | A blocks P2; A `related` B |
| Child A blocks sibling Child B (same parent) | Valid — intra-bundle dependency |

#### Remediation During Audit

Blocking relations are valuable — always preserve them by fixing the structural issue, not removing the relation.

| Violation | Fix |
|-----------|-----|
| Cross-project A blocks B | Move one issue to the other's project. Blocking relation stays valid. |
| Cross-bundle child A→B (same project) | Remove child relation, add parent-level blocking, `related` on children. |
| Cross-bundle child A→B (cross-project) | Move one bundle's parent to correct project, then lift to parent level. |
| Child→standalone A→B (same project) | Remove child relation, add parent blocks standalone, `related` on children. |

### Issue Dependencies vs Blocked Label

| Scenario | Use |
|----------|-----|
| Issue A blocked by Issue B (same project) | Issue relation: `--blocked-by` |
| Blocked by external factor (vendor, license, approval) | `blocked` label + comment |

### Relation Types

| Type | Meaning |
|------|---------|
| `blocks` | This issue blocks another from proceeding |
| `blocked-by` | Cannot proceed until another completes |
| `related` | Informational link, no blocking |
| `duplicate` | Issues are duplicates (for merging) |

### CLI Quick Reference

```bash
# This issue blocks another
$ISSUE_CLI issues add-relation [ISSUE_ID] --blocks [OTHER_ISSUE_ID]

# This issue is blocked by another
$ISSUE_CLI issues add-relation [ISSUE_ID] --blocked-by [BLOCKER_ID]

# View all relations
$ISSUE_CLI cache issues list-relations [ISSUE_ID]
```

### Project Dependencies

Use when one project must complete before another can start.

```bash
# This project depends on another
$ISSUE_CLI projects add-dependency [PROJECT_ID] --blocked-by [OTHER_PROJECT_ID]

# View dependencies
$ISSUE_CLI cache projects list-dependencies [PROJECT_ID]
```

**Visualization**: In issue tracker timeline view:
- **Blue line**: Dependency satisfied (blocker finishes before blocked starts)
- **Red line**: Dependency violated (blocked starts before blocker ends)

### Integration with Workflows

**Cycle Planning** — when pulling issues:
1. Query blocked issues: `$ISSUE_CLI cache issues list --label "blocked" --max`
2. Check issue relations: `$ISSUE_CLI cache issues list-relations [ISSUE_ID]`
3. Score higher on Dependencies factor for issues that block many others

**Roadmap Review**:
1. Check project dependencies: `$ISSUE_CLI cache projects list-dependencies [PROJECT_ID]`
2. Look for violated dependencies (red lines in timeline)
3. Adjust project dates or priorities accordingly

### Blocker Checklists

**Adding a Blocker**:
1. Identify what's blocking
2. **If tracked issue**: `$ISSUE_CLI issues add-relation [ISSUE_ID] --blocked-by [BLOCKER_ID]`
3. **If external**: Add `blocked` label + comment explaining blocker
4. Update issue state if needed

**Resolving a Blocker**:
1. Complete the blocking issue (relation auto-clears)
2. **If external**: Remove `blocked` label + add resolution comment
3. Move blocked issue back to active state

---

## Issue Management

### Quick Reference

| Field | Required | Format |
|-------|----------|--------|
| Title | Yes | `[Verb]: [outcome]` — e.g., "Implement: user authentication" |
| Labels | Yes | Agent (one) + Stack (one+) + Workflow/Classification (if applicable) |
| Estimate | Yes | 1-5 points (1=hours, 3=day, 5=week) |
| Project | Yes | Active project name or "Backlog" |
| Description | Yes | Context, acceptance criteria, dependencies |

### Check for Duplicates

```bash
$ISSUE_CLI cache issues list --state "Backlog,Todo,In Progress" --max | grep -iE "keyword"
```

| Finding | Action |
|---------|--------|
| Exact match | Don't create — add context to existing |
| Overlapping | Expand existing OR create with blocking relation |
| Outdated | Update existing issue |
| Superseded | Cancel with note: "Superseded by [ISSUE_ID]" |

When finding potential duplicates, expand existing issues rather than creating overlapping new ones.

**Right size?** 1-5 days. < 1 hour → just do it. > 1 week → break down or create project.

### Labels

See project label taxonomy for definitions. See project label application guide for when to apply.

| Category | Exclusive? | Rule |
|----------|------------|------|
| Agent | YES | Exactly ONE per issue |
| Platform | YES | ONE per issue |
| Stack | NO | All that apply |
| Workflow | NO | All applicable gates |
| Classification | NO | Multiple allowed |

### Estimates

| Points | Duration | Action |
|--------|----------|--------|
| 0 | < 1 hour | Just do it, no issue |
| 1 | Few hours | Single issue |
| 2 | Half day | Single issue |
| 3 | Full day | Single issue, sub-issues if complex |
| 4 | 2-3 days | Single issue, sub-issues if complex |
| 5 | Week+ | Break into smaller issues |
| — | > 1 week | Create project, break into phases |

### Sub-Issues

Use `--parent [ISSUE_ID]` when:
- Components form ONE deliverable (must all complete together)
- Breaking down a Medium issue into 2-4 steps
- Multiple agents need visibility into parts of one issue

**Parent scope rule**: Parent issues with children are coordination summaries only.
- Implementation requirements live in sub-issues, never in parents
- When sub-issues are created, transfer all parent requirements to children
- Parents retain: summary, research/decision refs, effort rollup, child cross-references

**Grouping**: Deliverable-based, not layer-based.
- Correct: "User Auth → Login UI, Token Service" (one feature)
- Incorrect: All backend issues grouped (layer grouping)

```bash
$ISSUE_CLI issues create --title "Parse input data" --parent [ISSUE_ID]
$ISSUE_CLI cache issues children [ISSUE_ID]              # Direct children only
$ISSUE_CLI cache issues children [ISSUE_ID] --recursive  # All descendants (3 levels, includes blocks/blocked_by)
```

**Max depth**: Initiative → Project → Issue → Sub-Issue (no deeper)

### Description Template

```markdown
## Summary
[What and why]

## Acceptance Criteria
- [ ] [Testable outcome 1]
- [ ] [Testable outcome 2]

## References
- **Research**: docs/research/[ISSUE_ID]/findings.md (if applicable)
- **Decision**: docs/decisions/[DECISION_ID]-[DESCRIPTOR].md (if applicable)
- Blocked by: [ISSUE_ID] (same project only)
- Blocks: [OTHER_ISSUE_ID] (same project only)
- Related: [ISSUE_ID] (cross-project OK)
```

### CLI Command

```bash
$ISSUE_CLI issues create \
  --title "Implement user authentication service" \
  --project "Phase 1: Foundation" \
  --labels "backend,agent:[TYPE],critical-path" \
  --estimate 3 \
  --description "## Summary
Auth service for user login and session management.

## Acceptance Criteria
- [ ] Login endpoint returns JWT
- [ ] Token refresh works
- [ ] Unit tests pass"
```

### Cancellation

```bash
$ISSUE_CLI comments create [ISSUE_ID] --body "CANCELED: [REASON]"
$ISSUE_CLI issues update [ISSUE_ID] --state "Canceled"
```

Valid reasons: Requirement changed, superseded by [OTHER_ISSUE_ID], no longer needed.

### Priority

| Score | Priority |
|-------|----------|
| 8+ | P1 (Urgent) |
| 5-7 | P2 (High) |
| 3-4 | P3 (Normal) |
| 0-2 | P4 (Low) |

### Common Mistakes

| Mistake | Fix |
|---------|-----|
| No project | Always add `--project` |
| No/multiple agent labels | Exactly ONE `agent:*` |
| Missing stack | Always add stack(s) |
| Missing estimate | Always add `--estimate 1-5` |
| Vague title | Use `[Verb] [outcome]` |
| No acceptance criteria | Add testable outcomes |
| Too large (> 5 days) | Break into smaller issues |
| Line numbers in location | Use file path + function/struct name — line numbers go stale |

---

## Initiatives & Projects

### When to Create

| Create | When | Don't Create |
|--------|------|--------------|
| Initiative | Multi-month effort, company-level objective | Single project scope |
| Project | 2+ weeks, multiple related issues, milestone tracking needed | Single issue, ongoing maintenance, unclear scope |
| Milestone | Key checkpoint within project (Alpha, Beta, Release) | Don't over-milestone; 2-4 per project typical |

**Spike**: Time-boxed exploration project (1-2 weeks), e.g., "Spike: API Gateway Options"

### Project Lifecycle

```
Backlog → Started → [Paused] → Completed → Archived
```

| State | When to Use |
|-------|-------------|
| Backlog | Future work, not yet prioritized |
| Started | Active development |
| Paused | Temporarily blocked (document reason) |
| Completed | All issues done, ready for retro |
| Archived | After retrospective complete |

### Naming Convention

Format: `[Prefix] [Clear name]`

| Prefix | Use |
|--------|-----|
| `Initial` | Foundational/first implementation |
| `Phase N:` | Part of larger initiative |
| `Spike:` | Time-boxed exploration |

Examples: "Initial Auth Service", "Phase 2: API Integration", "Spike: GraphQL vs REST"

### Required Before Creating

1. **Scope**: What's included and excluded
2. **Success criteria**: How do we know it's done?
3. **Timeline**: Target completion (cycle or date)
4. **Dependencies**: What must complete first?

### CLI Commands

```bash
# Initiative
$ISSUE_CLI initiatives create --name "[NAME]" \
  --description "[DESCRIPTION]" \
  --content "[CONTENT]"

# Project
$ISSUE_CLI projects create --name "[NAME]" \
  --priority 2 \
  --description "[DESCRIPTION]" \
  --content "[CONTENT]"

# Milestone
$ISSUE_CLI milestones create --project "[PROJECT_NAME]" --name "[NAME]" --target-date [TARGET_DATE]
```

**Two-field pattern**: `--description` (255 char subtitle) + `--content` (markdown body, no limit)

### Breaking Down Projects

1. **Identify phases** → milestones (2-4 per project)
2. **Create issues per phase** → 1-5 day chunks
3. **Establish dependencies** → blocking relations
4. **Assign agents** → one agent per issue

**Example breakdown** (Initial Auth Service):
- Phase 1 (Core): `agent:[TYPE]` defines auth models → `agent:[TYPE]` builds API endpoints
- Phase 2 (Integration): `agent:[TYPE]` connects to identity provider; add `needs-perf-test` only when auth changes affect shared runtime paths, not admin-only features
- Phase 3 (Validation): integration tests + `needs-security-audit`

### State Transitions

**Starting**:
```bash
$ISSUE_CLI projects update [PROJECT_ID] --state started
$ISSUE_CLI issues update [ISSUE_ID] --state "In Progress"  # First issue
```

**Pausing**:
1. Document reason in comment
2. Set state to Paused
3. Reassign resources to other work

**Completing**:
1. Verify no Todo/In Progress issues remain
2. Set state to Completed
3. Archive project

### Checklist: New Project

Before:
- [ ] Scope clearly defined
- [ ] Success criteria testable
- [ ] Timeline estimated
- [ ] Dependencies identified
- [ ] Broken into 1-5 day issues

After:
- [ ] Project created in issue tracker
- [ ] Initial issues created with labels
- [ ] Dependencies linked
- [ ] First issues moved to In Progress

---

## Label Management

### Exclusivity Rules

See project label taxonomy for full taxonomy and colors.

**Key rule**: Labels in a parent group (Agent, Platform) are exclusive — only ONE per issue. Labels without a parent (Stack, Workflow, Classification) allow multiples.

**"labelIds not exclusive child labels" error** = You tried to use multiple labels from Agent or Platform group.

### When to Create Labels

**Create when**:
- New agent added (requires agent definition first)
- New stack component introduced
- New workflow state needed

**Do NOT create when**:
- Existing label covers the use case
- One-off categorization (use description instead)
- No clear owner or purpose defined

### Label Ownership

| Label Type | Owner | Approval | Notes |
|------------|-------|----------|-------|
| `agent:*` | tpm | Yes | Requires project agent definition |
| Stack | tpm | Yes | Architectural change |
| Workflow | tpm | No | Operational |
| Classification | tpm | No | Operational |
| Platform | tpm | Yes | Architectural change |

### Creating Agent Labels

Agent labels are special — MUST have agent definition AND parent group.

1. **Create the agent definition** in project agent definitions
2. **Update** project label taxonomy
3. **tpm** creates label:
   ```bash
   $ISSUE_CLI labels create --name "agent:[NAME]" --color "#9C27B0" --parent "Agent"
   ```

**TPM should NOT create `agent:*` labels unprompted** — only after the agent definition and taxonomy entry exist.

### Creating Other Labels

```bash
# Workflow labels (no parent - independent)
$ISSUE_CLI labels create --name "needs-[ACTION]" --color "#757575"

# Classification labels (no parent - independent)
$ISSUE_CLI labels create --name "[TYPE]" --color "#E53935"

# Stack labels (requires review)
$ISSUE_CLI labels create --name "[STACK_NAME]" --color "#FF6B35"
```

After creating, update project label taxonomy.

### Label Lifecycle

**Deprecating**:
1. Remove from active issues (reassign)
2. Archive in issue tracker (don't delete — preserves history)
3. Mark deprecated in taxonomy

**Renaming** — avoid renaming (creates confusion). Instead:
1. Create new label with correct name
2. Migrate issues from old to new
3. Archive old label

### Checklist: New Label

Before:
- [ ] No existing label covers this
- [ ] Determined parent group (exclusive) or none (independent)
- [ ] Color consistent with category
- [ ] Approval obtained if required

After:
- [ ] Label created in issue tracker
- [ ] Taxonomy updated
- [ ] Announced in handoff/comment

---

## Workflows

TPM workflows return JSON recommendations only. The orchestrator or user handles interaction and execution.

| Workflow | Purpose | File |
|----------|---------|------|
| Cycle Planning | Analyze backlog, compute architecture order, select issues for cycle | [tpm-cycle-plan.md](workflows/tpm-cycle-plan.md) |
| Roadmap Planning | Cross-project analysis, architecture gaps, issue organization | [tpm-roadmap-plan.md](workflows/tpm-roadmap-plan.md) |
| Issue/Project Audit | Audit issues for relations, hierarchy, duplicates, configuration | [tpm-audit.md](workflows/tpm-audit.md) |
| Project Order Audit | Analyze project dependencies, verify ordering, state transitions | [tpm-audit-project-order.md](workflows/tpm-audit-project-order.md) |

---

## Schemas

Output schemas define the JSON structure returned by each workflow.

| Schema | Purpose | File |
|--------|---------|------|
| Cycle Plan Output | Planned work, velocity, capacity, health, actions | [cycle-plan-output.md](schemas/cycle-plan-output.md) |
| Roadmap Plan Output | Organized issues, gaps, duplicates, project placement | [roadmap-plan-output.md](schemas/roadmap-plan-output.md) |
| Audit Output | Findings arrays (relations, priorities, agents, hierarchy) | [audit-output.md](schemas/audit-output.md) |
| Project Order Output | Recommended order, reorders, state transitions | [audit-project-order-output.md](schemas/audit-project-order-output.md) |
