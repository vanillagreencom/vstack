#!/bin/bash
# GitHub API - Find a PR comment by pattern and author
# Usage: find-comment.sh <PR-number> --pattern <regex> [--author <login>]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Find PR Comment

Usage: find-comment.sh <PR-number> --pattern <regex> [--author <login>]

Arguments:
  PR-number    PR number (required)

Options:
  --pattern    Regex pattern to match in comment body (required)
  --author     Filter by author login (optional)

Output:
{
  "id": 12345678,
  "author": "username",
  "body": "comment text...",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z",
  "url": "https://github.com/..."
}

Returns last matching comment. Empty object {} if no match.

Examples:
  # Find summary comment by current user
  find-comment.sh 23 --pattern "Recommendations.*Processed" --author "\$(gh api user -q .login)"

  # Find any comment with pattern
  find-comment.sh 23 --pattern "LGTM"
EOF
}

find_comment() {
    local pr_num=""
    local pattern=""
    local author=""

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --pattern)
                pattern="$2"
                shift 2
                ;;
            --pattern=*)
                pattern="${1#--pattern=}"
                shift
                ;;
            --author)
                author="$2"
                shift 2
                ;;
            --author=*)
                author="${1#--author=}"
                shift
                ;;
            *)
                if [ -z "$pr_num" ]; then
                    pr_num="$1"
                else
                    echo "{\"error\": \"Unexpected argument: $1\"}" >&2
                    exit 1
                fi
                shift
                ;;
        esac
    done

    if [ -z "$pr_num" ]; then
        echo '{"error": "PR number required"}' >&2
        exit 1
    fi

    if [ -z "$pattern" ]; then
        echo '{"error": "--pattern required"}' >&2
        exit 1
    fi

    # Get repo info
    local repo_info
    repo_info=$(get_repo_info) || exit 1
    local owner repo
    owner=$(get_owner "$repo_info")
    repo=$(get_repo "$repo_info")

    # Fetch comments
    local comments
    comments=$(gh_rest "repos/$owner/$repo/issues/$pr_num/comments") || exit 1

    # Build jq filter
    local jq_filter='[.[]'

    if [ -n "$author" ]; then
        jq_filter+=" | select(.user.login == \"$author\")"
    fi

    jq_filter+=" | select(.body | test(\"$pattern\"))"
    jq_filter+='] | last'
    jq_filter+=' | if . then {id: .id, author: .user.login, body: .body, created_at: .created_at, updated_at: .updated_at, url: .html_url} else {} end'

    echo "$comments" | jq -c "$jq_filter"
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

find_comment "$@"
