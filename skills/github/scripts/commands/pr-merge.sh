#!/bin/bash
# Merge PR as bot account with safety checks
# Usage: pr-merge <PR_NUMBER> [--check] [--force] [--dry-run]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source shared library for load_bot_token (also sets PROJECT_ROOT)
# shellcheck disable=SC1091
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat <<'EOF'
Merge PR as bot account with safety checks

Usage: pr-merge <PR_NUMBER> [options]

Options:
  --squash         Squash and merge (default)
  --merge          Create merge commit
  --rebase         Rebase and merge
  --delete-branch  Delete branch after merge (default: true)
  --keep-branch    Keep branch after merge
  --check          Run checks only, output JSON, don't merge
  --force          Skip checks and merge (requires explicit user decision)
  --dry-run        Show what would happen without merging

Modes:
  (default)        Run checks, block if critical issues, merge if pass
  --check          Run checks, output JSON for workflow to parse
  --force          Skip all checks, merge immediately

Examples:
  github.sh pr-merge 42 --check   # Check only, workflow handles result
  github.sh pr-merge 42           # Check + merge if pass
  github.sh pr-merge 42 --force   # Skip checks, merge (DANGEROUS)
EOF
}

# Run safety checks, output JSON
run_checks() {
    local pr_num="$1"
    local can_merge=true
    local issues=()
    local warnings=()

    # Check PR exists first (use title - number alone doesn't validate)
    if ! gh pr view "$pr_num" --json title >/dev/null 2>&1; then
        jq -n '{can_merge: false, issues: ["not_found: PR #'"$pr_num"' not found"], warnings: [], mergeable: "UNKNOWN", review: ""}'
        return 0 # Return 0 so JSON is output, caller checks can_merge
    fi

    # 1. Check mergeable status
    local mergeable
    mergeable=$(gh pr view "$pr_num" --json mergeable --jq '.mergeable' 2>/dev/null || echo "UNKNOWN")
    if [ "$mergeable" = "MERGEABLE" ]; then
        : # ok
    elif [ "$mergeable" = "CONFLICTING" ]; then
        can_merge=false
        issues+=("conflicts: PR has merge conflicts. Resolve by rebasing onto your default branch and force-pushing")
    else
        can_merge=false
        issues+=("unknown: GitHub still computing mergeable status, retry in a few seconds")
    fi

    # 2. Check CI status
    local ci_json ci_pass
    if ! ci_json=$(gh pr checks "$pr_num" --json name,state 2>&1); then
        can_merge=false
        issues+=("ci_fetch_failed: Failed to fetch CI checks from GitHub")
    elif ! jq -e 'type == "array"' >/dev/null 2>&1 <<<"$ci_json"; then
        can_merge=false
        issues+=("ci_fetch_failed: Invalid CI response from GitHub")
    elif [ "$(echo "$ci_json" | jq 'length')" -eq 0 ]; then
        warnings+=("ci_unconfigured: No status checks configured")
    else
        ci_pass=$(echo "$ci_json" | jq 'all(.state == "SUCCESS" or .state == "SKIPPED")')
        if [ "$ci_pass" != "true" ]; then
            local failed
            failed=$(echo "$ci_json" | jq -r '[.[] | select(.state != "SUCCESS" and .state != "SKIPPED")] | map(.name + " (" + .state + ")") | join(", ")')
            can_merge=false
            issues+=("ci_failed: $failed")
        fi
    fi

    # 3. Check unresolved threads
    local unresolved
    unresolved=$("$SCRIPT_DIR/pr-threads.sh" "$pr_num" --unresolved 2>/dev/null | jq -r '.unresolved_count // 0')
    if [ "$unresolved" != "0" ]; then
        warnings+=("unresolved_threads: $unresolved thread(s) need attention")
    fi

    # 4. Check review status
    # reviewDecision is only populated with branch protection rules requiring approvals.
    # Fall back to checking latestReviews for both APPROVED and CHANGES_REQUESTED states.
    local review="" has_approved_review=false has_changes_requested=false
    local review_json
    if ! review_json=$(json_or_default '{}' object gh pr view "$pr_num" --json reviewDecision,latestReviews); then
        can_merge=false
        issues+=("review_fetch_failed: Failed to fetch review status from GitHub")
    else
        review=$(echo "$review_json" | jq -r '.reviewDecision // ""')
        has_approved_review=$(echo "$review_json" | jq '[.latestReviews[] | select(.state == "APPROVED")] | length > 0')
        has_changes_requested=$(echo "$review_json" | jq '[.latestReviews[] | select(.state == "CHANGES_REQUESTED")] | length > 0')

        if [ "$review" = "CHANGES_REQUESTED" ] || [ "$has_changes_requested" = "true" ]; then
            can_merge=false
            issues+=("changes_requested: Reviewer requested changes")
        elif [ "$review" != "APPROVED" ] && [ "$has_approved_review" != "true" ]; then
            warnings+=("not_approved: Review status is '$review'")
        fi
    fi

    # Output JSON
    local issues_json warnings_json
    issues_json=$(printf '%s\n' "${issues[@]:-}" | jq -R -s -c 'split("\n") | map(select(. != ""))')
    warnings_json=$(printf '%s\n' "${warnings[@]:-}" | jq -R -s -c 'split("\n") | map(select(. != ""))')

    jq -n \
        --argjson can_merge "$can_merge" \
        --argjson issues "$issues_json" \
        --argjson warnings "$warnings_json" \
        --arg mergeable "$mergeable" \
        --arg review "$review" \
        '{can_merge: $can_merge, issues: $issues, warnings: $warnings, mergeable: $mergeable, review: $review}'
}

