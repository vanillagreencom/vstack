# GitHub Queries

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when using
> the GitHub Queries CLI. Humans may also find it useful, but guidance
> here is optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

CLI wrapper for GitHub API operations used in PR workflows. Provides structured JSON output for all commands, bot account support for automated operations, and configurable patterns for issue ID extraction.

## Entry Point

```bash
scripts/github.sh <command> [options]
scripts/github.sh -C <path> <command> [options]  # Run in different directory
```

## Configuration

Set these in `.env.local` or as environment variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `GH_BOT_TOKEN` | Bot account GitHub token | Falls back to `gh` auth |
| `GH_BOT_USERNAME` | Bot username for review/comment filtering | `review-bot[bot]` |
| `GH_ISSUE_PATTERN` | Regex for issue ID extraction from branches | `[A-Z]+-[0-9]+` |

Bot token supports direct tokens (`ghp_*`, `gho_*`, `ghs_*`, `ghr_*`, `github_pat_*`) and 1Password references (`op://vault/item/field`).

## Commands

### PR Data

```bash
# Get PR data with all threads and comments
github.sh pr-data 23
github.sh pr-data              # Uses current branch's PR
github.sh pr-data 23 --actionable  # Only unresolved non-outdated threads

# Quick PR view (wraps gh pr view)
github.sh pr-view 23 --json number,title,state
github.sh pr-view              # Current branch
```

### Thread Operations

```bash
# Count unresolved threads
github.sh pr-threads 23 --unresolved | jq '.unresolved_count'

# Resolve / unresolve threads
github.sh resolve-thread PRRT_kwDO... PRRT_kwDO...
github.sh unresolve-thread PRRT_kwDO...
```

### Comment Operations

```bash
# Post comment and replies
github.sh post-comment 23 "All feedback addressed"
github.sh post-reply 12345678 "Fixed!"

# Find and edit comments
github.sh find-comment 23 --pattern "## Summary"
github.sh edit-comment 12345678 "Updated body"
```

### PR Lists

```bash
# List PRs ready for merge
github.sh pr-list-ready                 # JSON (default)
github.sh pr-list-ready --format=table  # Human-readable
github.sh pr-list-ready --all           # All repos, not just local branches

# List PRs with CI failures
github.sh pr-list-failing
github.sh pr-list-failing --all --format=table
```

### CI Logs

```bash
github.sh ci-logs 42               # JSON with metadata
github.sh ci-logs 42 --lines 200   # More log lines
github.sh ci-logs 42 --format=text # Human-readable
```

### PR Creation

```bash
github.sh pr-create --title "feat: Add feature" --body "## Summary"
github.sh pr-create --draft --label defer-ci
github.sh pr-create --dry-run  # Preview without creating
```

Safety checks (skip with `--force`):
1. Not creating PR from main/master
2. Head branch has commits ahead of base
3. Branch has been pushed to remote

### PR Merging

```bash
github.sh pr-merge 42 --check   # Check readiness, output JSON
github.sh pr-merge 42           # Check + merge if pass
github.sh pr-merge 42 --force   # Skip checks (requires explicit decision)
```

Check output:
```json
{"can_merge": true, "issues": [], "warnings": [], "mergeable": "MERGEABLE", "review": "APPROVED"}
```

### Cross-PR Analysis

```bash
github.sh pr-cross-check                 # Quick heuristic of ready PRs
github.sh pr-cross-check 70 71 --quick   # Quick check specific PRs (~5s)
github.sh pr-cross-check 70 71 --verify  # Full verification (auto-detects build system)
```

### Issue ID Extraction

```bash
github.sh pr-issue 42               # JSON: {"issue": "ABC-150", "branch": "..."}
github.sh pr-issue 42 --format=text # Plain: ABC-150
```

Configurable via `GH_ISSUE_PATTERN` env var. Default matches `ABC-123` style patterns.

### Bot Review Status

```bash
# Check review status with baseline comparison
github.sh pr-review-status 23 --baseline-ts "2024-01-05T12:00:00Z" --baseline-threads 2

# Get bot sticky comment
github.sh sticky-comment 23                # Full JSON
github.sh sticky-comment 23 --verdict      # Quick: approved/changes/pending
github.sh sticky-comment 23 --analysis     # Deep: recommendation, remaining items
github.sh sticky-comment 23 --body         # Just comment body

# Check bot token
github.sh bot-token               # JSON: {"configured": true, "valid": true}
github.sh bot-token --format=text # Text: "configured"

# Dismiss blocking review
github.sh dismiss-review 23 --bot --message "Re-reviewed"
```

## Output Formats

| Format | Description |
|--------|-------------|
| `safe` | DEFAULT. Flat, normalized JSON |
| `raw` | Original GitHub API structure |
| `text` | Plain text extraction |
| `table` | Human-readable table |

`--json` is accepted as alias for `--format=safe`.

## Error Handling

- Returns `{"error": "message"}` on stderr with exit code 1
- Automatic retry on rate limiting (3 attempts with backoff)
- NOT_FOUND errors return clean `{"error": "Not found"}`

## Troubleshooting

**`Expected VAR_SIGN, actual: UNKNOWN_CHAR`**: Use multi-line GraphQL + `-F` for variables (shell escaping issue with `$` in single-line queries).

## Dependencies

- `gh` CLI authenticated (`gh auth login`)
- `jq` for JSON processing
- `op` CLI (optional, for 1Password token references)
