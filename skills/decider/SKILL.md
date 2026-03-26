---
name: decider
description: Architectural decision document management — templates, creation workflows, search/query, supersession tracking, and INDEX maintenance. Use when creating, searching, updating, or superseding decision entries (DXXX documents).
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Decider

Architectural decision document management with canonical templates, creation/update workflows, and a search CLI. Provides the single source of truth for decision entry format and lifecycle.

## When to Apply

Reference these guidelines when:
- Creating a new decision entry after research completion
- Recording a significant path choice during implementation
- Searching for existing decisions governing an area of code
- Checking if a proposed change contradicts an active decision
- Superseding or partially superseding an existing decision
- Including decision context in PR bodies or delegation prompts
- Validating decision references in issue descriptions

## Skill Dependencies

This skill is self-contained. Other skills depend on it:

| Dependent Skill | Purpose |
|-----------------|---------|
| Orchestration | Decision creation in research-complete, search in review/submit workflows |
| Issue Lifecycle | Decision search in dev-implement/dev-fix/qa-review, creation in dev-implement |
| Project Management | Decision search in audit/roadmap workflows |

Project-level configuration:

| Variable | Purpose | Default |
|----------|---------|---------|
| `$DECISIONS_CMD` | Path to decisions CLI | `scripts/decisions` |
| `$DECISIONS_DIR` | Path to decision documents directory | — (required) |

## Templates

| Template | Purpose |
|----------|---------|
| `templates/decision-entry.md` | Decision file format (minimal, standard, comprehensive) |
| `templates/index-row.md` | INDEX.md table row format |

## Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `workflows/create-decision.md` | Research complete, significant path choice | Assign ID, write file, add INDEX row, update superseded |
| `workflows/update-decision.md` | New decision affects existing | Supersede, partial supersede, or revisit existing entries |
| `workflows/search-decisions.md` | Before implementing, reviewing, auditing | Search by issue, keywords, or ID |

## Schemas

| Schema | Purpose |
|--------|---------|
| `schemas/decision-format.md` | Canonical format constraints for decision documents and INDEX |

## Scripts

| Script | Purpose |
|--------|---------|
| `scripts/decisions` | CLI entry point for `$DECISIONS_CMD` — search, next-id, get |

## CLI Commands

| Command | Purpose | Output |
|---------|---------|--------|
| `$DECISIONS_CMD search --issue [ID]` | Find decisions linked to an issue | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD search "[KEYWORDS]"` | Ranked keyword search (AND, scored) | JSON `[{id, decision, path, score}]` |
| `$DECISIONS_CMD search "a\|b"` | Regex OR search | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD list` | List all active decisions | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD next-id` | Get next available DXXX | Single `DXXX` line |
| `$DECISIONS_CMD get [DXXX]` | Get decision details | JSON `{id, decision, status, date, path}` |

Options: `--limit N` (default: 5) for search results.

## Decision Lifecycle

```
Research Complete → Create Decision (§ 6.1)
                        ↓
                 INDEX.md + DXXX-descriptor.md
                        ↓
            ┌───────────┴───────────┐
            ↓                       ↓
    Search/Reference         Update/Supersede
    (review, audit,          (new research,
     implementation)          revisit conditions met)
```

## Quick Reference

### Creating Decisions

1. Get next ID: `$DECISIONS_CMD next-id`
2. Select template size (minimal/standard/comprehensive) from `templates/decision-entry.md`
3. Write decision file to `[project decision documents]/[DECISION_ID]-[DESCRIPTOR].md`
4. Add row to `[project decision documents]/INDEX.md`
5. Update any partially superseded decisions

### Searching Decisions

1. By issue: `$DECISIONS_CMD search --issue [ISSUE_ID]`
2. By keywords: `$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"`
3. Read full decision files — index summaries are insufficient for understanding scope and rejected alternatives
4. Suggestions contradicting active decisions are invalid unless decision is flawed

### Decision Entry Format

All entries require: title (`# DXXX: Title`), date, status, research ref (or `—`), decision statement, rationale, revisit conditions. See `schemas/decision-format.md` for full constraints.

## Configuration

| Variable | Purpose | Required |
|----------|---------|----------|
| `DECISIONS_DIR` | Decision documents directory path | Yes |

## System Dependencies

- `bash` 4+
- `jq` for JSON processing
- `grep` with `-P` (PCRE) support
- `sed`, `find`

## Full Compiled Document

For the complete guide with all templates, workflows, and schemas expanded inline: `AGENTS.md`
