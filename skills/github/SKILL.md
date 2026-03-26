---
name: github
description: GitHub API CLI for PR operations — threads, comments, reviews, CI logs, merging, and cross-PR analysis. Use when querying PR data, managing review threads, posting comments, checking CI status, or automating merge workflows.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# GitHub Queries

CLI wrapper for GitHub API operations used in PR workflows.

## When to Apply

Reference these guidelines when:
- Querying PR data, threads, or review status
- Posting or editing PR comments and thread replies
- Automating PR creation, merging, or CI log retrieval
- Building multi-PR analysis workflows
- Integrating bot account operations

## Entry Point

```bash
scripts/github.sh <command> [options]
```

## Commands

| Command | Purpose |
|---------|---------|
| `pr-data <N>` | Get PR with threads, comments, files |
| `pr-view [N] [--json FIELDS]` | View PR details (wraps gh pr view) |
| `pr-threads <N> [--unresolved]` | Get thread list/count |
| `pr-review-status <N> [--baseline-ts TS]` | Check state, determine if action needed |
| `pr-list-ready [--all] [--format=safe\|table]` | List PRs ready for merge |
| `pr-list-failing [--all] [--format=safe\|table]` | List PRs with CI failures |
| `pr-create [--title T] [--body B] [--draft]` | Create PR as bot (checks: not main, has commits) |
| `pr-merge <N> [--check\|--force]` | Merge PR as bot (--check: JSON output for workflow) |
| `pr-cross-check [N...] [--quick\|--verify]` | Analyze PRs: --quick (heuristic) or --verify (full build+test) |
| `pr-issue <N> [--format=safe\|text]` | Extract issue ID from PR branch |
| `ci-logs <N> [--lines N] [--format=safe\|text]` | Get CI failure logs for PR |
| `bot-token [--format=safe\|text]` | Check if bot token is configured |
| `dismiss-review <PR> [--bot\|--user NAME]` | Dismiss blocking review |
| `resolve-thread <PRRT_...>` | Mark thread(s) resolved |
| `unresolve-thread <PRRT_...>` | Reopen thread(s) |
| `post-reply <id> <body>` | Reply to review comment |
| `post-comment <PR> <body>` | Post PR-level comment |
| `find-comment <PR> --pattern <regex>` | Find comment by pattern/author |
| `edit-comment <id> <body>` | Edit existing comment |
| `sticky-comment <PR> [--verdict\|--analysis]` | Get bot sticky comment |

## Output Formats

| Format | Description | Commands |
|--------|-------------|----------|
| `safe` | DEFAULT. Flat, normalized JSON | All |
| `raw` | Original API structure | pr-data, pr-threads |
| `text` | Plain text extraction | pr-issue, ci-logs, bot-token |
| `table` | Human-readable table | pr-list-ready, pr-list-failing |

## Configuration

| Variable | Purpose | Default |
|----------|---------|---------|
| `GH_BOT_TOKEN` | Bot account GitHub token (in `.env.local`) | Falls back to `gh` auth |
| `GH_BOT_USERNAME` | Bot username for review/comment filtering | `claude[bot]` |
| `GH_ISSUE_PATTERN` | Regex for issue ID extraction from branches | `[A-Z]+-[0-9]+` |

## Error Handling

- Returns `{"error": "message"}` on stderr with exit code 1
- Automatic retry on rate limiting (3 attempts with backoff)
- NOT_FOUND errors return clean `{"error": "Not found"}`

## Dependencies

- `gh` CLI authenticated (`gh auth login`)
- `jq` for JSON processing
- `op` CLI (optional, for 1Password token references)

## Full Compiled Document

For the complete guide with all examples: `AGENTS.md`
