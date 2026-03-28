# GitHub Queries

CLI wrapper for GitHub API operations used in PR workflows.

## Structure

- `scripts/github.sh` - Main entry point (command router)
- `scripts/commands/` - Individual command scripts (one per command)
- `scripts/lib/github-api.sh` - Shared library (auth, GraphQL, REST, error handling)
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Configuration

Set in `.env.local` or as environment variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `GH_BOT_TOKEN` | Bot account GitHub token | Falls back to `gh` auth |
| `GH_BOT_USERNAME` | Bot username for filtering | `review-bot[bot]` |
| `GH_ISSUE_PATTERN` | Regex for branch issue extraction | `[A-Z]+-[0-9]+` |

## Minimum Setup

To make this skill work:

1. Authenticate the `gh` CLI: `gh auth login`
2. Optionally set `GH_BOT_TOKEN` if PR/comment/review actions should use a bot account instead of your current `gh` login.
3. Optionally set `GH_ISSUE_PATTERN` if your branch names do not use the default `ABC-123` style issue IDs.

Quick check:

```bash
./scripts/github.sh pr-view 123 --json number,title,state
./scripts/github.sh bot-token
```

## Adding a New Command

1. Create `scripts/commands/<command-name>.sh`
2. Source `../lib/github-api.sh` for shared functions
3. Add a `show_help()` function
4. Add the command to the case statement in `scripts/github.sh`
5. Update SKILL.md command table and AGENTS.md examples

## Dependencies

- `gh` CLI authenticated
- `jq` for JSON processing
- `op` CLI (optional, for 1Password token references)

## Verification (pr-cross-check --verify)

`verify-lib.sh` auto-detects the project's build system and runs the right commands. Override order:
1. `GH_VERIFY_CMD` env var
2. `verify.sh` in project root
3. Auto-detect from `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`, `Makefile`
