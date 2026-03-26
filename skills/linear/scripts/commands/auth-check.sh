#!/bin/bash
# Lightweight auth validation
# Usage: ./linear.sh auth-check
# Returns: {"ok": true/false, "error": "..."} — exit 0 on success, 1 on failure

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"

if [[ -z "${LINEAR_API_KEY:-}" ]]; then
  echo '{"ok":false,"error":"LINEAR_API_KEY not set"}'
  exit 1
fi

result=$(graphql_query "{ viewer { id } }" "{}" 2>/dev/null) || {
  echo '{"ok":false,"error":"API request failed"}'
  exit 1
}

viewer_id=$(echo "$result" | jq -r '.viewer.id // empty')
if [[ -n "$viewer_id" ]]; then
  echo '{"ok":true}'
else
  echo '{"ok":false,"error":"Invalid API key"}'
  exit 1
fi
