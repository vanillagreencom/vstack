#!/usr/bin/env bash
# ---
# name: task-completed-check
# event: TaskCompleted
# matcher:
# description: Run workspace lint checks before marking a task complete. Currently supports Rust (cargo clippy).
# safety: Prevents marking tasks done when source files have lint violations.
# timeout: 120
# ---

set -euo pipefail

# Consume stdin
cat > /dev/null

# Check for changed source files
CHANGED=$(git diff --name-only 2>/dev/null || true)
STAGED=$(git diff --cached --name-only 2>/dev/null || true)
ALL_CHANGED=$(printf '%s\n%s' "$CHANGED" "$STAGED" | sort -u | grep -v '^$' || true)

if [ -z "$ALL_CHANGED" ]; then
  exit 0
fi

# Check for Rust files
if echo "$ALL_CHANGED" | grep -qE '\.rs$'; then
  OUTPUT=$(cargo clippy --workspace --all-targets -- -D warnings 2>&1 || true)
  ISSUES=$(echo "$OUTPUT" | grep -E '^error' | head -15)

  if [ -n "$ISSUES" ]; then
    echo "Clippy errors found — fix before completing task:" >&2
    echo "$ISSUES" >&2
    exit 2
  fi
fi

exit 0
