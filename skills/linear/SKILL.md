---
name: linear
description: Linear API CLI for issues, projects, cycles, milestones, initiatives, labels, and comments. Use when querying or modifying Linear issues, planning cycles, managing projects, or any task involving Linear data.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Linear CLI

CLI wrapper for Linear's GraphQL API with local cache, bulk operations, and structured output.

## When to Apply

Reference these guidelines when:
- Querying or modifying Linear issues, projects, or cycles
- Planning sprints or managing milestones
- Applying labels or updating issue states
- Building automated workflows around Linear data
- Linking GitHub PRs to Linear issues

## Entry Point

```bash
scripts/linear.sh <resource> <action> [options]
```

## Hierarchy

```
INITIATIVE (Strategic goal — months)
  └── PROJECT (2-6 week deliverable)
        ├── MILESTONE (stage: Alpha, Beta, Release)
        │     └── ISSUE (1-5 day work item)
        └── ISSUE (work item without milestone)
              └── SUB-ISSUE (breakdown for parallel work)
```

## Commands

| Resource | Actions |
|----------|---------|
| `issues` | list, get, create, update, children, relations, bulk-get |
| `comments` | list, create |
| `projects` | list, get, create, update, dependencies, updates |
| `initiatives` | list, get, create, add-project |
| `milestones` | list, get, create |
| `labels` | list, create |
| `project-labels` | list, create |
| `teams` | list, get |
| `users` | list, get |
| `cycles` | list |
| `statuses` | list, get |
| `documents` | list, get |
| `sync` | Sync Linear data to local cache |
| `cache` | Query local cache (issues, projects, cycles, initiatives, comments, labels, attachments) |
| `auth-check` | Validate API key |

## Cache Pattern

Reads go through `cache`. Writes go through live commands (auto-update cache via write-through). Sync at session start or when cache is stale.

```bash
# READS → cache (fast, no API calls)
linear.sh cache issues list --project "Phase 2" --state "Todo,In Progress"
linear.sh cache issues get ABC-100 --with-bundle

# WRITES → live (hit API, auto-update cache)
linear.sh issues create --title "New task" --project "Phase 2"
linear.sh issues update ABC-100 --state "Done"

# SYNC → refresh cache
linear.sh sync --reconcile      # Incremental + reconcile archived
linear.sh sync --full           # Full re-sync
```

## Output Formats

| Format | Description |
|--------|-------------|
| `safe` | DEFAULT. Flat, null-safe JSON |
| `ids` | Newline-separated identifiers |
| `table` | Human-readable table |
| `raw` | Original GraphQL structure |

## Configuration

| Variable | Purpose | Default |
|----------|---------|---------|
| `LINEAR_API_KEY` | API key (required, in `.env.local`) | — |
| `LINEAR_TEAM` | Default team name | `Claude` |
| `LINEAR_FORMAT` | Default output format | `safe` |
| `LINEAR_TEAM_PREFIX` | Issue identifier prefix | `CC` |

## Common Pitfalls

| Option | Accepts | On failure |
|--------|---------|-----------|
| `--project` | Name or UUID | Fail with "not found" |
| `--state` | Exact name (case-sensitive) | Fail, lists available states |
| `--milestone` | Name or UUID | Fail with "not found" |
| `--labels` | Comma-separated names | Warn + skip invalid, continue |
| `--assignee` | Name or `me` | Silent fail |

## Resources

### ctx7 CLI

| Library | ctx7 ID | Use For |
|---------|---------|---------|
| Linear API | `/websites/studio_apollographql_public_linear-api_variant_current` | GraphQL schema reference |
| Linear SDK | `/linear/linear` | SDK docs with examples |
| Linear Guides | `/websites/linear_app_developers` | Developer guides |

## Dependencies

- `curl` for API calls
- `jq` for JSON processing

## Full Compiled Document

For the complete guide with all examples and reference tables: `AGENTS.md`
