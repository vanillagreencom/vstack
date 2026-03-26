#!/bin/bash

set -euo pipefail

completion_expected_state() {
	local parent_id="${1:-}"

	if [[ -n "$parent_id" ]]; then
		printf 'Done\n'
		return 0
	fi

	printf 'In Progress\n'
}

completion_state_matches() {
	local state="${1:-}"
	local parent_id="${2:-}"
	local expected_state

	expected_state=$(completion_expected_state "$parent_id")
	[[ "$state" == "$expected_state" ]]
}

build_completion_validation_result() {
	local issue_id="$1"
	local state="$2"
	local parent_id="$3"
	local has_summary="$4"
	local state_ok="false"
	local ok="false"

	if completion_state_matches "$state" "$parent_id"; then
		state_ok="true"
	fi

	if [[ "$state_ok" == "true" && "$has_summary" == "true" ]]; then
		ok="true"
	fi

	jq -n \
		--arg id "$issue_id" \
		--arg state "$state" \
		--argjson state_ok "$state_ok" \
		--argjson has_summary "$has_summary" \
		--argjson ok "$ok" \
		'{id: $id, state: $state, state_ok: $state_ok, has_summary: $has_summary, ok: $ok}'
}
