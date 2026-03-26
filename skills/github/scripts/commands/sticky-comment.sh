#!/bin/bash
# Get claude bot sticky comment from PR
# Usage: ./sticky-comment.sh <PR#> [--body|--updated-at|--verdict|--analysis]
#
# Returns JSON by default, or specific field with flags:
#   --body         Just the comment body
#   --updated-at   Just the updated_at timestamp
#   --verdict      "approved" | "changes" | "pending" (emoji-based)
#   --analysis     Deep analysis: recommendation, remaining items, merge readiness
#
# Exit codes:
#   0 - Success
#   1 - Usage error or no sticky comment found

set -euo pipefail

_SC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$_SC_DIR/../lib/github-api.sh"

show_help() {
    cat << 'EOF'
Get Claude Bot Sticky Comment

Usage: sticky-comment.sh <PR#> [options]

Arguments:
  PR#              PR number (required)

Options:
  --body           Just the comment body
  --updated-at     Just the updated_at timestamp
  --verdict        "approved" | "changes" | "pending" (emoji-based)
  --analysis       Deep analysis: recommendation, remaining items, merge readiness
  --help, -h       Show this help

Output (default):
  JSON object with body, updated_at, id, and computed verdict

Analysis Output:
  {
    "recommendation": "approve" | "changes" | "block" | "pending",
    "remaining_item": "issue title if any",
    "can_merge": true | false
  }

Examples:
  sticky-comment.sh 23                  # Full JSON
  sticky-comment.sh 23 --body           # Just comment body
  sticky-comment.sh 23 --analysis       # Merge readiness analysis
  sticky-comment.sh 23 --verdict        # Quick status: approved/changes/pending
EOF
}

PR_NUM="${1:-}"
FLAG="${2:-}"

# Handle --help as first argument
if [[ "$PR_NUM" == "--help" || "$PR_NUM" == "-h" ]]; then
    show_help
    exit 0
fi

BOT_USER="${GH_BOT_USERNAME:-claude[bot]}"

if [[ -z "$PR_NUM" ]]; then
  echo '{"error": "Usage: sticky-comment.sh <PR#> [--body|--updated-at|--verdict|--analysis]"}' >&2
  exit 1
fi

# Fetch comments with error handling
RESPONSE=$(gh api "repos/{owner}/{repo}/issues/$PR_NUM/comments" 2>&1) || {
  ERROR_MSG=$(echo "$RESPONSE" | tr '\n' ' ' | head -c 100)
  jq -n --arg msg "API failed: $ERROR_MSG" '{error: $msg}' >&2
  exit 1
}

# Check for API error response (has "message" field but no array)
if echo "$RESPONSE" | jq -e '.message' >/dev/null 2>&1; then
  echo "{\"error\": \"$(echo "$RESPONSE" | jq -r '.message')\"}" >&2
  exit 1
fi

