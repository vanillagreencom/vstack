#!/usr/bin/env bash
# ---
# name: post-compact-lsp-warn
# event: PostCompact
# matcher:
# description: Warn about potentially stale LSP diagnostics after context compaction.
# safety: Prevents agents from acting on outdated diagnostics after memory is compacted.
# ---

set -euo pipefail

# Consume stdin
cat > /dev/null

# Output warning as additional context
cat <<'EOF'
{"additionalContext":"Post-compact: LSP diagnostics and file state may be stale. Run your project's build/check command to verify before acting on diagnostics."}
EOF
