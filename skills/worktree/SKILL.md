---
name: worktree
description: "Git worktree management — create, list, remove isolated working copies with env/config symlinks. Use when user says /worktree or needs to manage worktrees."
license: MIT
user-invocable: true
argument-hint: "create <ID> [--base <ref>] [--pr <N>] | list | remove <ID|path>"
metadata:
  author: vanillagreen
  version: "1.0.0"
---

# Worktree Management

Portable git worktree manager. Layout: `project/main` (repo) + `project/trees/{id}` (worktrees).

Resolves project root via `git rev-parse`, detects default branch automatically, and reads all project-specific config from `.env.local`.

## Commands

| Command | Description |
|---------|-------------|
| `create` | Create worktree for issue. Reuses existing (with rebase). Auto-detects PR branches via `gh`. |
| `list` | List all worktrees |
| `remove` | Remove worktree, clean symlinks, prune branches |
| `cleanup` | Remove worktrees whose branches are merged |
| `path` | Print worktree path for issue ID |
| `exists` | Check if worktree exists for issue ID |
| `check` | Pre-create git state check (JSON: uncommitted, unpushed) |
| `push` | Push worktree branch with auto-rebase |

## .env.local Config

All optional. `.env.local` itself is always symlinked into worktrees.

| Variable | Purpose |
|----------|---------|
| `WORKTREE_DEFAULT_BRANCH` | Override default branch detection (fallback: `main`) |
| `WORKTREE_SYMLINKS` | Space-separated paths to symlink from main (e.g., `.cache benchmarks/results`) |
| `WORKTREE_COPIES` | Space-separated paths to copy from main (e.g., `.tooling/settings.json`) |
| `BOT_NAME` / `BOT_EMAIL` | Git identity for worktree commits |
| `BOT_SIGNING_KEY` | SSH signing key path |
| `BOT_REMOTE_NAME` / `BOT_REMOTE_URL` | Remote for bot pushes |

## Skill Invocation

When user runs `/worktree`, parse arguments and run the `scripts/worktree` script relative to this skill's install location:
- `/worktree create PROJ-123` → `scripts/worktree create PROJ-123`
- `/worktree list` → `scripts/worktree list`
- `/worktree remove PROJ-123` → `scripts/worktree remove PROJ-123`
