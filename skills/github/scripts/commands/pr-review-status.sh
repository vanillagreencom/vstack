#!/bin/bash
# Check PR review status and determine if action needed
# Usage: pr-review-status.sh <PR#> [--baseline-ts TS] [--baseline-threads N]
#
# Compares current state against optional baseline to detect changes.
# Used by orchestrator re-review loop to determine next action.
#
# Output JSON:
# {
#   "sticky_found": true,
#   "sticky_updated_at": "2024-01-05T...",
#   "verdict": "approved|changes|pending",
#   "unresolved_threads": 2,
#   "baseline_provided": true,
#   "changes_detected": true,
#   "needs_action": true,
#   "reason": "approved_clean|no_change|has_threads|verdict_not_approved|no_sticky"
# }
#
# Exit codes:
#   0 - Success (JSON output)
#   1 - Usage error or API failure

set -euo pipefail

CMD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$CMD_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Check PR Review Status

Usage: pr-review-status.sh <PR#> [options]

Arguments:
  PR#                    PR number (required)

Options:
  --baseline-ts TS       Previous sticky comment timestamp (ISO 8601)
  --baseline-threads N   Previous unresolved thread count
  --help                 Show this help

Output:
  JSON with current state and action recommendation.

Decision Logic:
  1. no_sticky        - No claude bot sticky found (caller decides)
  2. no_change        - Baseline provided, nothing changed → done
  3. approved_clean   - Verdict approved + 0 threads → done
  4. has_threads      - Unresolved threads exist → needs /review-pr-comments
  5. verdict_not_approved - Verdict is changes/pending → needs /review-pr-comments

Examples:
  # Initial check (no baseline)
  pr-review-status.sh 42

  # Check with baseline (in re-review loop)
  pr-review-status.sh 42 --baseline-ts "2024-01-05T12:00:00Z" --baseline-threads 3
EOF
}

# Parse arguments
PR_NUM=""
BASELINE_TS=""
BASELINE_THREADS=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --help|-h)
            show_help
            exit 0
            ;;
        --baseline-ts)
            BASELINE_TS="$2"
            shift 2
            ;;
        --baseline-threads)
            BASELINE_THREADS="$2"
            shift 2
            ;;
        *)
            if [[ -z "$PR_NUM" ]]; then
                PR_NUM="$1"
            fi
            shift
            ;;
    esac
done

if [[ -z "$PR_NUM" ]]; then
    echo '{"error": "Usage: pr-review-status.sh <PR#> [--baseline-ts TS] [--baseline-threads N]"}' >&2
    exit 1
fi

# Get sticky comment (with retry logic built into sticky-comment.sh)
STICKY_RESULT=$("$CMD_DIR/sticky-comment.sh" "$PR_NUM" 2>&1) || {
    # Check if it's "no sticky found" vs other error
    if echo "$STICKY_RESULT" | grep -q "No sticky comment found"; then
        jq -n '{
            sticky_found: false,
            sticky_updated_at: null,
            verdict: null,
            unresolved_threads: null,
            baseline_provided: false,
            changes_detected: false,
            needs_action: false,
            reason: "no_sticky"
        }'
        exit 0
    fi
    echo "$STICKY_RESULT" >&2
    exit 1
}

# Extract sticky data
STICKY_TS=$(echo "$STICKY_RESULT" | jq -r '.updated_at // empty')

# sticky-comment.sh already checks formal review first, then falls back to
# text parsing — use its computed verdict directly (avoids redundant API call)
VERDICT=$(echo "$STICKY_RESULT" | jq -r '.verdict // "pending"')

# Get unresolved thread count
THREADS_RESULT=$("$CMD_DIR/pr-threads.sh" "$PR_NUM" --unresolved 2>&1) || {
    echo "$THREADS_RESULT" >&2
    exit 1
}
UNRESOLVED_COUNT=$(echo "$THREADS_RESULT" | jq -r '.unresolved_count // 0')

# Determine if baseline was provided
BASELINE_PROVIDED="false"
if [[ -n "$BASELINE_TS" ]] || [[ -n "$BASELINE_THREADS" ]]; then
    BASELINE_PROVIDED="true"
fi

# Detect changes from baseline
CHANGES_DETECTED="false"
if [[ "$BASELINE_PROVIDED" == "true" ]]; then
    if [[ -n "$BASELINE_TS" ]] && [[ "$STICKY_TS" != "$BASELINE_TS" ]]; then
        CHANGES_DETECTED="true"
    fi
    if [[ -n "$BASELINE_THREADS" ]] && [[ "$UNRESOLVED_COUNT" != "$BASELINE_THREADS" ]]; then
        CHANGES_DETECTED="true"
    fi
fi

# Decision logic (matches original § 9.2 exactly)
# Order matters - evaluated top to bottom, first match wins
NEEDS_ACTION="false"
REASON=""

# Condition 5: No changes from baseline → done
if [[ "$BASELINE_PROVIDED" == "true" ]] && [[ "$CHANGES_DETECTED" == "false" ]]; then
    NEEDS_ACTION="false"
    REASON="no_change"
# Condition 6: Approved with 0 threads → done
elif [[ "$VERDICT" == "approved" ]] && [[ "$UNRESOLVED_COUNT" -eq 0 ]]; then
    NEEDS_ACTION="false"
    REASON="approved_clean"
# Condition 7a: Has unresolved threads → needs action
elif [[ "$UNRESOLVED_COUNT" -gt 0 ]]; then
    NEEDS_ACTION="true"
    REASON="has_threads"
# Condition 7b: Verdict is changes or pending → needs action
elif [[ "$VERDICT" == "changes" ]] || [[ "$VERDICT" == "pending" ]]; then
    NEEDS_ACTION="true"
    REASON="verdict_not_approved"
# Default: no action needed (shouldn't reach here normally)
else
    NEEDS_ACTION="false"
    REASON="approved_clean"
fi

# Output JSON
jq -n \
    --argjson sticky_found true \
    --arg sticky_updated_at "$STICKY_TS" \
    --arg verdict "$VERDICT" \
    --argjson unresolved_threads "$UNRESOLVED_COUNT" \
    --argjson baseline_provided "$BASELINE_PROVIDED" \
    --argjson changes_detected "$CHANGES_DETECTED" \
    --argjson needs_action "$NEEDS_ACTION" \
    --arg reason "$REASON" \
    '{
        sticky_found: $sticky_found,
        sticky_updated_at: $sticky_updated_at,
        verdict: $verdict,
        unresolved_threads: $unresolved_threads,
        baseline_provided: $baseline_provided,
        changes_detected: $changes_detected,
        needs_action: $needs_action,
        reason: $reason
    }'
