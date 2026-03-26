# Issues Reference

**Follow field requirements exactly. Check duplicates before creating.**

## Quick Reference

| Field | Required | Format |
|-------|----------|--------|
| Title | Yes | `[Verb]: [outcome]` — e.g., "Implement: user authentication" |
| Labels | Yes | Agent (one) + Stack (one+) + Workflow/Classification (if applicable) |
| Estimate | Yes | 1-5 points (1=hours, 3=day, 5=week) |
| Project | Yes | Active project name or "Backlog" |
| Description | Yes | Context, acceptance criteria, dependencies |

## Check for Duplicates

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

## Labels

See project label taxonomy for definitions. See project label application guide for when to apply.

| Category | Exclusive? | Rule |
|----------|------------|------|
| Agent | YES | Exactly ONE per issue |
| Platform | YES | ONE per issue |
| Stack | NO | All that apply |
| Workflow | NO | All applicable gates |
| Classification | NO | Multiple allowed |

## Estimates

| Points | Duration | Action |
|--------|----------|--------|
| 0 | < 1 hour | Just do it, no issue |
| 1 | Few hours | Single issue |
| 2 | Half day | Single issue |
| 3 | Full day | Single issue, sub-issues if complex |
| 4 | 2-3 days | Single issue, sub-issues if complex |
| 5 | Week+ | Break into smaller issues |
| — | > 1 week | Create project, break into phases |

## Sub-Issues

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

## Description Template

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

**For performance issues**, include:
```markdown
**Performance Target**: <target metric>
**Current Measurement**: <current value> (or "TBD - baseline needed")
**Optimization Approach**: [Strategy]
```

## CLI Command

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

## Cancellation

```bash
$ISSUE_CLI comments create [ISSUE_ID] --body "CANCELED: [REASON]"
$ISSUE_CLI issues update [ISSUE_ID] --state "Canceled"
```

Valid reasons: Requirement changed, superseded by [OTHER_ISSUE_ID], no longer needed.

## Priority

See [prioritization.md](prioritization.md) for scoring formula and factor definitions.

| Score | Priority |
|-------|----------|
| 8+ | P1 (Urgent) |
| 5-7 | P2 (High) |
| 3-4 | P3 (Normal) |
| 0-2 | P4 (Low) |

## Common Mistakes

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
