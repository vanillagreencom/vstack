#!/bin/bash
# GitHub API - Dismiss a PR review
# Usage: dismiss-review.sh <PR> [--bot] [--message "reason"]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Dismiss PR Review

Usage: dismiss-review.sh <PR> [options]

Arguments:
  PR             PR number

Options:
  --bot          Dismiss reviews from claude[bot] (default: all CHANGES_REQUESTED)
  --user <name>  Dismiss reviews from specific user
  --message <m>  Dismissal reason (default: "Contested with rationale")
  --dry-run      Show what would be dismissed without executing

Output:
{
  "success": true,
  "dismissed": [{"review_id": 123, "user": "claude[bot]", "state": "CHANGES_REQUESTED"}],
  "skipped": []
}

Examples:
  # Dismiss bot's blocking review
  dismiss-review.sh 473 --bot

  # Dismiss with custom message
  dismiss-review.sh 473 --bot --message "Core.Tests only targets net10.0"

  # Dismiss specific user's review
  dismiss-review.sh 473 --user "reviewer-name"

  # Dry run
  dismiss-review.sh 473 --bot --dry-run
EOF
}

dismiss_reviews() {
    local pr_number=""
    local filter_bot="false"
    local filter_user=""
    local message="Contested with rationale"
    local dry_run="false"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --bot)
                filter_bot="true"
                shift
                ;;
            --user)
                filter_user="$2"
                shift 2
                ;;
            --message)
                message="$2"
                shift 2
                ;;
            --dry-run)
                dry_run="true"
                shift
                ;;
            *)
                if [[ "$1" =~ ^[0-9]+$ ]] && [ -z "$pr_number" ]; then
                    pr_number="$1"
                else
                    echo "{\"error\": \"Unknown argument: $1\"}" >&2
                    exit 1
                fi
                shift
                ;;
        esac
    done

    if [ -z "$pr_number" ]; then
        pr_number=$(get_current_pr) || exit 1
    fi

    # Get repo info
    local repo_info owner repo
    repo_info=$(get_repo_info) || exit 1
    owner=$(get_owner "$repo_info")
    repo=$(get_repo "$repo_info")

    # Fetch reviews
    local reviews
    reviews=$(gh_rest "repos/$owner/$repo/pulls/$pr_number/reviews") || exit 1

    # Filter to CHANGES_REQUESTED reviews
    local blocking_reviews
    blocking_reviews=$(echo "$reviews" | jq -c '[.[] | select(.state == "CHANGES_REQUESTED")]')

    # Apply user filter
    if [ "$filter_bot" = "true" ]; then
        blocking_reviews=$(echo "$blocking_reviews" | jq -c '[.[] | select(.user.login == "claude[bot]")]')
    elif [ -n "$filter_user" ]; then
        blocking_reviews=$(echo "$blocking_reviews" | jq -c --arg user "$filter_user" '[.[] | select(.user.login == $user)]')
    fi

    local count
    count=$(echo "$blocking_reviews" | jq 'length')

    if [ "$count" -eq 0 ]; then
        echo '{"success": true, "dismissed": [], "skipped": [], "message": "No matching CHANGES_REQUESTED reviews found"}'
        return
    fi

    # Dry run
    if [ "$dry_run" = "true" ]; then
        echo "$blocking_reviews" | jq -c "{dry_run: true, would_dismiss: [.[] | {review_id: .id, user: .user.login, state: .state}]}"
        return
    fi

    # Dismiss each review — collect results as JSON lines
    local results_file
    results_file=$(mktemp)
    trap "rm -f '$results_file'" EXIT

    local review_ids
    review_ids=$(echo "$blocking_reviews" | jq -r '.[].id')

    for review_id in $review_ids; do
        local user_login
        user_login=$(echo "$blocking_reviews" | jq -r --argjson id "$review_id" '.[] | select(.id == $id) | .user.login')

        local exit_code=0
        gh api "repos/$owner/$repo/pulls/$pr_number/reviews/$review_id/dismissals" \
            -X PUT -f message="$message" -f event="DISMISS" >/dev/null 2>&1 || exit_code=$?

        if [ $exit_code -eq 0 ]; then
            echo "{\"review_id\":$review_id,\"user\":\"$user_login\",\"state\":\"DISMISSED\",\"ok\":true}" >> "$results_file"
        else
            echo "{\"review_id\":$review_id,\"user\":\"$user_login\",\"error\":\"dismiss failed\",\"ok\":false}" >> "$results_file"
        fi
    done

    # Build output from results file
    local dismissed_json failed_json
    dismissed_json=$(jq -s '[.[] | select(.ok == true) | del(.ok)]' "$results_file" 2>/dev/null || echo '[]')
    failed_json=$(jq -s '[.[] | select(.ok == false) | del(.ok)]' "$results_file" 2>/dev/null || echo '[]')

    local has_failures
    has_failures=$(echo "$failed_json" | jq 'length > 0')
    local success="true"
    [ "$has_failures" = "true" ] && success="false"

    echo "{\"success\": $success, \"dismissed\": $dismissed_json, \"failed\": $failed_json}"
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

dismiss_reviews "$@"