main() {
    local pr_num="" method="--squash" delete_branch=true
    local check_only=false force=false dry_run=false

    while [ $# -gt 0 ]; do
        case "$1" in
        --squash)
            method="--squash"
            shift
            ;;
        --merge)
            method="--merge"
            shift
            ;;
        --rebase)
            method="--rebase"
            shift
            ;;
        --delete-branch)
            delete_branch=true
            shift
            ;;
        --keep-branch)
            delete_branch=false
            shift
            ;;
        --check)
            check_only=true
            shift
            ;;
        --force)
            force=true
            shift
            ;;
        --dry-run)
            dry_run=true
            shift
            ;;
        --help | -h)
            show_help
            exit 0
            ;;
        [0-9]*)
            pr_num="$1"
            shift
            ;;
        *)
            echo "Error: Unknown option: $1" >&2
            exit 1
            ;;
        esac
    done

    if [ -z "$pr_num" ]; then
        echo '{"error": "PR number required"}' >&2
        exit 1
    fi

    # Check-only mode: output JSON and exit
    if [ "$check_only" = true ]; then
        run_checks "$pr_num"
        exit 0
    fi

    local token
    token=$(load_bot_token)

    # Unless --force, run checks
    if [ "$force" = false ]; then
        local check_result can_merge
        check_result=$(run_checks "$pr_num")
        can_merge=$(echo "$check_result" | jq -r '.can_merge')

        if [ "$can_merge" != "true" ]; then
            echo "Safety checks failed:" >&2
            echo "$check_result" | jq -r '.issues[]' | sed 's/^/  ✗ /' >&2
            echo "$check_result" | jq -r '.warnings[]' | sed 's/^/  ⚠ /' >&2
            echo "" >&2
            echo "Use --force to merge anyway (requires explicit decision)." >&2
            exit 1
        fi

        # Show warnings even on success
        local warnings
        warnings=$(echo "$check_result" | jq -r '.warnings | length')
        if [ "$warnings" -gt 0 ]; then
            echo "Warnings:" >&2
            echo "$check_result" | jq -r '.warnings[]' | sed 's/^/  ⚠ /' >&2
        fi
    else
        echo "⚠ --force: Skipping safety checks" >&2
    fi

    if [ "$dry_run" = true ]; then
        local token_status="not configured"
        [ -n "$token" ] && token_status="configured"
        echo "Would merge PR #$pr_num ($method, delete_branch=$delete_branch, token=$token_status)"
        exit 0
    fi

    # Execute merge (never pass --delete-branch to gh; it tries local git
    # checkout which fails inside worktrees). Delete remote branch via API.
    local -a cmd=(gh pr merge "$pr_num" "$method")

    if [ -n "$token" ]; then
        GH_TOKEN="$token" "${cmd[@]}"
    else
        echo "Warning: GH_BOT_TOKEN not configured, using current user" >&2
        "${cmd[@]}"
    fi

    # Delete remote branch via API (avoids gh's local git checkout)
    if [ "$delete_branch" = true ]; then
        local branch
        branch=$(gh pr view "$pr_num" --json headRefName --jq '.headRefName' 2>/dev/null || true)
        if [ -n "$branch" ]; then
            gh api -X DELETE "repos/{owner}/{repo}/git/refs/heads/$branch" 2>/dev/null || true
        fi
    fi
}

main "$@"
