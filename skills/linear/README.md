# Linear CLI

CLI wrapper for Linear's GraphQL API with local cache, bulk operations, and structured output.

## Structure

- `scripts/linear.sh` - Main entry point (resource router)
- `scripts/commands/` - Individual resource scripts (one per resource)
- `scripts/lib/common.sh` - Shared library (auth, GraphQL, formatting)
- `scripts/lib/cache.sh` - Cache management
- `scripts/lib/formatters.sh` - Output formatters (safe, table, ids, raw)
- `scripts/lib/attachments.sh` - Attachment download and caching
- `scripts/lib/issue-validation.sh` - Issue state validation
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Configuration

Set in `.env.local` or as environment variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `LINEAR_API_KEY` | API key (required) | — |
| `LINEAR_TEAM` | Default team name | `Claude` |
| `LINEAR_FORMAT` | Default output format | `safe` |
| `LINEAR_TEAM_PREFIX` | Issue identifier prefix | `CC` |

## Minimum Setup

To make this skill work:

1. Add `LINEAR_API_KEY` to `.env.local` or export it in your shell.
2. Optionally set `LINEAR_TEAM` and `LINEAR_TEAM_PREFIX` if the defaults do not match your project.
3. Verify auth:

```bash
./scripts/linear.sh auth-check
./scripts/linear.sh sync --reconcile
```

## Adding a New Resource

1. Create `scripts/commands/<resource>.sh`
2. Source `../lib/common.sh` for shared functions
3. Add a `show_help()` function
4. Add the resource to the case statement in `scripts/linear.sh`
5. Update SKILL.md command table and AGENTS.md examples

## Dependencies

- `curl` for API calls
- `jq` for JSON processing
