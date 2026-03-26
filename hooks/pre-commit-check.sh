#!/usr/bin/env bash
# ---
# name: pre-commit-check
# event: PreToolUse
# matcher: Bash
# description: Validate formatting and lint before git commits on source files. Currently supports Rust (cargo fmt + clippy).
# safety: Prevents committing code that fails format or lint checks.
# ---

set -euo pipefail

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | grep -o '"command"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | sed 's/.*"command"[[:space:]]*:[[:space:]]*"//;s/"$//' 2>/dev/null || true)

# Only relevant for git commit commands
if ! echo "$COMMAND" | grep -qE 'git[[:space:]]+commit'; then
  exit 0
fi

# Check staged files
STAGED=$(git diff --cached --name-only 2>/dev/null || true)
if [ -z "$STAGED" ]; then
  exit 0
fi

# Check for Rust files
if echo "$STAGED" | grep -qE '\.rs$'; then
  # Format check
  if ! cargo fmt --check 2>/dev/null; then
    echo "cargo fmt --check failed. Run 'cargo fmt' first." >&2
    exit 2
  fi

  # Clippy check
  if ! cargo clippy --workspace --all-targets -- -D warnings 2>/dev/null; then
    echo "cargo clippy found warnings. Fix them before committing." >&2
    exit 2
  fi
fi

exit 0
