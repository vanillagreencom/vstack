# Initiatives & Projects Reference

## When to Create

| Create | When | Don't Create |
|--------|------|--------------|
| Initiative | Multi-month effort, company-level objective | Single project scope |
| Project | 2+ weeks, multiple related issues, milestone tracking needed | Single issue, ongoing maintenance, unclear scope |
| Milestone | Key checkpoint within project (Alpha, Beta, Release) | Don't over-milestone; 2-4 per project typical |

## Hierarchy

See `project-management` skill (SKILL.md) for hierarchy overview.

```
Initiative → Project → Milestone → Issue → Sub-Issue
```

**Spike**: Time-boxed exploration project (1-2 weeks), e.g., "Spike: API Gateway Options"

## Project Lifecycle

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

## Naming Convention

Format: `[Prefix] [Clear name]`

| Prefix | Use |
|--------|-----|
| `Initial` | Foundational/first implementation |
| `Phase N:` | Part of larger initiative |
| `Spike:` | Time-boxed exploration |

Examples: "Initial Auth Service", "Phase 2: API Integration", "Spike: GraphQL vs REST"

## Creating

### Required Before Creating

1. **Scope**: What's included and excluded
2. **Success criteria**: How do we know it's done?
3. **Timeline**: Target completion (cycle or date)
4. **Dependencies**: What must complete first?

### CLI Commands

See issue tracker CLI skill for full CLI reference.

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

## Breaking Down Projects

1. **Identify phases** → milestones (2-4 per project)
2. **Create issues per phase** → 1-5 day chunks
3. **Establish dependencies** → blocking relations
4. **Assign agents** → one agent per issue

**Example breakdown** (Initial Auth Service):
- Phase 1 (Core): `agent:[TYPE]` defines auth models → `agent:[TYPE]` builds API endpoints
- Phase 2 (Integration): `agent:[TYPE]` connects to identity provider; add `needs-perf-test` only when auth changes affect shared runtime paths, not admin-only features
- Phase 3 (Validation): integration tests + `needs-security-audit`

## State Transitions

### Starting

```bash
$ISSUE_CLI projects update [PROJECT_ID] --state started
$ISSUE_CLI issues update [ISSUE_ID] --state "In Progress"  # First issue
```

### Pausing

1. Document reason in comment
2. Set state to Paused
3. Reassign resources to other work

### Completing

1. Verify no Todo/In Progress issues remain
2. Set state to Completed
3. Archive project

## Checklist: New Project

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
