# Decider

Architectural decision document management — templates, creation workflows, search/query CLI, and supersession tracking.

## Structure

```
skills/decider/
├── SKILL.md              # Quick-reference index for Claude Code and skill-aware harnesses
├── AGENTS.md             # Full compiled document for Codex, Copilot, Gemini CLI, and
│                         # 20+ harnesses. All workflows + templates + schemas expanded inline.
├── README.md             # This file — human-facing docs
├── templates/
│   ├── decision-entry.md # Decision file templates (minimal, standard, comprehensive)
│   └── index-row.md      # INDEX.md table row template
├── workflows/
│   ├── create-decision.md  # Create new decision: assign ID, write file, add INDEX row
│   ├── update-decision.md  # Supersede, partial supersede, or revisit existing decisions
│   └── search-decisions.md # Search by issue, keywords, or ID
├── schemas/
│   └── decision-format.md  # Canonical format constraints for decision documents
└── scripts/
    └── decisions           # CLI entry point ($DECISIONS_CMD)
```

This skill is workflow-based with templates and a CLI script. There is no `rules/` directory.

## Purpose

The decider skill provides the single source of truth for:

1. **Decision document format** — Three template sizes (minimal, standard, comprehensive) with consistent formatting rules
2. **Creation workflow** — Step-by-step process for recording decisions: ID assignment, template selection, file writing, INDEX maintenance, supersession handling
3. **Search/query CLI** — `$DECISIONS_CMD` interface for finding decisions by issue, keywords, or ID
4. **Update workflow** — Supersession, partial supersession, and revisitation of existing decisions
5. **Format schema** — Canonical constraints for file naming, metadata fields, status values, and cross-reference conventions

## Integration Points

This skill is a dependency for three workflow skills:

| Skill | Integration |
|-------|-------------|
| **Orchestration** | `research-complete` § 6.1 uses create-decision workflow; `review-pr`, `submit-pr`, `review-pr-comments` use search workflow |
| **Issue Lifecycle** | `dev-implement` § 4.3 uses create-decision workflow; `dev-fix` § 3 and `qa-review` § 2.1 use search workflow |
| **Project Management** | `tpm-audit` § 2 and `tpm-roadmap-plan` § 6.1 use search workflow |

## Configuration

### Required

| Variable | Purpose | Example |
|----------|---------|---------|
| `DECISIONS_DIR` | Path to decision documents directory | `docs/decisions` |

### CLI Setup

Set `$DECISIONS_CMD` to point to the decisions CLI script:

```bash
export DECISIONS_CMD="/path/to/skills/decider/scripts/decisions"
export DECISIONS_DIR="docs/decisions"
```

## CLI Commands

```bash
# Search by issue reference
decisions search --issue PROJ-189

# Search by keywords
decisions search "session caching"

# Get next available ID
decisions next-id

# Get decision details
decisions get D017
```

## Decision Templates

Three sizes based on decision scope:

| Template | Lines | When to Use |
|----------|-------|-------------|
| **Minimal** | 15-30 | Single technology choice, clear winner, 1-2 rationale points |
| **Standard** | 80-200 | Multiple alternatives considered, patterns to document, comparison tables |
| **Comprehensive** | 200-600 | Architecture-level decisions, multi-concern, API specs, design sections |

Choose the smallest template that covers the decision's scope. Keep tight — reference research for details.

## License

MIT
