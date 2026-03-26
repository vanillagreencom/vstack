#!/bin/bash
# GitHub API - Get PR review threads
# Usage: pr-threads.sh [PR-number] [--unresolved] [--format=safe|raw]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Get PR Review Threads

Usage: pr-threads.sh [PR-number|branch] [options]

Arguments:
  PR-number|branch    PR number or branch name (default: current branch's PR)

Options:
  --unresolved        Only show unresolved threads
  --resolved          Only show resolved threads
  --format=safe       Normalized structure with count (DEFAULT)
  --format=raw        Original GitHub API structure

Output (safe format):
{
  "count": 3,
  "unresolved_count": 2,
  "threads": [{
    "id": "PRRT_...",
    "is_resolved": false,
    "is_outdated": false,
    "path": "src/file.rs",
    "line": 42,
    "author": "reviewer",
    "body": "First comment text"
  }]
}

Examples:
  pr-threads.sh 23
  pr-threads.sh 23 --unresolved
  pr-threads.sh --format=raw
EOF
}

get_pr_threads() {
    local pr_ref=""
    local filter_unresolved="false"
    local filter_resolved="false"
    FORMAT="${DEFAULT_FORMAT}"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --unresolved)
                filter_unresolved="true"
                shift
                ;;
            --resolved)
                filter_resolved="true"
                shift
                ;;
            --format=*)
                FORMAT="${1#--format=}"
                shift
                ;;
            --format)
                FORMAT="$2"
                shift 2
                ;;
            *)
                pr_ref="$1"
                shift
                ;;
        esac
    done

    # Resolve PR number
    local pr_num
    pr_num=$(resolve_pr_number "$pr_ref") || exit 1

    # Get repo info
    local repo_info
    repo_info=$(get_repo_info) || exit 1
    local owner repo
    owner=$(get_owner "$repo_info")
    repo=$(get_repo "$repo_info")

    # GraphQL query
    local query='
query($owner: String!, $repo: String!, $number: Int!) {
  repository(owner: $owner, name: $repo) {
    pullRequest(number: $number) {
      reviewThreads(first: 100) {
        nodes {
          id
          isResolved
          isOutdated
          path
          line
          comments(first: 1) {
            nodes {
              author { login }
              body
            }
          }
        }
      }
    }
  }
}'

    local result
    result=$(gh_graphql "$query" -F owner="$owner" -F repo="$repo" -F number="$pr_num") || exit 1

    # Apply format and filters
    case "$FORMAT" in
        raw)
            echo "$result"
            ;;
        safe|*)
            local jq_filter
            if [ "$filter_unresolved" = "true" ]; then
                jq_filter='select(.isResolved == false)'
            elif [ "$filter_resolved" = "true" ]; then
                jq_filter='select(.isResolved == true)'
            else
                jq_filter='.'
            fi

            echo "$result" | jq --arg filter "$jq_filter" '{
                count: ([.repository.pullRequest.reviewThreads.nodes[] | '"$jq_filter"'] | length),
                unresolved_count: ([.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved == false)] | length),
                threads: [.repository.pullRequest.reviewThreads.nodes[] | '"$jq_filter"' | {
                    id: .id,
                    is_resolved: .isResolved,
                    is_outdated: .isOutdated,
                    path: (.path // ""),
                    line: (.line // null),
                    author: (.comments.nodes[0].author.login // ""),
                    body: (.comments.nodes[0].body // "")
                }]
            }'
            ;;
    esac
}

# Main
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    show_help
    exit 0
fi

get_pr_threads "$@"
