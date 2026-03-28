# Git Worktree Management

Git worktree lifecycle management with env/config symlinks.

## Structure

- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses
- `scripts/` - Worktree management scripts

## Quick Start

1. Run from the main checkout of a git repo that has an `origin` remote.
2. Optionally copy the repo-root [`.env.local.example`](/mnt/Tertiary/dev/vstack/main/.env.local.example) to `.env.local` and set any `WORKTREE_*` or `BOT_*` values you want.
3. Use the script:

```bash
./scripts/worktree create PROJ-123
./scripts/worktree list
./scripts/worktree path PROJ-123
./scripts/worktree remove PROJ-123
```

By default the tool:
- detects the default branch from `origin/HEAD` (fallback: `main`)
- creates worktrees under a sibling `trees/` directory
- symlinks `.env.local` into each worktree automatically

## Optional `.env.local` Settings

Set these only if your project needs them:

| Variable | Purpose |
|----------|---------|
| `WORKTREE_DEFAULT_BRANCH` | Override default branch detection |
| `WORKTREE_SYMLINKS` | Space-separated project-relative paths to symlink into worktrees |
| `WORKTREE_COPIES` | Space-separated project-relative files to copy into worktrees |
| `BOT_NAME` / `BOT_EMAIL` | Git identity to use inside worktrees |
| `BOT_SIGNING_KEY` | Optional SSH signing key for commits |
| `BOT_REMOTE_NAME` / `BOT_REMOTE_URL` | Optional remote used for bot pushes |
