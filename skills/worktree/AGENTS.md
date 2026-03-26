# Git Worktree Management

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when managing
> git worktrees. Humans may also find it useful, but guidance here is
> optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

Portable git worktree manager that creates, lists, and removes isolated working
copies with automatic env/config symlinks. Uses a layout of `project/main`
(repo) plus `project/trees/{id}` (worktrees), resolves project root via
`git rev-parse`, detects the default branch automatically, and reads all
project-specific configuration from `.env.local`.

---

## 1. Commands

| Command   | Description |
|-----------|-------------|
| `create`  | Create worktree for issue. Reuses existing (with rebase). Auto-detects PR branches via `gh`. |
| `list`    | List all worktrees |
| `remove`  | Remove worktree, clean symlinks, prune branches |
| `cleanup` | Remove worktrees whose branches are merged |
| `path`    | Print worktree path for issue ID |
| `exists`  | Check if worktree exists for issue ID |
| `check`   | Pre-create git state check (JSON: uncommitted, unpushed) |
| `push`    | Push worktree branch with auto-rebase |

## 2. .env.local Config

All optional. `.env.local` itself is always symlinked into worktrees.

| Variable | Purpose |
|----------|---------|
| `WORKTREE_DEFAULT_BRANCH` | Override default branch detection (fallback: `main`) |
| `WORKTREE_SYMLINKS` | Space-separated paths to symlink from main (e.g., `.cache benchmarks/results`) |
| `WORKTREE_COPIES` | Space-separated paths to copy from main (e.g., `.claude/settings.json`) |
| `BOT_NAME` / `BOT_EMAIL` | Git identity for worktree commits |
| `BOT_SIGNING_KEY` | SSH signing key path |
| `BOT_REMOTE_NAME` / `BOT_REMOTE_URL` | Remote for bot pushes |

## 3. Skill Invocation

When user runs `/worktree`, parse arguments and run the `scripts/worktree` script relative to this skill's install location:

- `/worktree create PROJ-123` -> `scripts/worktree create PROJ-123`
- `/worktree list` -> `scripts/worktree list`
- `/worktree remove PROJ-123` -> `scripts/worktree remove PROJ-123`
