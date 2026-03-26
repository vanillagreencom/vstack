# Linear CLI

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when interacting
> with Linear via the CLI. Humans may also find it useful, but guidance
> here is optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

CLI wrapper for Linear's GraphQL API with local cache, bulk operations, and structured JSON output. Supports issues, projects, cycles, milestones, initiatives, labels, comments, and attachments.

## Entry Point

```bash
scripts/linear.sh <resource> <action> [options]
```

## Configuration

Set in `.env.local` or as environment variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `LINEAR_API_KEY` | API key (required) | — |
| `LINEAR_TEAM` | Default team name | `Claude` |
| `LINEAR_FORMAT` | Default output format | `safe` |
| `LINEAR_TEAM_PREFIX` | Issue identifier prefix | `CC` |

## Hierarchy

```
INITIATIVE (Strategic goal — months)
  └── PROJECT (2-6 week deliverable)
        ├── PROJECT DEPENDENCIES (blocking/blocked-by other projects)
        ├── MILESTONE (stage: Alpha, Beta, Release)
        │     └── ISSUE (1-5 day work item)
        └── ISSUE (work item without milestone)
              └── SUB-ISSUE (breakdown for parallel work)
```

Issue relations (cross-cutting): `blocks`, `blocked-by`, `related`, `duplicate`

## Cache Pattern

Reads go through `cache`. Writes go through live commands (auto-update cache via write-through). Sync at session start or when cache is stale.

```bash
# READS → cache (fast, no API calls)
linear.sh cache issues list --project "Phase 2" --state "Todo,In Progress"
linear.sh cache issues get ABC-100 --with-bundle
linear.sh cache projects list --state started
linear.sh cache labels list
linear.sh cache comments list ABC-100

# WRITES → live (hit API, auto-update cache)
linear.sh issues create --title "New task" --project "Phase 2"
linear.sh issues update ABC-100 --state "Done"

# ATTACHMENTS → images/files from issues
linear.sh cache attachments list ABC-100
linear.sh cache attachments fetch ABC-100
linear.sh cache attachments stats

# SYNC → refresh cache
linear.sh sync --reconcile      # Incremental + reconcile archived/deleted
linear.sh sync                  # Incremental (reconcile hourly)
linear.sh sync --full           # Full re-sync from scratch
```

When to sync: start of every session (`--reconcile`), after user edits on Linear web/app, or when cache reads return stale data. Mutations auto-update cache — no sync needed after your own writes.

## Issues

```bash
# List with filters
linear.sh issues list --label "backend" --state "Todo,In Progress"
linear.sh cache issues list --project "Phase 2" --format=table

# Get single issue
linear.sh cache issues get ABC-100
linear.sh cache issues get ABC-100 --with-bundle  # Includes children, relations, comments

# Create
linear.sh issues create --title "New task" --project "Phase 2" --labels "component:api"
linear.sh issues create --title "Sub-task" --parent ABC-42

# Update
linear.sh issues update ABC-100 --state "Done"
linear.sh issues update ABC-100 --labels "blocked" --assignee "me"

# Children and relations
linear.sh issues children ABC-42              # Direct children
linear.sh issues children ABC-42 --recursive  # All descendants (3 levels)
linear.sh issues add-relation ABC-42 --blocks ABC-43
linear.sh issues add-relation ABC-42 --blocked-by ABC-41
linear.sh issues list-relations ABC-42

# Bulk operations
linear.sh issues bulk-get ABC-100 ABC-101 ABC-102
```

## Projects

```bash
linear.sh projects list --state started
linear.sh projects get <uuid>
linear.sh projects create --name "New Project" --description "..."
linear.sh projects add-dependency <id> --blocked-by <other-id>
linear.sh projects post-update <id> --health on-track --body "Progress update"
```

## Initiatives & Milestones

```bash
linear.sh initiatives list
linear.sh initiatives create --name "Phase 1" --target-date 2025-03-31
linear.sh initiatives add-project <id> --project "Market Data Pipeline"

linear.sh milestones list --project "Market Data Pipeline"
linear.sh milestones create --project <id> --name "Alpha" --target-date 2025-02-15
```

## Comments

```bash
linear.sh cache comments list ABC-100
linear.sh comments create ABC-100 --body "Status update: completed API integration"
```

## Metadata

```bash
linear.sh labels list
linear.sh statuses list           # Available workflow states
linear.sh teams list
linear.sh users list
linear.sh cycles list
linear.sh documents list
```

## Output Formats

| Format | Description |
|--------|-------------|
| `safe` | DEFAULT. Flat, null-safe JSON |
| `ids` | Newline-separated identifiers |
| `table` | Human-readable table |
| `raw` | Original GraphQL structure |

Safe format field mapping:
```
identifier → id         # ABC-XXX issue ID
id → uuid              # GraphQL UUID
state.name → state     # State name
state.type → state_type
sortOrder → sort_order  # Manual sort position
```

## Blocked Label vs Issue Relations

| Scenario | Use |
|----------|-----|
| Issue A blocked by Issue B (both in Linear) | Relation: `--blocked-by` |
| Issue blocked by external factor (vendor, license) | `blocked` label + comment |

Done issues auto-unblock. Keep relations for history.

## Common Pitfalls

| Option | Accepts | On failure |
|--------|---------|-----------|
| `--project` | Name or UUID | Fail with "not found" |
| `--state` | Exact name (case-sensitive) | Fail, lists available states |
| `--milestone` | Name or UUID | Fail with "not found" |
| `--labels` | Comma-separated names | Warn + skip invalid, continue |
| `--assignee` | Name or `me` | Silent fail |

- State names are case-sensitive and team-specific — verify with `linear.sh statuses list`
- Available states: Backlog, Todo, In Progress, In Review, Done, Canceled (not "Cancelled")
- `agent:*` labels are mutually exclusive (only one per issue)

## Troubleshooting

**"labelIds not exclusive child labels" error**: Using multiple labels from the same exclusive group. Only one `agent:*` label and one `platform:*` label per issue.

**Need raw GraphQL output?**: Use `--format=raw`

**Script help**: `linear.sh <resource> --help`

## Dependencies

- `curl` for API calls
- `jq` for JSON processing
