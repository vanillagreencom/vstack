#!/bin/bash
# GitHub API - Common functions via gh CLI
# Source this file in command scripts

set -euo pipefail

# Configuration
DEFAULT_FORMAT="safe" # safe, raw

json_or_default() {
    local fallback="$1"
    local expected_type="$2"
    shift 2

    local output=""
    if ! output=$("$@" 2>&1); then
        printf 'Command failed: %s\n' "$*" >&2
        if [[ -n "$output" ]]; then
            printf '%s\n' "$output" >&2
        fi
        printf '%s' "$fallback"
        return 1
    fi

    if ! jq -e --arg type "$expected_type" 'type == $type' >/dev/null 2>&1 <<<"$output"; then
        printf 'Invalid JSON response (expected %s): %s\n' "$expected_type" "$*" >&2
        if [[ -n "$output" ]]; then
            printf '%s\n' "$output" >&2
        fi
        printf '%s' "$fallback"
        return 2
    fi

    printf '%s' "$output"
}

# Internal lib directory (underscore prefix avoids overwriting caller's SCRIPT_DIR)
_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"

# Get repository owner and name from current git context
get_repo_info() {
    local info
    info=$(gh repo view --json owner,name 2>/dev/null) || {
        echo '{"error": "Not in a GitHub repository or gh not authenticated"}' >&2
        return 1
    }
    echo "$info"
}

# Extract owner from repo info
get_owner() {
    local repo_info="${1:-}"
    if [ -z "$repo_info" ]; then
        repo_info=$(get_repo_info) || return 1
    fi
    echo "$repo_info" | jq -r '.owner.login'
}

# Extract repo name from repo info
get_repo() {
    local repo_info="${1:-}"
    if [ -z "$repo_info" ]; then
        repo_info=$(get_repo_info) || return 1
    fi
    echo "$repo_info" | jq -r '.name'
}

# Collect local branch names for default PR list scoping.
# Shared refs mean branches checked out in linked worktrees also appear here.
get_local_branch_names_json() {
    local branches
    branches=$(git for-each-ref refs/heads --format='%(refname:short)' 2>/dev/null || true)

    if [ -z "$branches" ]; then
        printf '[]'
        return 0
    fi

    printf '%s\n' "$branches" | jq -R . | jq -s 'map(select(length > 0))'
}

# Default PR list scope:
#   1. repo-local branches (plain issue branches like proj-117)
#   2. current user's namespaced branches (user/proj-117)
filter_prs_to_default_scope() {
    local prs_json="$1"
    local gh_user="${2:-}"
    local local_branches_json="${3:-[]}"

    jq \
        --arg gh_user "$gh_user" \
        --argjson local_branches "$local_branches_json" \
        '
        map(
            .headRefName as $head |
            select(
                (($local_branches | index($head)) != null) or
                ($gh_user != "" and ($head | startswith($gh_user + "/")))
            )
        )
        ' <<<"$prs_json"
}

# Check gh CLI authentication
check_gh_auth() {
    if ! gh auth status &>/dev/null; then
        echo '{"error": "gh CLI not authenticated. Run: gh auth login"}' >&2
        return 1
    fi
}

