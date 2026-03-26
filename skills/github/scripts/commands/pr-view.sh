#!/bin/bash
# View PR details for current branch or specified PR
# Usage: pr-view [PR_NUMBER] [--json FIELDS]

set -euo pipefail

show_help() {
    cat << 'EOF'
View PR details

Usage: pr-view [PR_NUMBER] [options]

Arguments:
  PR_NUMBER    PR number (optional, defaults to current branch's PR)

Options:
  --json FIELDS    Output specific fields as JSON (e.g., --json number,title)
  --help           Show this help

Examples:
  github.sh pr-view              # View PR for current branch
  github.sh pr-view 68           # View PR #68
  github.sh pr-view --json number   # Check if PR exists (returns JSON or fails)
  github.sh -C /path/to/worktree pr-view --json number
EOF
}

main() {
    local pr_num=""
    local json_fields=""
    local -a extra_args=()

    while [ $# -gt 0 ]; do
        case "$1" in
            --json)
                json_fields="$2"
                shift 2
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            -*)
                extra_args+=("$1")
                shift
                ;;
            *)
                if [ -z "$pr_num" ]; then
                    pr_num="$1"
                else
                    extra_args+=("$1")
                fi
                shift
                ;;
        esac
    done

    local -a cmd=(gh pr view)
    [ -n "$pr_num" ] && cmd+=("$pr_num")
    [ -n "$json_fields" ] && cmd+=(--json "$json_fields")
    [ ${#extra_args[@]} -gt 0 ] && cmd+=("${extra_args[@]}")

    "${cmd[@]}"
}

main "$@"
