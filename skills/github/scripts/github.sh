#!/bin/bash
# GitHub API CLI - Main Entry Point
# Usage: ./github.sh [-C <path>] <command> [options]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Parse -C flag (must come before command, like git)
WORK_DIR=""
if [ "${1:-}" = "-C" ]; then
    if [ -z "${2:-}" ]; then
        echo "Error: -C requires a path argument" >&2
        exit 1
    fi
    WORK_DIR="$2"
    shift 2
fi

show_help() {
    cat << 'EOF'
GitHub API CLI

Usage: ./github.sh [-C <path>] <command> [options]

Global Options:
  -C <path>    Run as if started in <path> (like git -C)

Commands:
  pr-data            Get PR with threads, comments, and files
  pr-view            View PR details (current branch or by number)
  pr-threads         Get PR review threads (optionally filtered)
  pr-review-status   Check review state, determine if action needed
  pr-list-ready      List PRs ready for merge
  pr-list-failing    List PRs with CI failures
  pr-create          Create PR as bot account
  pr-merge           Merge PR as bot account (with safety checks)
  pr-cross-check     Analyze multiple PRs for conflicts/dependencies
  pr-issue           Extract issue ID from PR branch name
  ci-logs            Get CI failure logs for a PR
  bot-token          Check bot token configuration
  dismiss-review     Dismiss a PR review (bot or specific user)
  resolve-thread     Resolve a review thread
  unresolve-thread   Unresolve a review thread
  post-reply         Reply to a review comment
  post-comment       Post a PR-level comment
  find-comment       Find a comment by pattern/author
  edit-comment       Edit an existing comment
  sticky-comment     Get claude bot sticky comment with verdict

Output Formats:
  --format=safe    Flat, normalized structure (DEFAULT)
  --format=raw     Original GitHub API structure

Examples:
  # Get PR data with all threads and comments
  ./github.sh pr-data 23
  ./github.sh pr-data              # Uses current branch's PR

  # Get unresolved threads only
  ./github.sh pr-threads 23 --unresolved

  # Resolve a thread
  ./github.sh resolve-thread PRRT_kwDO...

  # Post replies
  ./github.sh post-reply 12345678 "Thanks, fixed!"
  ./github.sh post-comment 23 "Addressed all feedback"

For command-specific help:
  ./github.sh <command> --help
EOF
}

# Route to command script
command="${1:-help}"
shift || true

case "$command" in
    pr-data|pr-view|pr-threads|pr-review-status|pr-list-ready|pr-list-failing|pr-create|pr-merge|pr-cross-check|pr-issue|ci-logs|bot-token|dismiss-review|resolve-thread|unresolve-thread|post-reply|post-comment|find-comment|edit-comment|sticky-comment)
        script="$SCRIPT_DIR/commands/${command}.sh"
        if [ -f "$script" ]; then
            if [ -n "$WORK_DIR" ]; then
                # Run in subshell to preserve caller's cwd
                (cd "$WORK_DIR" && exec bash "$script" "$@")
            else
                exec bash "$script" "$@"
            fi
        else
            echo "Error: Command script not found: $script" >&2
            exit 1
        fi
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Error: Unknown command '$command'" >&2
        echo "Run './github.sh --help' for usage." >&2
        exit 1
        ;;
esac
