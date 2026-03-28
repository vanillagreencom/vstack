---
name: project-management
description: TPM analysis workflows for roadmap planning, cycle planning, prioritization, and progress tracking. Returns JSON recommendations — orchestrator or user executes.
license: MIT
user-invocable: true
dependencies:
  optional: [linear, decider]
metadata:
  author: vanillagreen
  version: "1.1.0"
---

# Project Management

TPM methodology for roadmap planning, cycle planning, issue auditing, and progress tracking. Workflows analyze state and return structured JSON recommendations — the orchestrator or user handles execution.

## When to Apply

Reference these guidelines when:
- Planning development cycles (sprint/iteration planning)
- Analyzing roadmap priorities and project ordering
- Auditing issues for relations, hierarchy, duplicates, or misconfigurations
- Creating or breaking down initiatives, projects, and issues
- Managing dependencies between issues and projects
- Prioritizing work using structured scoring
- Tracking project health indicators
- Managing label taxonomy and application

## Skill Dependencies

Workflows reference these companion skills. Install and configure per your project:

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments, labels, relations | `$ISSUE_CLI` |
| Decider skill (optional) | Decision search for audit/roadmap contradiction checks | `$DECISIONS_CMD` |

Project-level configuration:

| Variable | Purpose |
|----------|---------|
| `$VALIDATE_CMD` | Build + test + lint command (optional) |
| `$DECISIONS_CMD` | Decision document lookup (optional) |

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

## Prioritization

```
Score = (Critical Path x 3) + (Dependencies x 2) + (Risk x 2) + (Value x 1) - (Estimate x 0.5)
```

**Thresholds**: 8+ P1 | 5-7 P2 | 3-4 P3 | 0-2 P4

## Health Indicators

| Indicator | Green | Yellow | Red |
|-----------|-------|--------|-----|
| Blocked issues | 0 | 1-2 | 3+ |
| In Progress age | <3 days | 3-7 days | >7 days |
| Completion ratio (7d) | >0.8 | 0.5-0.8 | <0.5 |

## Workflows

TPM workflows return JSON recommendations only. Orchestrator or user handles interaction and execution.

| Workflow | Purpose |
|----------|---------|
| [tpm-cycle-plan](workflows/tpm-cycle-plan.md) | Analyze backlog, compute architecture order |
| [tpm-roadmap-plan](workflows/tpm-roadmap-plan.md) | Cross-project analysis, architecture gaps |
| [tpm-audit](workflows/tpm-audit.md) | Audit issues/projects for relations, hierarchy |
| [tpm-audit-project-order](workflows/tpm-audit-project-order.md) | Analyze project dependencies and ordering |

## Output Schemas

| Workflow | Schema |
|----------|--------|
| Cycle planning | [cycle-plan-output.md](schemas/cycle-plan-output.md) |
| Roadmap analysis | [roadmap-plan-output.md](schemas/roadmap-plan-output.md) |
| Issue/project audit | [audit-output.md](schemas/audit-output.md) |
| Project order audit | [audit-project-order-output.md](schemas/audit-project-order-output.md) |

## References

| Topic | Location |
|-------|----------|
| Issue creation | [references/issues.md](references/issues.md) |
| Initiatives & Projects | [references/initiatives-projects.md](references/initiatives-projects.md) |
| Dependencies | [references/dependencies.md](references/dependencies.md) |
| Prioritization factors | [references/prioritization.md](references/prioritization.md) |
| Label management | [references/labels.md](references/labels.md) |
| Issue tracker CLI | Companion issue tracker skill (`$ISSUE_CLI`) |

## Full Compiled Document

For the complete guide with all content expanded: `AGENTS.md`
