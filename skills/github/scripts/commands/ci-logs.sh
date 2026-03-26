#!/bin/bash
# Get CI failure logs for a PR
# Usage: ci-logs <PR_NUMBER> [--lines N] [--json]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "$SCRIPT_DIR/../lib/github-api.sh"

show_help() {
    cat <<'EOF'
Get CI failure logs for a PR

Usage: ci-logs <PR_NUMBER> [options]

Options:
  --lines N       Number of log lines to show (default: 100)
  --format=safe   JSON output with metadata (default)
  --format=text   Human-readable text output
  --json          Alias for --format=safe (deprecated)

Fetches the first failed workflow run for the PR and returns:
  - Job name (rust, etc.)
  - Error type (based on job name heuristics)
  - Run ID for further investigation
  - Failed log output (last N lines)

Examples:
  github.sh ci-logs 42               # JSON output with metadata
  github.sh ci-logs 42 --lines 200   # More log lines (JSON)
  github.sh ci-logs 42 --format=text # Human-readable text
EOF
}

classify_error_type() {
    local job_name="$1"
    local logs="$2"

    # Check logs for specific patterns
    if echo "$logs" | grep -qi 'cargo fmt\|Diff in'; then
        echo "fmt"
    elif echo "$logs" | grep -qi 'clippy'; then
        echo "clippy"
    elif echo "$logs" | grep -qi 'cargo test\|test result:\|FAILED'; then
        echo "test"
    elif echo "$logs" | grep -qi 'error\[E\|cannot find\|unresolved\|build failed'; then
        echo "build"
    else
        # Fallback to job name heuristics
        case "$job_name" in
        *fmt* | *format*) echo "fmt" ;;
        *clippy* | *lint*) echo "clippy" ;;
        *test*) echo "test" ;;
        *build*) echo "build" ;;
        *) echo "unknown" ;;
        esac
    fi
}

main() {
    local pr_num="" lines=100 format="safe"

    while [ $# -gt 0 ]; do
        case "$1" in
        --lines)
            if [ -n "${2:-}" ]; then
                lines="$2"
                shift 2
            else
                echo "Error: --lines requires a number" >&2
                exit 1
            fi
            ;;
        --format=*)
            format="${1#--format=}"
            shift
            ;;
        --json)
            format="safe"
            shift
            ;; # Deprecated alias
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
        echo "Error: PR number required" >&2
        exit 1
    fi

    # Check PR exists
    if ! gh pr view "$pr_num" --json title >/dev/null 2>&1; then
        echo "Error: PR #$pr_num not found" >&2
        exit 1
    fi

    # Get failed checks
    local checks_json
    if ! checks_json=$(json_or_default '[]' array gh pr checks "$pr_num" --json name,state,workflowName,workflowRunDatabaseId); then
        if [ "$format" = "safe" ]; then
            jq -n --arg pr "$pr_num" '{error: "ci_fetch_failed", details: ("Failed to fetch CI checks for PR #" + $pr)}'
            exit 0
        fi
        echo "Error: failed to fetch CI checks for PR #$pr_num" >&2
        exit 1
    fi

    # Find first failed check
    local failed_check
    failed_check=$(echo "$checks_json" | jq -r '[.[] | select(.state == "FAILURE")] | .[0] // empty')

    if [ -z "$failed_check" ]; then
        if [ "$format" = "safe" ]; then
            jq -n '{error: "No failed checks found for this PR"}'
        else
            echo "No failed checks found for PR #$pr_num"
        fi
        exit 0
    fi

    local run_id job_name workflow_name
    run_id=$(echo "$failed_check" | jq -r '.workflowRunDatabaseId')
    job_name=$(echo "$failed_check" | jq -r '.name')
    workflow_name=$(echo "$failed_check" | jq -r '.workflowName')

    # Fetch failed logs
    local logs
    logs=$(gh run view "$run_id" --log-failed 2>&1 | tail -"$lines" || echo "Failed to fetch logs")

    # Classify error type
    local error_type
    error_type=$(classify_error_type "$job_name" "$logs")

    case "$format" in
    safe | json)
        jq -n \
            --arg run_id "$run_id" \
            --arg job "$job_name" \
            --arg workflow "$workflow_name" \
            --arg error_type "$error_type" \
            --arg logs "$logs" \
            '{run_id: $run_id, job: $job, workflow: $workflow, error_type: $error_type, logs: $logs}'
        ;;
    text)
        echo "Job: $job_name"
        echo "Workflow: $workflow_name"
        echo "Error type: $error_type"
        echo "Run ID: $run_id"
        echo ""
        echo "--- Failed logs (last $lines lines) ---"
        echo "$logs"
        ;;
    *)
        echo "Error: Unknown format: $format. Use: safe, text" >&2
        exit 1
        ;;
    esac
}

main "$@"
