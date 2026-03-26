#!/bin/bash
# GitHub API - Resolve review thread(s)
# Usage: resolve-thread.sh <thread-id> [<thread-id>...]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Resolve Review Thread(s)

Usage: resolve-thread.sh <thread-id> [<thread-id>...]

Arguments:
  thread-id    GraphQL thread ID (PRRT_...) - can specify multiple

Options:
  --dry-run    Show what would be done without executing

Output:
{
  "success": true,
  "resolved": ["PRRT_...", "PRRT_..."],
  "failed": []
}

Examples:
  # Single thread
  resolve-thread.sh PRRT_kwDONRcYOs6D8dg9

  # Multiple threads
  resolve-thread.sh PRRT_... PRRT_... PRRT_...

  # From stdin (one per line)
  echo "PRRT_..." | resolve-thread.sh --stdin
EOF
}

resolve_threads() {
    local thread_ids=()
    local dry_run="false"
    local from_stdin="false"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --dry-run)
                dry_run="true"
                shift
                ;;
            --stdin)
                from_stdin="true"
                shift
                ;;
            PRRT_*)
                thread_ids+=("$1")
                shift
                ;;
            *)
                echo "{\"error\": \"Invalid thread ID: $1 (must start with PRRT_)\"}" >&2
                exit 1
                ;;
        esac
    done

    # Read from stdin if requested
    if [ "$from_stdin" = "true" ]; then
        while IFS= read -r line; do
            [[ "$line" == PRRT_* ]] && thread_ids+=("$line")
        done
    fi

    if [ ${#thread_ids[@]} -eq 0 ]; then
        echo '{"error": "No thread IDs provided"}' >&2
        exit 1
    fi

    # Dry run - just report
    if [ "$dry_run" = "true" ]; then
        printf '{"dry_run": true, "would_resolve": %s}\n' \
            "$(printf '%s\n' "${thread_ids[@]}" | jq -R . | jq -s .)"
        exit 0
    fi

    # Single thread - simple mutation
    if [ ${#thread_ids[@]} -eq 1 ]; then
        local query='
mutation($threadId: ID!) {
  resolveReviewThread(input: {threadId: $threadId}) {
    thread { id isResolved }
  }
}'
        local result
        result=$(gh_graphql "$query" -F threadId="${thread_ids[0]}") || {
            echo "{\"success\": false, \"resolved\": [], \"failed\": [\"${thread_ids[0]}\"]}"
            exit 1
        }

        local resolved
        resolved=$(echo "$result" | jq -r '.resolveReviewThread.thread.isResolved // false')
        if [ "$resolved" = "true" ]; then
            echo "{\"success\": true, \"resolved\": [\"${thread_ids[0]}\"], \"failed\": []}"
        else
            echo "{\"success\": false, \"resolved\": [], \"failed\": [\"${thread_ids[0]}\"]}"
            exit 1
        fi
        return
    fi

    # Multiple threads - batch mutation with aliases
    local mutation_parts=()
    local idx=0
    for tid in "${thread_ids[@]}"; do
        mutation_parts+=("t${idx}: resolveReviewThread(input: {threadId: \"$tid\"}) { thread { id isResolved } }")
        idx=$((idx + 1))
    done

    local batch_query
    batch_query="mutation { $(printf '%s\n' "${mutation_parts[@]}" | tr '\n' ' ') }"

    local result
    result=$(gh_graphql "$batch_query") || {
        echo "{\"success\": false, \"resolved\": [], \"failed\": $(printf '%s\n' "${thread_ids[@]}" | jq -R . | jq -s .)}"
        exit 1
    }

    # Parse results
    local resolved=()
    local failed=()
    idx=0
    for tid in "${thread_ids[@]}"; do
        local is_resolved
        is_resolved=$(echo "$result" | jq -r ".t${idx}.thread.isResolved // false")
        if [ "$is_resolved" = "true" ]; then
            resolved+=("$tid")
        else
            failed+=("$tid")
        fi
        idx=$((idx + 1))
    done

    # Output result
    local resolved_json failed_json
    resolved_json=$(printf '%s\n' "${resolved[@]:-}" | jq -R 'select(length > 0)' | jq -s . 2>/dev/null || echo '[]')
    failed_json=$(printf '%s\n' "${failed[@]:-}" | jq -R 'select(length > 0)' | jq -s . 2>/dev/null || echo '[]')

    local success="true"
    [ ${#failed[@]} -gt 0 ] && success="false"

    echo "{\"success\": $success, \"resolved\": $resolved_json, \"failed\": $failed_json}"
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

resolve_threads "$@"
