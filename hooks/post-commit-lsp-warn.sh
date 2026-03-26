#!/usr/bin/env bash
# ---
# name: post-commit-lsp-warn
# event: PostToolUse
# matcher: Bash
# description: Warn about potentially stale LSP diagnostics after git commits that touch source files.
# safety: Prevents agents from acting on outdated LSP diagnostics in the ~30s after a commit.
# ---

set -euo pipefail

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | grep -o '"command"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | sed 's/.*"command"[[:space:]]*:[[:space:]]*"//;s/"$//' 2>/dev/null || true)

# Only relevant for git commit commands
if ! echo "$COMMAND" | grep -qE 'git[[:space:]]+commit'; then
  exit 0
fi

# Check if source files were in the commit
CHANGED=$(git diff HEAD~1 HEAD --name-only 2>/dev/null || true)
if [ -z "$CHANGED" ]; then
  exit 0
fi

# Check for common source file extensions
if echo "$CHANGED" | grep -qE '\.(rs|ts|tsx|js|jsx|py|go|java|c|cpp|h|hpp)$'; then
  echo '{"additionalContext":"Post-commit: LSP diagnostics may be stale for ~30s. Verify with your build command before acting on new diagnostics."}'
fi
