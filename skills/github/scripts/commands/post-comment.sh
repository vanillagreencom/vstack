#!/bin/bash
# GitHub API - Post a PR-level comment
# Usage: post-comment.sh <PR-number> <body>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Post PR-Level Comment

Usage: post-comment.sh <PR-number> <body>

Arguments:
  PR-number    PR number (or branch name, or empty for current branch)
  body         Comment text

Options:
  --dry-run    Show what would be posted without executing

Output:
{
  "success": true,
  "url": "https://github.com/.../issuecomment-..."
}

Examples:
  # Post comment to PR
  post-comment.sh 23 "Addressed all feedback"

  # Current branch's PR
  post-comment.sh "" "Changes pushed"

  # Dry run
  post-comment.sh 23 "Comment text" --dry-run

Note: PR-level comments appear in the Conversation tab, not as review thread replies.
EOF
}

post_comment() {
    local pr_ref=""
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
                if [ -z "$pr_ref" ] || [ "$pr_ref" = "--" ]; then
                    pr_ref="$1"
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

    if [ -z "$body" ]; then
        echo '{"error": "Comment body required"}' >&2
        exit 1
    fi

    # Resolve PR number
    local pr_num
    pr_num=$(resolve_pr_number "$pr_ref") || exit 1

    # Get repo info
    local repo_info
    repo_info=$(get_repo_info) || exit 1
    local owner repo
    owner=$(get_owner "$repo_info")
    repo=$(get_repo "$repo_info")

    # Dry run
    if [ "$dry_run" = "true" ]; then
        echo "{\"dry_run\": true, \"pr\": $pr_num, \"body\": $(echo "$body" | jq -Rs .)}"
        exit 0
    fi

    # Post comment via REST API (issues endpoint works for PRs)
    local result
    result=$(gh_rest "repos/$owner/$repo/issues/$pr_num/comments" \
        -f body="$body") || exit 1

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

post_comment "$@"
