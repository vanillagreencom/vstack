#!/bin/bash
# GitHub API - Unresolve review thread(s)
# Usage: unresolve-thread.sh <thread-id> [<thread-id>...]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Unresolve Review Thread(s)

Usage: unresolve-thread.sh <thread-id> [<thread-id>...]

Arguments:
  thread-id    GraphQL thread ID (PRRT_...) - can specify multiple

Options:
  --dry-run    Show what would be done without executing

Output:
{
  "success": true,
  "unresolved": ["PRRT_...", "PRRT_..."],
  "failed": []
}

Examples:
  # Single thread
  unresolve-thread.sh PRRT_kwDONRcYOs6D8dg9

  # Multiple threads
  unresolve-thread.sh PRRT_... PRRT_...
EOF
}

unresolve_threads() {
    local thread_ids=()
    local dry_run="false"

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

    if [ ${#thread_ids[@]} -eq 0 ]; then
        echo '{"error": "No thread IDs provided"}' >&2
        exit 1
    fi

    # Dry run
    if [ "$dry_run" = "true" ]; then
        printf '{"dry_run": true, "would_unresolve": %s}\n' \
            "$(printf '%s\n' "${thread_ids[@]}" | jq -R . | jq -s .)"
        exit 0
    fi

    # Single thread
    if [ ${#thread_ids[@]} -eq 1 ]; then
        local query='
mutation($threadId: ID!) {
  unresolveReviewThread(input: {threadId: $threadId}) {
    thread { id isResolved }
  }
}'
        local result
        result=$(gh_graphql "$query" -F threadId="${thread_ids[0]}") || {
            echo "{\"success\": false, \"unresolved\": [], \"failed\": [\"${thread_ids[0]}\"]}"
            exit 1
        }

        # Success if thread is now unresolved (isResolved=false)
        local is_resolved
        is_resolved=$(echo "$result" | jq -r 'if .unresolveReviewThread.thread.isResolved == false then "false" elif .unresolveReviewThread.thread.isResolved == true then "true" else "null" end')
        if [ "$is_resolved" = "false" ]; then
            echo "{\"success\": true, \"unresolved\": [\"${thread_ids[0]}\"], \"failed\": []}"
        else
            echo "{\"success\": false, \"unresolved\": [], \"failed\": [\"${thread_ids[0]}\"]}"
            exit 1
        fi
        return
    fi

    # Multiple threads - batch mutation
    local mutation_parts=()
    local idx=0
    for tid in "${thread_ids[@]}"; do
        mutation_parts+=("t${idx}: unresolveReviewThread(input: {threadId: \"$tid\"}) { thread { id isResolved } }")
        idx=$((idx + 1))
    done

    local batch_query
    batch_query="mutation { $(printf '%s\n' "${mutation_parts[@]}" | tr '\n' ' ') }"

    local result
    result=$(gh_graphql "$batch_query") || {
        echo "{\"success\": false, \"unresolved\": [], \"failed\": $(printf '%s\n' "${thread_ids[@]}" | jq -R . | jq -s .)}"
        exit 1
    }

    # Parse results
    local unresolved=()
    local failed=()
    idx=0
    for tid in "${thread_ids[@]}"; do
        local is_resolved
        is_resolved=$(echo "$result" | jq -r "if .t${idx}.thread.isResolved == false then \"false\" else \"true\" end")
        if [ "$is_resolved" = "false" ]; then
            unresolved+=("$tid")
        else
            failed+=("$tid")
        fi
        idx=$((idx + 1))
    done

    # Output
    local unresolved_json failed_json
    unresolved_json=$(printf '%s\n' "${unresolved[@]:-}" | jq -R 'select(length > 0)' | jq -s . 2>/dev/null || echo '[]')
    failed_json=$(printf '%s\n' "${failed[@]:-}" | jq -R 'select(length > 0)' | jq -s . 2>/dev/null || echo '[]')

    local success="true"
    [ ${#failed[@]} -gt 0 ] && success="false"

    echo "{\"success\": $success, \"unresolved\": $unresolved_json, \"failed\": $failed_json}"
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

unresolve_threads "$@"
