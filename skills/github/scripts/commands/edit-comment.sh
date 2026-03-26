#!/bin/bash
# GitHub API - Edit an existing PR comment
# Usage: edit-comment.sh <comment-id> <body>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Edit PR Comment

Usage: edit-comment.sh <comment-id> <body>

Arguments:
  comment-id   Numeric comment ID (from find-comment or URL)
  body         New comment text

Options:
  --dry-run    Show what would be edited without executing

Output:
{
  "success": true,
  "url": "https://github.com/..."
}

Examples:
  # Edit a comment
  edit-comment.sh 12345678 "Updated content"

  # Dry run
  edit-comment.sh 12345678 "New text" --dry-run

Note: Get comment ID from find-comment or from GitHub URL (#issuecomment-<ID>)
EOF
}

edit_comment() {
    local comment_id=""
    local body=""
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
            *)
                if [ -z "$comment_id" ]; then
                    comment_id="$1"
                elif [ -z "$body" ]; then
                    body="$1"
                else
                    echo "{\"error\": \"Unexpected argument: $1\"}" >&2
                    exit 1
                fi
                shift
                ;;
        esac
    done

    if [ -z "$comment_id" ]; then
        echo '{"error": "Comment ID required"}' >&2
        exit 1
    fi

    if [ -z "$body" ]; then
        echo '{"error": "Comment body required"}' >&2
        exit 1
    fi

    # Validate comment ID is numeric
    if ! [[ "$comment_id" =~ ^[0-9]+$ ]]; then
        echo "{\"error\": \"Comment ID must be numeric: $comment_id\"}" >&2
        exit 1
    fi

    # Get repo info
    local repo_info
    repo_info=$(get_repo_info) || exit 1
    local owner repo
    owner=$(get_owner "$repo_info")
    repo=$(get_repo "$repo_info")

    # Dry run
    if [ "$dry_run" = "true" ]; then
        echo "{\"dry_run\": true, \"comment_id\": $comment_id, \"body_preview\": $(echo "$body" | head -c 100 | jq -Rs .)}"
        exit 0
    fi

    # Edit comment via REST API
    local result
    result=$(gh api -X PATCH "repos/$owner/$repo/issues/comments/$comment_id" \
        -f body="$body" 2>&1) || {
        echo "{\"error\": \"Failed to edit comment: $result\"}" >&2
        exit 1
    }

    # Extract URL from response
    local url
    url=$(echo "$result" | jq -r '.html_url // .url // ""')

    if [ -n "$url" ]; then
        echo "{\"success\": true, \"url\": \"$url\"}"
    else
        echo '{"success": true, "url": null}'
    fi
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

edit_comment "$@"
