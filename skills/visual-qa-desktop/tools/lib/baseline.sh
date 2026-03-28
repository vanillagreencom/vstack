cmd_baseline() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop baseline capture|check [options]"
    local subcmd="$1"; shift

    case "$subcmd" in
        capture) _baseline_run false "$@" ;;
        check)   _baseline_run true "$@" ;;
        *)       die "Unknown baseline subcommand: $subcmd (use capture|check)" ;;
    esac
}

_binary_has_dev_screenshot_support() {
    local binary="$1"
    [[ -f "$binary" ]] || return 1
    LC_ALL=C grep -aq "$VQA_SCREENSHOT_ENV_KEY" "$binary" 2>/dev/null
}

_detect_platform() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)       echo "linux" ;;
    esac
}

_baseline_run() {
    local do_compare="$1"; shift
    if [[ "$VQA_SUPPORTS_LAYOUT" != "true" ]]; then
        die "VQA_TARGET=${VQA_TARGET} does not support baseline capture/check"
    fi
    local fixtures_dir="${VQA_BASELINE_FIXTURES_DIR:-$ROOT_DIR/testdata/golden/layouts}"
    local platform
    platform=$(_detect_platform)
    local golden_dir="${VQA_BASELINE_GOLDEN_DIR:-$ROOT_DIR/testdata/golden/screenshots/$platform}"
    local output_dir=""
    local scenarios="all"
    local build=true

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --scenarios) scenarios="$2"; shift 2 ;;
            --output)    output_dir="$2"; shift 2 ;;
            --no-build)  build=false; shift ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    if [[ -z "$output_dir" ]]; then
        if [[ "$do_compare" == "false" ]]; then
            output_dir="$golden_dir"
        else
            output_dir="$ROOT_DIR/testdata/screenshots/baseline-check-$(date +%Y%m%d-%H%M%S)"
        fi
    fi
    mkdir -p "$output_dir"

    if [[ "$build" == "true" ]]; then
        info "Building ${VQA_TARGET_NAME}..."
        if ! build_target 2>&1; then
            die "Build failed"
        fi
    fi

    local fixture_files=()
    if [[ "$scenarios" == "all" ]]; then
        for f in "$fixtures_dir"/$VQA_BASELINE_FIXTURE_GLOB; do
            [[ -f "$f" ]] && fixture_files+=("$f")
        done
    else
        IFS=',' read -ra names <<< "$scenarios"
        for name in "${names[@]}"; do
            local f="$fixtures_dir/${name}"
            if [[ ! -f "$f" ]] && [[ "$name" != *.* ]] && [[ "$VQA_BASELINE_FIXTURE_GLOB" == \** ]]; then
                f="$fixtures_dir/${name}${VQA_BASELINE_FIXTURE_GLOB#\*}"
            fi
            [[ -f "$f" ]] || die "Fixture not found: $f"
            fixture_files+=("$f")
        done
    fi

    [[ ${#fixture_files[@]} -gt 0 ]] || die "No fixture files found in $fixtures_dir"

    local total=${#fixture_files[@]}
    local passed=0 failed=0 regressions=0
    local results=()
    local diagnostics_dir="$output_dir/baseline-logs"
    local shared_failure_consistent=true
    local shared_failure_stage=""
    local shared_failure_error=""
    local shared_failure_hint=""
    local shared_failure_log=""
    mkdir -p "$diagnostics_dir"

    if [[ "$do_compare" == "true" ]]; then
        info "Baseline check: $total scenarios (comparing against $golden_dir)"
    else
        info "Baseline capture: $total scenarios → $output_dir"
    fi

    if [[ "$build" == "false" ]]; then
        local binary="$VQA_APP_BINARY"
        if [[ -x "$binary" ]] && ! _binary_has_dev_screenshot_support "$binary"; then
            local preflight_log="$diagnostics_dir/preflight.log"
            local preflight_error="Binary $binary does not include the screenshot path required for baseline capture."
            local preflight_hint="Rebuild with: $VQA_BUILD_CMD, or rerun baseline capture without --no-build."

            cat > "$preflight_log" <<EOF
FAIL: visual-qa-desktop baseline preflight failed
stage: build/bootstrap
cause: $preflight_error
hint: $preflight_hint
binary: $binary
EOF

            warn "Baseline preflight failed: $preflight_error"
            warn "  hint: $preflight_hint"

            for f in "${fixture_files[@]}"; do
                local name
                name=$(basename "$f")
                name="${name%.*}"
                echo "  $name: FAILED (build/bootstrap)"
                results+=("$(jq -nc \
                    --arg scenario "$name" \
                    --arg stage "build/bootstrap" \
                    --arg error "$preflight_error" \
                    --arg hint "$preflight_hint" \
                    --arg log_path "$preflight_log" \
                    '{scenario:$scenario,captured:false,stage:$stage,error:$error,hint:$hint,log_path:$log_path}')")
            done

            failed=$total
            shared_failure_stage="build/bootstrap"
            shared_failure_error="$preflight_error"
            shared_failure_hint="$preflight_hint"
            shared_failure_log="$preflight_log"
        fi
    fi

    if [[ $failed -eq 0 ]]; then
        for f in "${fixture_files[@]}"; do
            local name
            name=$(basename "$f")
            name="${name%.*}"
            local screenshot capture_output scenario_log stage error hint
            scenario_log="$diagnostics_dir/${name}.capture.log"

            if capture_output=$("$SCRIPT_DIR/screenshot" --no-build --layout "$f" --output "$output_dir" 2>&1); then
                printf '%s\n' "$capture_output" > "$scenario_log"
                screenshot=$(printf '%s\n' "$capture_output" | tail -1)
            else
                printf '%s\n' "$capture_output" > "$scenario_log"
                stage=$(printf '%s\n' "$capture_output" | sed -n 's/^stage: //p' | head -1)
                error=$(printf '%s\n' "$capture_output" | sed -n 's/^cause: //p' | head -1)
                hint=$(printf '%s\n' "$capture_output" | sed -n 's/^hint: //p' | head -1)
                warn "$name: capture FAILED${stage:+ ($stage)}"
                [[ -n "$error" ]] && warn "  cause: $error"
                results+=("$(jq -nc \
                    --arg scenario "$name" \
                    --arg stage "$stage" \
                    --arg error "$error" \
                    --arg hint "$hint" \
                    --arg log_path "$scenario_log" \
                    '{scenario:$scenario,captured:false,stage:$stage,error:$error,hint:$hint,log_path:$log_path}')")
                failed=$((failed + 1))

                if [[ -z "$shared_failure_error" ]]; then
                    shared_failure_stage="$stage"
                    shared_failure_error="$error"
                    shared_failure_hint="$hint"
                    shared_failure_log="$scenario_log"
                elif [[ "$shared_failure_stage" != "$stage" ]] || [[ "$shared_failure_error" != "$error" ]]; then
                    shared_failure_consistent=false
                fi
                continue
            fi

            if [[ ! -f "$screenshot" ]]; then
                error="Screenshot helper exited successfully but did not produce a file."
                hint="Inspect the scenario log for the raw capture output."
                warn "$name: capture FAILED"
                results+=("$(jq -nc \
                    --arg scenario "$name" \
                    --arg stage "screenshot capture" \
                    --arg error "$error" \
                    --arg hint "$hint" \
                    --arg log_path "$scenario_log" \
                    '{scenario:$scenario,captured:false,stage:$stage,error:$error,hint:$hint,log_path:$log_path}')")
                failed=$((failed + 1))

                if [[ -z "$shared_failure_error" ]]; then
                    shared_failure_stage="screenshot capture"
                    shared_failure_error="$error"
                    shared_failure_hint="$hint"
                    shared_failure_log="$scenario_log"
                elif [[ "$shared_failure_stage" != "screenshot capture" ]] || [[ "$shared_failure_error" != "$error" ]]; then
                    shared_failure_consistent=false
                fi
                continue
            fi

            local golden_match="null" rmse="null"
            if [[ "$do_compare" == "true" ]]; then
                local golden_file
                golden_file=$(find "$golden_dir" -name "${name}_*" -type f 2>/dev/null | head -1)
                if [[ -n "$golden_file" ]] && command -v compare &>/dev/null; then
                    rmse=$(compare -metric RMSE "$screenshot" "$golden_file" /dev/null 2>&1 | awk 'NR==1 { print $1 }' || echo "error")
                    if [[ "$rmse" != "error" ]] && (( $(echo "$rmse < 0.05" | bc -l 2>/dev/null || echo 0) )); then
                        golden_match="true"
                    else
                        golden_match="false"
                        regressions=$((regressions + 1))
                    fi
                    echo "  $name: RMSE=$rmse $([ "$golden_match" = "true" ] && echo "✓" || echo "✗ REGRESSION")"
                elif [[ -z "$golden_file" ]]; then
                    echo "  $name: captured (no baseline to compare)"
                else
                    echo "  $name: captured (compare not installed, skipping diff)"
                fi
            else
                echo "  $name: captured"
            fi

            results+=("$(jq -nc \
                --arg scenario "$name" \
                --arg path "$screenshot" \
                --argjson golden_match "$golden_match" \
                --argjson rmse "$rmse" \
                '{scenario:$scenario,captured:true,path:$path,golden_match:$golden_match,rmse:$rmse}')")
            passed=$((passed + 1))
        done
    fi

    local mode verdict
    if [[ "$do_compare" == "true" ]]; then
        mode="baseline-check"
        [[ $regressions -gt 0 || $failed -gt 0 ]] && verdict="action_required" || verdict="pass"
    else
        mode="baseline-capture"
        [[ $failed -gt 0 ]] && verdict="action_required" || verdict="pass"
    fi

    local scenarios_json="[]"
    if [[ ${#results[@]} -gt 0 ]]; then
        scenarios_json="[$(IFS=,; echo "${results[*]}")]"
    fi

    local shared_failure_json="null"
    if [[ $failed -eq $total ]] && [[ $total -gt 0 ]] && [[ "$shared_failure_consistent" == "true" ]] && [[ -n "$shared_failure_error" ]]; then
        shared_failure_json=$(jq -nc \
            --arg stage "$shared_failure_stage" \
            --arg error "$shared_failure_error" \
            --arg hint "$shared_failure_hint" \
            --arg log_path "$shared_failure_log" \
            '{stage:$stage,error:$error,hint:$hint,log_path:$log_path}')
    fi

    local report="$output_dir/baseline-report.json"
    jq -n \
        --arg agent "visual-qa-desktop" \
        --arg mode "$mode" \
        --arg timestamp "$(date +%Y-%m-%dT%H:%M:%S%z)" \
        --arg verdict "$verdict" \
        --arg summary "$total scenarios, $passed captured, $failed failed, $regressions regressions" \
        --arg diagnostics_dir "$diagnostics_dir" \
        --argjson scenarios "$scenarios_json" \
        --argjson shared_failure "$shared_failure_json" \
        --argjson screenshots_captured "$passed" \
        --argjson scenarios_tested "$total" \
        --argjson regressions "$regressions" \
        --argjson pass "$([ "$verdict" = "pass" ] && echo "true" || echo "false")" \
        '{
            agent: $agent,
            mode: $mode,
            timestamp: $timestamp,
            verdict: $verdict,
            summary: $summary,
            scenarios: $scenarios,
            shared_failure: $shared_failure,
            qa_metadata: {
                visual_qa: {
                    screenshots_captured: $screenshots_captured,
                    scenarios_tested: $scenarios_tested,
                    regressions: $regressions,
                    pass: $pass,
                    diagnostics_dir: $diagnostics_dir
                }
            }
        }' > "$report"

    if [[ "$do_compare" == "false" ]]; then
        cat > "$output_dir/BASELINES.txt" <<EOF
captured: $(date +%Y-%m-%d\ %H:%M:%S)
commit: $(git -C "$ROOT_DIR" rev-parse --short HEAD 2>/dev/null || echo "unknown")
scenarios: $total
EOF
    fi

    echo ""
    if [[ "$do_compare" == "true" ]]; then
        echo "Baseline check: $passed/$total captured, $regressions regressions"
    else
        echo "Baseline capture: $passed/$total scenarios"
    fi
    echo "Report: $report"
    [[ "$verdict" == "pass" ]]
}
