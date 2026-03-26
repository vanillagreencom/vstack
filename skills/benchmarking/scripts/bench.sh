#!/usr/bin/env bash
# Benchmark query and recording tool (schema v3)
# Usage: bench.sh <command> [options]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
RESULTS_DIR="${BENCH_RESULTS_DIR:-${PROJECT_ROOT}/benchmarks/results}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

usage() {
	cat <<EOF
Benchmark Query Tool (schema v3)

Usage: bench.sh <command> [options]

Commands:
  query <component> [--days N]    Query benchmark history (default: 30 days)
  latest <component>              Get most recent result for component
  baseline <component>            Get baseline (oldest in last 90 days or tagged)
  record <component> <json>       Record a new benchmark result (schema v3 JSON)
  regression <component> [--threshold N] [--baseline FILE]  Check for regression
  compare <component> <commit1> <commit2>  Compare two commits
  list                            List all components with results
  summary                         Summary of all components' latest results

Components are auto-discovered from benchmarks/results/ subdirectories.
Override results location: BENCH_RESULTS_DIR=/path/to/results

Examples:
  bench.sh query my_component --days 7
  bench.sh latest my_component
  bench.sh regression my_component --threshold 5
  bench.sh record my_component '{"schema_version":3,...}'
EOF
}

ensure_dirs() {
	mkdir -p "$RESULTS_DIR"
}

ensure_component_dir() {
	local component="$1"
	mkdir -p "$RESULTS_DIR/$component"
}

timestamp_now() {
	date +%s
}

timestamp_days_ago() {
	local days="${1:-30}"
	if [[ "$(uname)" == "Darwin" ]]; then
		date -v-${days}d +%s
	else
		date -d "$days days ago" +%s
	fi
}

git_commit() {
	git -C "$PROJECT_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown"
}

platform_id() {
	local os arch
	os="$(uname -s | tr '[:upper:]' '[:lower:]')"
	arch="$(uname -m)"
	echo "${os}-${arch}"
}

kernel_version() {
	uname -r 2>/dev/null || echo "unknown"
}

cpu_model() {
	if [[ -f /proc/cpuinfo ]]; then
		grep -m1 "model name" /proc/cpuinfo 2>/dev/null | sed 's/.*: //' || echo "unknown"
	elif command -v sysctl &>/dev/null; then
		sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "unknown"
	else
		echo "unknown"
	fi
}

cpu_governor() {
	if [[ -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]]; then
		cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || echo "unknown"
	else
		echo "unknown"
	fi
}

# ---------------------------------------------------------------------------
# Schema v3 helpers
# ---------------------------------------------------------------------------