# Find sticky comment: prefer "View job" (sticky marker), fall back to review headers
STICKY=$(echo "$RESPONSE" | jq '
  [.[] | select(.user.login == "'"$BOT_USER"'")] |
  # First try: sticky with View job
  (map(select(.body | test("View job"; "i"))) | first) //
  # Fallback: any with review headers
  (map(select(.body | test("## Review|### Inline"; "i"))) | last) //
  empty
')

# Validate we got a comment object (has id and body)
# Retry once after brief delay if not found (handles API sync delay)
if [[ -z "$STICKY" || "$STICKY" == "null" ]] || ! echo "$STICKY" | jq -e '.id and .body' >/dev/null 2>&1; then
  sleep 2
  RESPONSE=$(gh api "repos/{owner}/{repo}/issues/$PR_NUM/comments" 2>&1) || true
  STICKY=$(echo "$RESPONSE" | jq '
    [.[] | select(.user.login == "'"$BOT_USER"'")] |
    (map(select(.body | test("View job"; "i"))) | first) //
    (map(select(.body | test("## Review|### Inline"; "i"))) | last) //
    empty
  ' 2>/dev/null || echo "")

  if [[ -z "$STICKY" || "$STICKY" == "null" ]] || ! echo "$STICKY" | jq -e '.id and .body' >/dev/null 2>&1; then
    echo '{"error": "No sticky comment found"}' >&2
    exit 1
  fi
fi

# Helper: detect verdict from body text (emoji-based, legacy)
#
# Returns "pending" when the comment is purely a task checklist with unchecked
# items and no final review section. The bot posts an in-progress checklist
# before it finishes reviewing; treating that as a terminal verdict causes
# bot-review-wait to exit before the bot has actually posted its findings.
#
# A comment is considered "review complete" (not just in-progress) when it
# contains at least one of:
#   - A review section header: ## Review, ### Inline, ### Recommendation
#   - An explicit approval/changes statement
# If none of those are present and unchecked items remain, return "pending".
get_verdict() {
  local body="$1"

  # Check if comment has a final review section (not just a task checklist)
  local has_review_section
  has_review_section=$(echo "$body" | grep -ciE '## Review|### Inline|### Recommendation|Recommendation:|View job' || true)

  # If no review section present, check for unchecked checklist items.
  # An in-progress checklist (unchecked items, no review section) = still processing.
  if [[ $has_review_section -eq 0 ]]; then
    local unchecked
    unchecked=$(echo "$body" | grep -c '^\s*- \[ \]' || true)
    if [[ $unchecked -gt 0 ]]; then
      echo "pending"
      return
    fi
  fi

  # Standard emoji/keyword verdict detection
  local has_check has_approv has_warn has_changes
  has_check=$(echo "$body" | grep -c "✅" || true)
  has_approv=$(echo "$body" | grep -ci "approv" || true)
  has_warn=$(echo "$body" | grep -c "⚠️" || true)
  has_changes=$(echo "$body" | grep -ci "changes" || true)

  if [[ $has_check -gt 0 && $has_approv -gt 0 ]]; then
    if [[ $has_warn -gt 0 || $has_changes -gt 0 ]]; then
      echo "changes"  # Mixed signals = treat as changes
    else
      echo "approved"
    fi
  elif [[ $has_warn -gt 0 || $has_changes -gt 0 ]]; then
    echo "changes"
  else
    echo "pending"
  fi
}

# Helper: deep analysis of bot recommendation
# Parses multiple patterns from bot comments
get_analysis() {
  local body="$1"
  local rec_type="pending"

  # Pattern 1: Explicit "### Recommendation" section
  local rec_section
  rec_section=$(echo "$body" | grep -A5 -i "### Recommendation\|Recommendation:" | head -6 || true)

  # Pattern 2: "Status:" or "Verdict:" lines
  local status_line
  status_line=$(echo "$body" | grep -i "Status:\|Verdict:" | head -2 || true)

  # Pattern 3: Direct approval statements in body
  local approval_statement
  approval_statement=$(echo "$body" | grep -iE "✅.*approved|\*\*Approved\*\*|is \*\*approved\*\*|approved for merge|ready for merge|Review Complete ✅" | head -2 || true)

  # Combine all sources for analysis
  local all_signals="$rec_section $status_line $approval_statement"

  # Determine recommendation type (order matters: most specific first)
  if echo "$all_signals" | grep -qi "approve with follow-up\|approve.*follow-up"; then
    rec_type="approve_with_followup"
  elif echo "$all_signals" | grep -qiE "✅.*[Aa]pproved|\*\*[Aa]pproved\*\*|approved for merge|ready for merge|Verdict.*Approved"; then
    rec_type="approve"
  elif echo "$all_signals" | grep -qiE "will block|blocks merge|cannot merge|do not merge|reject"; then
    # Note: avoid matching "no blocking issues" which is positive
    rec_type="block"
  elif echo "$all_signals" | grep -qi "changes requested\|address.*before\|needs changes"; then
    rec_type="changes"
  elif echo "$all_signals" | grep -qi "Review Complete ✅"; then
    # "Review Complete ✅" without explicit rejection = approve
    rec_type="approve"
  fi

  # Extract remaining items (under "### Remaining Issue" or similar)
  local remaining_section remaining_title
  remaining_section=$(echo "$body" | sed -n '/### Remaining Issue/,/###/p' | head -20 || true)
  if [[ -n "$remaining_section" ]]; then
    # Get the bolded title (first **text** pattern) - macOS compatible
    remaining_title=$(echo "$remaining_section" | sed -n 's/.*\*\*\([^*]*\)\*\*.*/\1/p' | head -1 || true)
  else
    remaining_title=""
  fi

  # Determine if merge-ready (approve or approve_with_followup)
  local can_merge="false"
  if [[ "$rec_type" == "approve" || "$rec_type" == "approve_with_followup" ]]; then
    can_merge="true"
  fi

  # Output JSON
  jq -n \
    --arg rec "$rec_type" \
    --arg remaining "$remaining_title" \
    --argjson can_merge "$can_merge" \
    '{recommendation: $rec, remaining_item: $remaining, can_merge: $can_merge}'
}

case "$FLAG" in
  --body)
    echo "$STICKY" | jq -r '.body'
    ;;
  --updated-at)
    echo "$STICKY" | jq -r '.updated_at'
    ;;
  --verdict)
    # Primary: formal GitHub review state (structured, reliable)
    FORMAL=$(get_formal_review_verdict "$PR_NUM")
    if [[ -n "$FORMAL" ]]; then
      echo "$FORMAL"
    else
      # Fallback: sticky comment text parsing
      BODY=$(echo "$STICKY" | jq -r '.body')
      get_verdict "$BODY"
    fi
    ;;
  --analysis)
    BODY=$(echo "$STICKY" | jq -r '.body')
    get_analysis "$BODY"
    ;;
  *)
    # Return full JSON with computed verdict
    # Primary: formal review; fallback: sticky text parsing
    BODY=$(echo "$STICKY" | jq -r '.body')
    FORMAL=$(get_formal_review_verdict "$PR_NUM")
    VERDICT="${FORMAL:-$(get_verdict "$BODY")}"
    # Sanitize control characters in body to prevent jq parse errors
    echo "$STICKY" | jq --arg v "$VERDICT" '.body |= gsub("[[:cntrl:]]"; "") | . + {verdict: $v}'
    ;;
esac
