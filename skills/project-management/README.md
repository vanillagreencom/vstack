# Project Management Skill

TPM methodology for roadmap planning, cycle planning, issue auditing, prioritization, and progress tracking. Workflows analyze issue tracker state and return structured JSON recommendations — the orchestrator or user handles execution.

This skill is methodology-based (no `rules/` directory). All guidance lives in reference documents, workflows, and schemas.

## Structure

```
skills/project-management/
├── SKILL.md                                # Quick-reference index for Claude Code and skill-aware harnesses
├── AGENTS.md                               # Full compiled document for Codex, Copilot, Gemini CLI, and other harnesses
├── README.md                               # This file — human-facing docs
├── references/
│   ├── issues.md                           # Issue creation, fields, sub-issues, estimates, templates
│   ├── initiatives-projects.md             # Initiative/project lifecycle, naming, breakdown
│   ├── dependencies.md                     # Blocking rules, relation types, remediation
│   ├── prioritization.md                   # Scoring formula, factor definitions, trade-offs
│   └── labels.md                           # Label taxonomy, exclusivity, creation, lifecycle
├── workflows/
│   ├── tpm-cycle-plan.md                   # Analyze backlog, compute architecture order for cycle
│   ├── tpm-roadmap-plan.md                 # Cross-project analysis, architecture gaps
│   ├── tpm-audit.md                        # Audit issues/projects for relations, hierarchy
│   └── tpm-audit-project-order.md          # Analyze project dependencies and ordering
└── schemas/
    ├── cycle-plan-output.md                # Cycle planning JSON output schema
    ├── roadmap-plan-output.md              # Roadmap analysis JSON output schema
    ├── audit-output.md                     # Issue/project audit JSON output schema
    └── audit-project-order-output.md       # Project order audit JSON output schema
```

## Skill Dependencies

This skill requires an issue tracker CLI for all read/write operations. Configure the `$ISSUE_CLI` variable to point to your issue tracker's CLI tool.

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD, cache, comments, labels, relations | `$ISSUE_CLI` |

## Configuration Variables

| Variable | Required | Purpose | Example |
|----------|----------|---------|---------|
| `$ISSUE_CLI` | Yes | Path or alias to issue tracker CLI | `linear.sh`, `gh issue`, custom script |
| `$VALIDATE_CMD` | No | Build + test + lint command | `make validate`, `npm test` |

### Example Setup

```bash
# In your project's .env or shell config
export ISSUE_CLI="./scripts/linear.sh"
export VALIDATE_CMD="make validate"
```

## Key Concepts

- **Hierarchy**: Initiative → Project → Milestone → Issue → Sub-Issue
- **Prioritization**: Weighted scoring formula (Critical Path x3, Dependencies x2, Risk x2, Value x1, Estimate x-0.5)
- **Same-project rule**: Blocking relations and parent-child relations must be within the same project
- **Blocking level rule**: Blocking relations go on bundle parents, not children
- **Workflows return JSON only**: No direct modifications to the issue tracker — recommendations are executed by the caller