# Extract comparison/display rows from a v3 result.
# Output: measurement_key<TAB>metric_kind<TAB>label<TAB>unit<TAB>value
measurement_rows() {
	echo "$1" | jq -r '
		(.tool.name // "unknown") as $tool |
		(.measurements // [])[]? |
		if .metric_kind == "latency_distribution" then
			[
				["latency_distribution:p50", .metric_kind, "P50", (.unit // "ns"), (.distribution.p50 // 0)],
				["latency_distribution:p99", .metric_kind, "P99", (.unit // "ns"), (.distribution.p99 // 0)]
			]
			+ if (.distribution.p999 // null) != null then
				[["latency_distribution:p999", .metric_kind, "P99.9", (.unit // "ns"), .distribution.p999]]
			  else
				[]
			  end
			| .[]
		elif .metric_kind == "wall_time" and $tool == "beacon-capture" then
			[
				[
					"wall_time:p50",
					.metric_kind,
					"P50",
					(.unit // ""),
					(.summary.estimate.value // 0)
				]
			]
			+ if (.summary.confidence_interval.upper // null) != null then
				[[
					"wall_time:p99",
					.metric_kind,
					"P99",
					(.unit // ""),
					.summary.confidence_interval.upper
				]]
			  else
				[]
			  end
			| .[]
		else
			[
				(.metric_kind + ":" + (
					if .metric_kind == "wall_time" then
						if (.summary.estimate.stat // "") == "mean" or (.summary.estimate.stat // "") == "" then "mean"
						else (.summary.estimate.stat // "mean")
						end
					elif .metric_kind == "instruction_count" then "ir"
					elif .metric_kind == "estimated_cycles" then "cycles"
					elif .metric_kind == "throughput" then "rate"
					else (.summary.estimate.stat // .metric_kind // "unknown")
					end
				)),
				.metric_kind,
				(
					if .metric_kind == "wall_time" then
						if (.summary.estimate.stat // "") == "p50" then "P50"
						elif (.summary.estimate.stat // "") == "mean" or (.summary.estimate.stat // "") == "" then "mean"
						else (.summary.estimate.stat // "mean")
						end
					elif .metric_kind == "instruction_count" then "Ir"
					elif .metric_kind == "estimated_cycles" then "EstCycles"
					elif .metric_kind == "throughput" then "Rate"
					else (.summary.estimate.stat // .metric_kind // "unknown")
					end
				),
				(.unit // ""),
				(.summary.estimate.value // 0)
			]
		end
		| @tsv
	'
}

measurement_row_for_key() {
	local result_json="$1"
	local measurement_key="$2"
	measurement_rows "$result_json" | awk -F '\t' -v key="$measurement_key" '$1 == key { print; exit }'
}

result_variant_key() {
	echo "$1" | jq -r '
		(.measurements // []) as $m |
		([ $m[]? | .topology // .topology_label | select(.) ] | first // "") as $topology |
		(.load_condition // "") as $root_load |
		([ $m[]? | .load_condition | select(.) ] | first // (if ($root_load | length) > 0 then $root_load else "" end)) as $load_condition |
		[
			(.operation // "default"),
			$topology,
			$load_condition
		] | @tsv
	'
}

variant_display_label() {
	local variant_key="$1"
	IFS=$'\t' read -r operation topology load_condition <<<"$variant_key"
	local label="$operation"
	if [[ -n "$topology" ]]; then
		label="${label} [${topology}]"
	fi
	if [[ -n "$load_condition" ]]; then
		label="${label} {${load_condition}}"
	fi
	printf '%s\n' "$label"
}

result_for_commit_variant() {
	local all_results="$1"
	local commit_hash="$2"
	local variant_key="$3"

	echo "$all_results" | jq --arg c "$commit_hash" --arg key "$variant_key" '
		def variant_key:
			(.measurements // []) as $m
			| ([ $m[]? | .topology // .topology_label | select(.) ] | first // "") as $topology
			| (.load_condition // "") as $root_load
			| ([ $m[]? | .load_condition | select(.) ] | first // (if ($root_load | length) > 0 then $root_load else "" end)) as $load_condition
			| [
				(.operation // "default"),
				$topology,
				$load_condition
			] | @tsv;
		[.[] | select(.commit_hash == $c and (variant_key == $key))] | sort_by(.timestamp) | last
	'
}

latest_result_for_variant() {
	local all_results="$1"
	local variant_key="$2"

	echo "$all_results" | jq --arg key "$variant_key" '
		def variant_key:
			(.measurements // []) as $m
			| ([ $m[]? | .topology // .topology_label | select(.) ] | first // "") as $topology
			| (.load_condition // "") as $root_load
			| ([ $m[]? | .load_condition | select(.) ] | first // (if ($root_load | length) > 0 then $root_load else "" end)) as $load_condition
			| [
				(.operation // "default"),
				$topology,
				$load_condition
			] | @tsv;
		[.[] | select(variant_key == $key)] | sort_by(.timestamp) | last
	'
}

variant_keys_from_results() {
	local all_results="$1"

	echo "$all_results" | jq -c '.[]' | while IFS= read -r result; do
		[[ -z "$result" ]] && continue
		result_variant_key "$result"
	done
}

classify_change() {
	local metric_kind="$1"
	local change="$2"
	local threshold="$3"

	if [[ "$metric_kind" == "throughput" ]]; then
		if (($(awk "BEGIN{print ($change < -$threshold) ? 1 : 0}"))); then
			echo "regressed"
		elif (($(awk "BEGIN{print ($change > $threshold) ? 1 : 0}"))); then
			echo "improved"
		else
			echo "neutral"
		fi
	else
		if (($(awk "BEGIN{print ($change > $threshold) ? 1 : 0}"))); then
			echo "regressed"
		elif (($(awk "BEGIN{print ($change < -$threshold) ? 1 : 0}"))); then
			echo "improved"
		else
			echo "neutral"
		fi
	fi
}

percent_change_or_na() {
	local baseline="$1"
	local latest="$2"
	local precision="${3:-2}"

	awk -v baseline="$baseline" -v latest="$latest" -v precision="$precision" '
		BEGIN {
			if (baseline == 0) {
				print "N/A"
				exit 0
			}

			printf "%.*f", precision, ((latest - baseline) * 100) / baseline
		}
	'
}

print_change_column() {
	local change="$1"

	if [[ "$change" == "N/A" ]]; then
		printf "%8s" "N/A"
	else
		printf "%+7.1f%%" "$change"
	fi
}

# Extract tool name
tool_name() {
	echo "$1" | jq -r '.tool.name // "unknown"'
}

# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

cmd_list() {
	echo "Components with benchmark results:"
	for component in market_data order_execution risk ipc ui; do
		local count
		count=$(find "$RESULTS_DIR/$component" -name "*.json" 2>/dev/null | wc -l | tr -d ' ')
		if [[ "$count" -gt 0 ]]; then
			echo "  $component: $count results"
		fi
	done
}

cmd_query() {
	local component="$1"
	local days="${2:-30}"
	local since
	since=$(timestamp_days_ago "$days")
	local dir="$RESULTS_DIR/$component"

	if [[ ! -d "$dir" ]]; then
		echo "No results for component: $component" >&2
		return 1
	fi

	find "$dir" -name "*.json" -print0 2>/dev/null |
		xargs -0 cat 2>/dev/null |
		jq -s --argjson since "$since" '
            [.[] | select(.timestamp >= $since)]
            | sort_by(.timestamp)
            | reverse
        ' 2>/dev/null || echo "[]"
}

cmd_latest() {
	local component="$1"
	local dir="$RESULTS_DIR/$component"

	if [[ ! -d "$dir" ]]; then
		echo "No results for component: $component" >&2
		return 1
	fi

	find "$dir" -name "*.json" -print0 2>/dev/null |
		xargs -0 cat 2>/dev/null |
		jq -s 'sort_by(.timestamp) | last // empty' 2>/dev/null
}

cmd_baseline() {
	local component="$1"
	local dir="$RESULTS_DIR/$component"
	local since
	since=$(timestamp_days_ago 90)

	if [[ -f "$dir/baseline.json" ]]; then
		cat "$dir/baseline.json"
		return 0
	fi

	local issue_baselines
	issue_baselines=$(find "$dir" -name "baseline_*.json" 2>/dev/null)
	if [[ -n "$issue_baselines" ]]; then
		local issue_baseline
		issue_baseline=$(find "$dir" -name "baseline_*.json" -print0 2>/dev/null | xargs -0 ls -t 2>/dev/null | head -1)
		if [[ -n "$issue_baseline" ]]; then
			cat "$issue_baseline"
			return 0
		fi
	fi

	find "$dir" -name "*.json" ! -name "baseline*.json" -print0 2>/dev/null |
		xargs -0 cat 2>/dev/null |
		jq -s --argjson since "$since" '
            [.[] | select(.timestamp >= $since)]
            | sort_by(.timestamp)
            | first // empty
        ' 2>/dev/null
}

VALID_COMPONENTS="market_data order_execution risk ipc ui"

cmd_record() {
	local component="$1"
	local json="$2"

	if ! echo "$VALID_COMPONENTS" | grep -qw "$component"; then
		echo -e "${RED}Invalid component: $component${NC}" >&2
		echo "Valid components: $VALID_COMPONENTS" >&2
		return 1
	fi

	local dir="$RESULTS_DIR/$component"
	ensure_dirs

	# Legacy flat-file format support (auto-converts to schema v3 on record).
	local has_legacy has_measurements legacy_source legacy_sample_count legacy_kind
	has_legacy=$(echo "$json" | jq 'has("p50_ns") or has("p99_ns")' 2>/dev/null)
	has_measurements=$(echo "$json" | jq '(.measurements // []) | length > 0' 2>/dev/null)
	if [[ "$has_legacy" == "true" && "$has_measurements" != "true" ]]; then
		legacy_source=$(echo "$json" | jq -r '.source // empty' 2>/dev/null)
		legacy_sample_count=$(echo "$json" | jq -r '.sample_count // empty' 2>/dev/null)
		legacy_kind=""
		if [[ "$legacy_source" == "iai-callgrind" ]]; then
			legacy_kind="iai-callgrind"
		elif [[ "$legacy_source" == "criterion" ]]; then
			legacy_kind="criterion"
		elif [[ "$legacy_source" == "beacon-capture" ]]; then
			if [[ "$component" != "ui" ]]; then
				echo -e "${RED}Unsupported legacy beacon payload outside ui${NC}" >&2
				return 1
			fi
			legacy_kind="beacon-capture"
		elif [[ -n "$legacy_source" ]]; then
			echo -e "${RED}Unsupported legacy source: $legacy_source${NC}" >&2
			return 1
		elif [[ "$component" == "ui" && -n "$legacy_sample_count" ]]; then
			legacy_kind="beacon-capture"
		elif [[ "$component" == "ui" ]]; then
			echo -e "${RED}Ambiguous legacy ui payload without source/sample_count${NC}" >&2
			return 1
		elif [[ -n "$legacy_sample_count" ]]; then
			echo -e "${RED}Ambiguous legacy payload with sample_count but no source${NC}" >&2
			return 1
		else
			legacy_kind="criterion"
		fi

		echo -e "${YELLOW}Converting legacy flat payload to v3 measurements[]${NC}" >&2
		if [[ "$legacy_kind" == "iai-callgrind" ]]; then
			json=$(echo "$json" | jq '
				. + {
					tool: { name: "iai-callgrind", version: "0.16.1" },
					measurements: [
						{
							metric_kind: "instruction_count",
							unit: "instructions",
							summary: { estimate: { stat: "Ir", value: .p50_ns } }
						},
						{
							metric_kind: "estimated_cycles",
							unit: "cycles",
							summary: { estimate: { stat: "EstimatedCycles", value: .p99_ns } }
						}
					]
				}
				| del(.p50_ns, .p99_ns, .p999_ns, .sample_count, .source)
			')
		elif [[ "$legacy_kind" == "beacon-capture" ]]; then
			json=$(echo "$json" | jq '
				. + {
					tool: { name: "beacon-capture", version: "1" },
					measurements: [{
						metric_kind: "wall_time",
						unit: "ns",
						summary: {
							estimate: { stat: "p50", value: .p50_ns },
							confidence_interval: { level: 0.95, lower: .p50_ns, upper: .p99_ns }
						}
					}]
				}
				| del(.p50_ns, .p99_ns, .p999_ns, .sample_count, .source)
			')
		else
			json=$(echo "$json" | jq '
				. + {
					tool: { name: "criterion", version: "0.5" },
					measurements: [{
						metric_kind: "wall_time",
						unit: "ns",
						summary: {
							estimate: { stat: "mean", value: .p50_ns },
							confidence_interval: { level: 0.95, lower: .p50_ns, upper: .p99_ns }
						}
					}]
				}
				| del(.p50_ns, .p99_ns, .p999_ns, .sample_count, .source)
			')
		fi
	fi

	local timestamp commit platform kernel cpu governor
	timestamp=$(timestamp_now)
	commit=$(git_commit)
	platform=$(platform_id)
	kernel=$(kernel_version)
	cpu=$(cpu_model)
	governor=$(cpu_governor)

	local operation
	operation=$(echo "$json" | jq -r '.operation // "default"' 2>/dev/null)
	local op_hash
	if [[ "$(uname)" == "Darwin" ]]; then
		op_hash=$(printf '%s' "$operation" | md5 | cut -c1-8)
	else
		op_hash=$(printf '%s' "$operation" | md5sum | cut -c1-8)
	fi
	local filename="${timestamp}_${commit}_${op_hash}.json"

	# Enrich v3 JSON with defaults for missing fields
	echo "$json" | jq \
		--argjson ts "$timestamp" \
		--arg commit "$commit" \
		--arg comp "$component" \
		--arg plat "$platform" \
		--arg kern "$kernel" \
		--arg cpu "$cpu" \
		--arg gov "$governor" '
		. + {
			schema_version: (.schema_version // 3),
			timestamp: (.timestamp // $ts),
			commit_hash: (.commit_hash // $commit),
			component: (.component // $comp),
			environment: ((.environment // {}) + {
				platform: ((.environment // {}).platform // $plat),
				kernel: ((.environment // {}).kernel // $kern),
				cpu_model: ((.environment // {}).cpu_model // $cpu),
				cpu_governor: ((.environment // {}).cpu_governor // $gov)
			}),
			tool: (.tool // { name: "unknown", version: "0" }),
			measurements: (.measurements // [])
		}
	' >"$dir/$filename"

	echo -e "${GREEN}Recorded:${NC} $dir/$filename"
}

# Detect environment differences between two v3 JSON objects
check_env_diff() {
	local base_json="$1"
	local latest_json="$2"
	local diffs=()

	local b_kern l_kern b_cpu l_cpu b_gov l_gov
	b_kern=$(echo "$base_json" | jq -r '.environment.kernel // .kernel // empty')
	l_kern=$(echo "$latest_json" | jq -r '.environment.kernel // .kernel // empty')
	b_cpu=$(echo "$base_json" | jq -r '.environment.cpu_model // .cpu_model // empty')
	l_cpu=$(echo "$latest_json" | jq -r '.environment.cpu_model // .cpu_model // empty')
	b_gov=$(echo "$base_json" | jq -r '.environment.cpu_governor // .cpu_governor // empty')
	l_gov=$(echo "$latest_json" | jq -r '.environment.cpu_governor // .cpu_governor // empty')

	if [[ -n "$b_kern" && -n "$l_kern" && "$b_kern" != "$l_kern" ]]; then
		diffs+=("kernel: $b_kern -> $l_kern")
	fi
	if [[ -n "$b_cpu" && -n "$l_cpu" && "$b_cpu" != "$l_cpu" ]]; then
		diffs+=("cpu: $b_cpu -> $l_cpu")
	fi
	if [[ -n "$b_gov" && -n "$l_gov" && "$b_gov" != "$l_gov" ]]; then
		diffs+=("governor: $b_gov -> $l_gov")
	fi

	if [[ ${#diffs[@]} -gt 0 ]]; then
		echo -e "${YELLOW}WARNING: Environment changed between baseline and latest${NC}"
		for d in "${diffs[@]}"; do
			echo -e "  ${YELLOW}$d${NC}"
		done
		echo -e "${YELLOW}Regressions may reflect environment changes, not code changes${NC}"
		echo ""
		return 1
	fi
	return 0
}

cmd_regression() {
	local component="$1"
	local threshold="${2:-5}"
	local baseline_file="${3:-}"
	local dir="$RESULTS_DIR/$component"
	local regression_found=0

	if [[ ! -d "$dir" ]]; then
		echo -e "${YELLOW}No results for component: $component${NC}"
		return 0
	fi

	local all_results
	all_results=$(find "$dir" -name "*.json" ! -name "baseline*.json" -print0 2>/dev/null |
		xargs -0 cat 2>/dev/null | jq -s '.' 2>/dev/null)

	if [[ -z "$all_results" || "$all_results" == "[]" || "$all_results" == "null" ]]; then
		echo -e "${YELLOW}No results found for $component${NC}"
		return 0
	fi

	# --baseline FILE mode
	if [[ -n "$baseline_file" ]]; then
		if [[ ! -f "$baseline_file" ]]; then
			echo -e "${RED}Baseline file not found: $baseline_file${NC}" >&2
			return 1
		fi

		local baseline_data
		baseline_data=$(cat "$baseline_file")
		local baseline_variant baseline_label baseline_commit_hash
		baseline_variant=$(result_variant_key "$baseline_data")
		baseline_label=$(variant_display_label "$baseline_variant")
		baseline_commit_hash=$(echo "$baseline_data" | jq -r '.commit_hash // "file"')

		local latest_result_json latest_commit
		latest_result_json=$(latest_result_for_variant "$all_results" "$baseline_variant")
		if [[ -n "$latest_result_json" && "$latest_result_json" != "null" ]]; then
			latest_commit=$(echo "$latest_result_json" | jq -r '.commit_hash // "unknown"')
		else
			latest_commit="none"
		fi

		echo "Component: $component"
		echo "Baseline: $baseline_commit_hash [file: $(basename "$baseline_file")] | Latest: $latest_commit"
		echo ""

		printf "  %-40s %12s %12s %8s  %s\n" "Operation" "Baseline" "Latest" "Change" "Status"
		printf "  %-40s %12s %12s %8s  %s\n" "---" "---" "---" "---" "---"

		local baseline_rows
		if [[ -z "$latest_result_json" || "$latest_result_json" == "null" ]]; then
			printf "  %-40s %12s %12s %8s  %s\n" \
				"$baseline_label" "present" "missing" "N/A" "${RED}variant missing${NC}"
			echo ""
			echo -e "${RED}REGRESSION DETECTED${NC}: Matching variant missing in recorded results"
			return 1
		fi
		check_env_diff "$baseline_data" "$latest_result_json" || true
		baseline_rows=$(measurement_rows "$baseline_data")

		if [[ -z "$baseline_rows" ]]; then
			echo -e "${YELLOW}Baseline file contains no measurable v3 entries${NC}"
			return 0
		fi

		while IFS=$'\t' read -r b_key b_kind b_label b_unit b_val; do
			[[ -z "$b_key" ]] && continue

			local latest_row l_key l_kind l_label l_unit l_val
			latest_row=$(measurement_row_for_key "$latest_result_json" "$b_key")
			if [[ -z "$latest_row" ]]; then
				printf "  %-40s %10.2f%-2s %12s %8s  %s\n" \
					"$baseline_label ($b_label)" "$b_val" "$b_unit" "missing" "N/A" "${RED}metric missing${NC}"
				regression_found=1
				continue
			fi

			IFS=$'\t' read -r l_key l_kind l_label l_unit l_val <<<"$latest_row"
			if (($(awk "BEGIN{print ($b_val == 0) ? 1 : 0}"))); then
				continue
			fi

			local change status classification display_unit
			change=$(awk "BEGIN{printf \"%.2f\", (($l_val - $b_val) * 100) / $b_val}")
			status="✓"
			classification=$(classify_change "$b_kind" "$change" "$threshold")
			if [[ "$classification" == "regressed" ]]; then
				status="${RED}REGRESSED${NC}"
				regression_found=1
			elif [[ "$classification" == "improved" ]]; then
				status="${GREEN}improved${NC}"
			fi

			display_unit="$b_unit"
			if [[ -z "$display_unit" ]]; then
				display_unit="$l_unit"
			fi
			printf "  %-40s %10.2f%-2s %10.2f%-2s %+7.1f%%  " \
				"$baseline_label ($b_label)" "$b_val" "$display_unit" "$l_val" "$display_unit" "$change"
			echo -e "$status"
		done <<<"$baseline_rows"

		echo ""
		if [[ "$regression_found" -eq 1 ]]; then
			echo -e "${RED}REGRESSION DETECTED${NC}: Operation exceeds ${threshold}% threshold"
			return 1
		else
			echo -e "${GREEN}PASS${NC}: Within ${threshold}% tolerance"
			return 0
		fi
	fi

	# Default mode: compare latest commit vs previous commit
	local commits
	commits=$(echo "$all_results" | jq -r '[.[] | {commit_hash, timestamp}] | unique_by(.commit_hash) | sort_by(.timestamp) | reverse | .[].commit_hash')
	local num_commits
	num_commits=$(echo "$commits" | wc -l | tr -d ' ')

	if [[ "$num_commits" -lt 2 ]]; then
		echo -e "${YELLOW}Only one commit recorded for $component — no comparison available${NC}"
		return 0
	fi

	local latest_commit="" baseline_commit=""
	local has_git=false

	if git -C "$PROJECT_ROOT" rev-parse HEAD &>/dev/null; then
		has_git=true
		while IFS= read -r candidate; do
			[[ -z "$candidate" ]] && continue
			if git -C "$PROJECT_ROOT" merge-base --is-ancestor "$candidate" HEAD 2>/dev/null; then
				if [[ -z "$latest_commit" ]]; then
					latest_commit="$candidate"
				else
					baseline_commit="$candidate"
					break
				fi
			fi
		done <<<"$commits"
	fi

	if [[ -z "$latest_commit" ]]; then
		latest_commit=$(echo "$commits" | head -1)
	fi
	if [[ -z "$baseline_commit" ]]; then
		if [[ "$has_git" == true ]]; then
			echo -e "${YELLOW}No baseline commit reachable from HEAD for $component${NC}"
			return 0
		fi
		baseline_commit=$(echo "$commits" | grep -v "^${latest_commit}$" | head -1)
	fi

	if [[ -z "$baseline_commit" || "$baseline_commit" == "$latest_commit" ]]; then
		echo -e "${YELLOW}Only one distinct commit available for $component — no comparison possible${NC}"
		return 0
	fi

	local baseline_result_json latest_result_json
	baseline_result_json=$(echo "$all_results" | jq --arg c "$baseline_commit" '[.[] | select(.commit_hash == $c)] | sort_by(.timestamp) | last')
	latest_result_json=$(echo "$all_results" | jq --arg c "$latest_commit" '[.[] | select(.commit_hash == $c)] | sort_by(.timestamp) | last')
	local baseline_platform latest_platform
	baseline_platform=$(echo "$baseline_result_json" | jq -r '.environment.platform // .platform // "unknown"')
	latest_platform=$(echo "$latest_result_json" | jq -r '.environment.platform // .platform // "unknown"')
	if [[ "$baseline_platform" != "$latest_platform" && "$baseline_platform" != "unknown" && "$latest_platform" != "unknown" ]]; then
		echo -e "${YELLOW}WARNING: Cross-platform comparison ($baseline_platform vs $latest_platform)${NC}"
	fi

	echo "Component: $component"
	echo "Baseline: $baseline_commit ($baseline_platform) | Latest: $latest_commit ($latest_platform)"
	echo ""

	check_env_diff "$baseline_result_json" "$latest_result_json" || true

	# Compare every baseline variant; missing variants in latest are regressions.
	local variants
	variants=$(echo "$all_results" | jq -r --arg bc "$baseline_commit" --arg lc "$latest_commit" '
		def variant_key:
			(.measurements // []) as $m
			| ([ $m[]? | .topology // .topology_label | select(.) ] | first // "") as $topology
			| (.load_condition // "") as $root_load
			| ([ $m[]? | .load_condition | select(.) ] | first // (if ($root_load | length) > 0 then $root_load else "" end)) as $load_condition
			| [
				(.operation // "default"),
				$topology,
				$load_condition
			] | @tsv;
		([.[] | select(.commit_hash == $bc) | variant_key] | unique)[]
	' 2>/dev/null)

	if [[ -z "$variants" ]]; then
		echo -e "${YELLOW}No matching operations between commits${NC}"
		return 0
	fi

	printf "  %-40s %14s %14s %8s  %s\n" "Operation" "Baseline" "Latest" "Change" "Status"
	printf "  %-40s %14s %14s %8s  %s\n" "---" "---" "---" "---" "---"

	while IFS= read -r variant_key; do
		[[ -z "$variant_key" ]] && continue
		local variant_label
		variant_label=$(variant_display_label "$variant_key")

		local b_result l_result
		b_result=$(result_for_commit_variant "$all_results" "$baseline_commit" "$variant_key")
		l_result=$(result_for_commit_variant "$all_results" "$latest_commit" "$variant_key")
		if [[ -z "$l_result" || "$l_result" == "null" ]]; then
			printf "  %-40s %14s %14s %8s  %s\n" \
				"$variant_label" "present" "missing" "N/A" "${RED}variant missing${NC}"
			regression_found=1
			continue
		fi

		local baseline_rows
		baseline_rows=$(measurement_rows "$b_result")
		while IFS=$'\t' read -r b_key b_kind b_label b_unit b_val; do
			[[ -z "$b_key" ]] && continue

			local latest_row l_key l_kind l_label l_unit l_val
			latest_row=$(measurement_row_for_key "$l_result" "$b_key")
			if [[ -z "$latest_row" ]]; then
				printf "  %-40s %14s %14s %8s  %s\n" \
					"$variant_label ($b_label)" "$b_kind" "missing" "N/A" "${RED}metric missing${NC}"
				regression_found=1
				continue
			fi

			IFS=$'\t' read -r l_key l_kind l_label l_unit l_val <<<"$latest_row"
			if (($(awk "BEGIN{print ($b_val == 0) ? 1 : 0}"))); then
				continue
			fi

			local change status classification display_unit
			change=$(awk "BEGIN{printf \"%.2f\", (($l_val - $b_val) * 100) / $b_val}")
			status="✓"
			classification=$(classify_change "$b_kind" "$change" "$threshold")
			if [[ "$classification" == "regressed" ]]; then
				status="${RED}REGRESSED${NC}"
				regression_found=1
			elif [[ "$classification" == "improved" ]]; then
				status="${GREEN}improved${NC}"
			fi

			display_unit="$b_unit"
			if [[ -z "$display_unit" ]]; then
				display_unit="$l_unit"
			fi
			printf "  %-40s %12.2f%-2s %12.2f%-2s %+7.1f%%  " \
				"$variant_label ($b_label)" "$b_val" "$display_unit" "$l_val" "$display_unit" "$change"
			echo -e "$status"
		done <<<"$baseline_rows"

	done <<<"$variants"

	echo ""
	if [[ "$regression_found" -eq 1 ]]; then
		echo -e "${RED}REGRESSION DETECTED${NC}: One or more operations exceed ${threshold}% threshold"
		return 1
	else
		echo -e "${GREEN}PASS${NC}: All operations within ${threshold}% tolerance"
		return 0
	fi
}

cmd_compare() {
	local component="$1"
	local commit1="$2"
	local commit2="$3"
	local dir="$RESULTS_DIR/$component"

	local results1="" results2=""
	results1=$(find "$dir" -name "*_${commit1}_*.json" -print0 2>/dev/null | xargs -0 cat 2>/dev/null | jq -s '.')
	results2=$(find "$dir" -name "*_${commit2}_*.json" -print0 2>/dev/null | xargs -0 cat 2>/dev/null | jq -s '.')

	if [[ -z "$results1" || "$results1" == "[]" ]]; then
		echo "No results found for commit: $commit1" >&2
		return 1
	fi
	if [[ -z "$results2" || "$results2" == "[]" ]]; then
		echo "No results found for commit: $commit2" >&2
		return 1
	fi

	echo "Comparing $component benchmarks: $commit1 vs $commit2"
	echo ""
	printf "  %-40s %14s %14s %8s\n" "Operation" "$commit1" "$commit2" "Change"
	printf "  %-40s %14s %14s %8s\n" "---" "---" "---" "---"

	{
		variant_keys_from_results "$results1"
		variant_keys_from_results "$results2"
	} | sort -u | while IFS= read -r variant_key; do
		local a_result b_result
		local variant_label
		variant_label=$(variant_display_label "$variant_key")
		a_result=$(latest_result_for_variant "$results1" "$variant_key")
		b_result=$(latest_result_for_variant "$results2" "$variant_key")

		local baseline_rows
		if [[ -z "$a_result" || "$a_result" == "null" ]]; then
			baseline_rows=$(measurement_rows "$b_result")
		else
			baseline_rows=$(measurement_rows "$a_result")
		fi
		while IFS=$'\t' read -r a_key a_kind a_label a_unit a_val; do
			[[ -z "$a_key" ]] && continue

			if [[ -z "$a_result" || "$a_result" == "null" ]]; then
				local b_row b_key b_kind b_label b_unit b_val
				b_row=$(measurement_row_for_key "$b_result" "$a_key")
				IFS=$'\t' read -r b_key b_kind b_label b_unit b_val <<<"$b_row"
				printf "  %-40s %14s %12.2f%-2s %8s\n" \
					"$variant_label ($a_label)" "missing" "$b_val" "$b_unit" "N/A"
				continue
			fi

			if [[ -z "$b_result" || "$b_result" == "null" ]]; then
				printf "  %-40s %12.2f%-2s %14s %8s\n" "$variant_label ($a_label)" "$a_val" "$a_unit" "missing" "N/A"
				continue
			fi

			local latest_row b_key b_kind b_label b_unit b_val display_unit change
			latest_row=$(measurement_row_for_key "$b_result" "$a_key")
			if [[ -z "$latest_row" ]]; then
				printf "  %-40s %12.2f%-2s %14s %8s\n" "$variant_label ($a_label)" "$a_val" "$a_unit" "missing" "N/A"
				continue
			fi

			IFS=$'\t' read -r b_key b_kind b_label b_unit b_val <<<"$latest_row"
				display_unit="$a_unit"
				if [[ -z "$display_unit" ]]; then
					display_unit="$b_unit"
				fi
				change=$(percent_change_or_na "$a_val" "$b_val" 1)
				printf "  %-40s %12.2f%-2s %12.2f%-2s " \
					"$variant_label ($a_label)" "$a_val" "$display_unit" "$b_val" "$display_unit"
				print_change_column "$change"
				printf "\n"
			done <<<"$baseline_rows"
		done
}

cmd_summary() {
	echo "Benchmark Summary (Latest Results, schema v3)"
	echo "==============================================="
	echo ""

	for component in market_data order_execution risk ipc ui; do
		local latest
		latest=$(cmd_latest "$component" 2>/dev/null || true)
		if [[ -n "$latest" && "$latest" != "null" ]]; then
			local op commit tool_n
			op=$(echo "$latest" | jq -r '.operation // "default"')
			commit=$(echo "$latest" | jq -r '.commit_hash // "unknown"')
			tool_n=$(tool_name "$latest")
			local rows
			rows=$(measurement_rows "$latest")
			while IFS=$'\t' read -r key kind label unit val; do
				[[ -z "$key" ]] && continue
				printf "%-16s %-35s %s: %10.2f%-2s  (%s)  [%s]\n" \
					"$component" "($op)" "$label" "$val" "$unit" "$tool_n" "$commit"
			done <<<"$rows"
		fi
	done

	return 0
}


# Main entry point
main() {
	if [[ $# -lt 1 ]]; then
		usage
		exit 1
	fi

	local cmd="$1"
	shift

	case "$cmd" in
	query)
		[[ $# -lt 1 ]] && {
			echo "Usage: bench.sh query <component> [--days N]" >&2
			exit 1
		}
		local component="$1"
		local days=30
		shift
		while [[ $# -gt 0 ]]; do
			case "$1" in
			--days)
				days="$2"
				shift 2
				;;
			*) shift ;;
			esac
		done
		cmd_query "$component" "$days"
		;;
	latest)
		[[ $# -lt 1 ]] && {
			echo "Usage: bench.sh latest <component>" >&2
			exit 1
		}
		cmd_latest "$1"
		;;
	baseline)
		[[ $# -lt 1 ]] && {
			echo "Usage: bench.sh baseline <component>" >&2
			exit 1
		}
		cmd_baseline "$1"
		;;
	record)
		[[ $# -lt 2 ]] && {
			echo "Usage: bench.sh record <component> <json>" >&2
			exit 1
		}
		cmd_record "$1" "$2"
		;;
	regression)
		[[ $# -lt 1 ]] && {
			echo "Usage: bench.sh regression <component> [--threshold N] [--baseline FILE]" >&2
			exit 1
		}
		local component="$1"
		local threshold=5
		local baseline_file=""
		shift
		while [[ $# -gt 0 ]]; do
			case "$1" in
			--threshold)
				threshold="$2"
				shift 2
				;;
			--baseline)
				baseline_file="$2"
				shift 2
				;;
			*) shift ;;
			esac
		done
		cmd_regression "$component" "$threshold" "$baseline_file"
		;;
	compare)
		[[ $# -lt 3 ]] && {
			echo "Usage: bench.sh compare <component> <commit1> <commit2>" >&2
			exit 1
		}
		cmd_compare "$1" "$2" "$3"
		;;
	list)
		cmd_list
		;;
	summary)
		cmd_summary
		;;
	help | --help | -h)
		usage
		;;
	*)
		echo "Unknown command: $cmd" >&2
		usage
		exit 1
		;;
	esac
}

main "$@"