# Execute GraphQL query with error handling and retry
# Usage: graphql_query "query { ... }" '{"var": "value"}'
# Or with -F variables: graphql_query "query($x: Type!) { ... }" -F x="value"
gh_graphql() {
    local query="$1"
    shift
    local max_retries=3
    local retry_delay=1
    local attempt=1

    check_gh_auth || return 1

    while [ $attempt -le $max_retries ]; do
        local response
        local stderr_output
        local exit_code=0

        # Execute query - capture stdout and stderr separately
        response=$(gh api graphql -f query="$query" "$@" 2>/dev/null) || exit_code=$?

        if [ $exit_code -eq 0 ]; then
            # Check for GraphQL errors
            local errors
            errors=$(echo "$response" | jq -r '.errors // empty' 2>/dev/null)
            if [ -n "$errors" ] && [ "$errors" != "null" ]; then
                local error_type error_msg
                error_type=$(echo "$response" | jq -r '.errors[0].type // ""')
                error_msg=$(echo "$response" | jq -r '.errors[0].message // "Unknown GraphQL error"')

                # Translate common errors
                case "$error_type" in
                NOT_FOUND)
                    echo '{"error": "Not found"}' >&2
                    ;;
                *)
                    echo "{\"error\": \"GraphQL: $error_msg\"}" >&2
                    ;;
                esac
                return 1
            fi
            # Success - return data
            echo "$response" | jq -c '.data'
            return 0
        fi

        # Non-zero exit - check if we got JSON with errors
        if [ -n "$response" ]; then
            local errors
            errors=$(echo "$response" | jq -r '.errors // empty' 2>/dev/null)
            if [ -n "$errors" ] && [ "$errors" != "null" ]; then
                local error_type error_msg
                error_type=$(echo "$response" | jq -r '.errors[0].type // ""')
                error_msg=$(echo "$response" | jq -r '.errors[0].message // "Unknown error"')

                case "$error_type" in
                NOT_FOUND)
                    echo '{"error": "Not found"}' >&2
                    ;;
                *)
                    echo "{\"error\": \"$error_msg\"}" >&2
                    ;;
                esac
                return 1
            fi
        fi

        # Handle HTTP/network errors
        if [ $attempt -lt $max_retries ]; then
            sleep $retry_delay
            retry_delay=$((retry_delay * 2))
            attempt=$((attempt + 1))
            continue
        fi

        echo '{"error": "GitHub API request failed"}' >&2
        return 1
    done
}

# Execute REST API call with error handling
# Usage: gh_rest "repos/{owner}/{repo}/pulls/123"
gh_rest() {
    local endpoint="$1"
    shift
    local max_retries=3
    local retry_delay=1
    local attempt=1

    check_gh_auth || return 1

    while [ $attempt -le $max_retries ]; do
        local response
        local exit_code=0

        response=$(gh api "$endpoint" "$@" 2>&1) || exit_code=$?

        if [ $exit_code -eq 0 ]; then
            echo "$response"
            return 0
        fi

        # Handle errors
        case "$response" in
        *"401"* | *"Unauthorized"*)
            echo '{"error": "GitHub authentication failed"}' >&2
            return 1
            ;;
        *"403"* | *"rate limit"*)
            if [ $attempt -lt $max_retries ]; then
                sleep $retry_delay
                retry_delay=$((retry_delay * 2))
                attempt=$((attempt + 1))
                continue
            fi
            echo '{"error": "GitHub rate limited"}' >&2
            return 1
            ;;
        *"404"* | *"Not Found"*)
            echo '{"error": "Resource not found"}' >&2
            return 1
            ;;
        *)
            if [ $attempt -lt $max_retries ]; then
                sleep $retry_delay
                retry_delay=$((retry_delay * 2))
                attempt=$((attempt + 1))
                continue
            fi
            local clean_error
            clean_error=$(echo "$response" | head -c 200 | tr '\n' ' ')
            echo "{\"error\": \"$clean_error\"}" >&2
            return 1
            ;;
        esac
    done
}

# Parse --format argument from args
# Usage: remaining_args=$(parse_format_arg "$@")
# Sets FORMAT global variable
parse_format_arg() {
    FORMAT="${DEFAULT_FORMAT}"
    local remaining_args=()

    while [[ $# -gt 0 ]]; do
        case "$1" in
        --format)
            FORMAT="$2"
            shift 2
            ;;
        --format=*)
            FORMAT="${1#--format=}"
            shift
            ;;
        *)
            remaining_args+=("$1")
            shift
            ;;
        esac
    done

    # Validate format
    case "$FORMAT" in
    safe | raw) ;;
    *)
        echo "{\"error\": \"Invalid format: $FORMAT. Use: safe, raw\"}" >&2
        return 1
        ;;
    esac

    echo "${remaining_args[@]:-}"
}

