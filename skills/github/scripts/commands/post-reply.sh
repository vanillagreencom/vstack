#!/bin/bash
# GitHub API - Reply to a review thread or comment
# Usage: post-reply.sh <thread-id|comment-id> <body> [--pr <N>]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Reply to Review Thread or Comment

Usage: post-reply.sh <id> <body> [options]

Arguments:
  id            Thread ID (PRRT_...) or numeric comment ID
  body          Reply text

Options:
  --pr <N>      PR number (required for numeric comment ID, ignored for thread ID)
  --dry-run     Show what would be posted without executing

Output:
{
  "success": true,
  "url": "https://github.com/.../discussion_r..."
}

Examples:
  # Reply to a thread (preferred - uses GraphQL)
  post-reply.sh PRRT_kwDOQchwXs5n... "Thanks, fixed!"

  # Reply to a comment by numeric ID (legacy - uses REST)
  post-reply.sh 2633519824 "Fixed!" --pr 23

  # Dry run
  post-reply.sh PRRT_kwDOQchwXs5n... "Fixed!" --dry-run
EOF
}

# Reply using GraphQL (for thread IDs)
reply_via_graphql() {
    local thread_id="$1"
    local body="$2"

    local query='mutation($threadId: ID!, $body: String!) {
        addPullRequestReviewThreadReply(input: {
            pullRequestReviewThreadId: $threadId
            body: $body
        }) {
            comment {
                id
                url
            }
        }
    }'

    local result
    result=$(gh_graphql "$query" -F threadId="$thread_id" -F body="$body") || return 1

    local url
    url=$(echo "$result" | jq -r '.addPullRequestReviewThreadReply.comment.url // ""')

    if [ -n "$url" ]; then
        echo "{\"success\": true, \"url\": \"$url\"}"
    else
        echo '{"success": true, "url": null}'
    fi
}

# Reply using REST API (for numeric comment IDs)
reply_via_rest() {
    local comment_id="$1"
    local body="$2"
    local pr_num="$3"
    local owner="$4"
    local repo="$5"

    local result
    result=$(gh_rest "repos/$owner/$repo/pulls/$pr_num/comments/$comment_id/replies" \
        -f body="$body") || return 1

    local url
    url=$(echo "$result" | jq -r '.html_url // .url // ""')

    if [ -n "$url" ]; then
        echo "{\"success\": true, \"url\": \"$url\"}"
    else
        echo '{"success": true, "url": null}'
    fi
}

post_reply() {
    local id=""
    local body=""
    local pr_ref=""
    local dry_run="false"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --pr)
                pr_ref="$2"
                shift 2
                ;;
            --dry-run)
                dry_run="true"
                shift
                ;;
            *)
                if [ -z "$id" ]; then
                    id="$1"
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

    if [ -z "$id" ]; then
        echo '{"error": "Thread/comment ID required"}' >&2
        exit 1
    fi

    if [ -z "$body" ]; then
        echo '{"error": "Reply body required"}' >&2
        exit 1
    fi

    # Detect ID type: thread ID starts with PRRT_, numeric is comment ID
    local is_thread_id="false"
    if [[ "$id" == PRRT_* ]]; then
        is_thread_id="true"
    fi

    # Dry run
    if [ "$dry_run" = "true" ]; then
        if [ "$is_thread_id" = "true" ]; then
            echo "{\"dry_run\": true, \"method\": \"graphql\", \"thread_id\": \"$id\", \"body\": $(echo "$body" | jq -Rs .)}"
        else
            local pr_num
            pr_num=$(resolve_pr_number "$pr_ref") || exit 1
            echo "{\"dry_run\": true, \"method\": \"rest\", \"pr\": $pr_num, \"comment_id\": \"$id\", \"body\": $(echo "$body" | jq -Rs .)}"
        fi
        exit 0
    fi

    # Execute based on ID type
    if [ "$is_thread_id" = "true" ]; then
        # GraphQL for thread IDs
        reply_via_graphql "$id" "$body"
    else
        # REST for numeric comment IDs
        local pr_num
        pr_num=$(resolve_pr_number "$pr_ref") || exit 1

        local repo_info
        repo_info=$(get_repo_info) || exit 1
        local owner repo
        owner=$(get_owner "$repo_info")
        repo=$(get_repo "$repo_info")

        reply_via_rest "$id" "$body" "$pr_num" "$owner" "$repo"
    fi
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

post_reply "$@"