# Load and validate bot token from .env.local
# Supports direct tokens (ghp_*, gho_*, ghs_*, ghr_*) and 1Password references (op://...)
# Returns: token string if valid, empty string if not configured/invalid
# Outputs: diagnostic messages to stderr
load_bot_token() {
    local token=""
    local env_file="$PROJECT_ROOT/.env.local"

    # Load from .env.local if exists
    if [ -f "$env_file" ]; then
        # shellcheck disable=SC1090
        source "$env_file"
        token="${GH_BOT_TOKEN:-}"
    fi

    # Empty token - not configured
    if [ -z "$token" ]; then
        return 0
    fi

    # Check for 1Password reference
    if [[ "$token" == op://* ]]; then
        if command -v op &>/dev/null; then
            local resolved
            if resolved=$(op read "$token" 2>/dev/null); then
                token="$resolved"
            else
                echo "Warning: Failed to resolve 1Password reference. Run: op signin" >&2
                return 0
            fi
        else
            echo "Warning: GH_BOT_TOKEN is a 1Password reference but 'op' CLI not found" >&2
            echo "  Install: https://developer.1password.com/docs/cli/get-started/" >&2
            return 0
        fi
    fi

    # Validate GitHub token format
    # Classic: ghp_, gho_, ghs_, ghr_
    # Fine-grained: github_pat_
    if [[ "$token" =~ ^gh[pors]_ ]] || [[ "$token" =~ ^github_pat_ ]]; then
        echo "$token"
        return 0
    fi

    # Invalid format
    echo "Warning: GH_BOT_TOKEN has invalid format (expected ghp_*, gho_*, ghs_*, ghr_*, or github_pat_*)" >&2
    echo "  Current value starts with: ${token:0:4}..." >&2
    echo "  Fix: Update .env.local with a valid GitHub token" >&2
    return 0
}

# Get formal review verdict from GitHub API (primary source - structured data)
# The bot submits a formal GitHub review (APPROVED/CHANGES_REQUESTED) which is
# the most reliable verdict signal. Falls back to empty string if no formal review.
# Usage: get_formal_review_verdict <PR#>
# Returns: "approved", "changes", or "" (no formal review found)
get_formal_review_verdict() {
    local pr="$1"
    local bot_user="${GH_BOT_USERNAME:-review-bot[bot]}"
    local review_state
    review_state=$(gh api "repos/{owner}/{repo}/pulls/$pr/reviews" \
        --jq "[.[] | select(.user.login == \"$bot_user\")] | last | .state // empty" 2>/dev/null || echo "")
    case "$review_state" in
    APPROVED) echo "approved" ;;
    CHANGES_REQUESTED) echo "changes" ;;
    *) echo "" ;;
    esac
}

# Check if bot token is configured and valid
# Usage: check_bot_token [format]
# format: safe (default), text
check_bot_token() {
    local format="${1:-safe}"
    local token
    token=$(load_bot_token 2>/dev/null)

    case "$format" in
    safe | json | true) # "true" for backward compat with old boolean param
        if [ -n "$token" ]; then
            echo '{"configured": true, "valid": true}'
        else
            echo '{"configured": false, "valid": false}'
        fi
        ;;
    text | false) # "false" for backward compat
        if [ -n "$token" ]; then
            echo "configured"
        else
            echo "not configured"
        fi
        ;;
    *)
        echo "Error: Unknown format: $format. Use: safe, text" >&2
        return 1
        ;;
    esac
}

# Get PR number from current branch
get_current_pr() {
    local pr_json
    pr_json=$(gh pr view --json number 2>/dev/null) || {
        echo '{"error": "No PR found for current branch"}' >&2
        return 1
    }
    echo "$pr_json" | jq -r '.number'
}

# Resolve PR reference (number, branch, or current)
# Usage: resolve_pr_number "23" or "feature-branch" or ""
resolve_pr_number() {
    local ref="${1:-}"

    if [ -z "$ref" ]; then
        get_current_pr
        return
    fi

    # If numeric, return as-is
    if [[ "$ref" =~ ^[0-9]+$ ]]; then
        echo "$ref"
        return
    fi

    # Try to find PR by branch name
    local pr_json
    pr_json=$(gh pr view "$ref" --json number 2>/dev/null) || {
        echo "{\"error\": \"No PR found for: $ref\"}" >&2
        return 1
    }
    echo "$pr_json" | jq -r '.number'
}
