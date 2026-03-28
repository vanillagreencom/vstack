_perform_tab_drag() {
    local src_name="$1" drop_x="$2" drop_y="$3"
    local src sx sy
    src=$(cmd_locate "$src_name" 2>/dev/null) || die "Source tab not found: \"$src_name\""
    sx=$(echo "$src" | cut -d' ' -f1); sy=$(echo "$src" | cut -d' ' -f2)

    info "tab-drag: $src_name ($sx,$sy) → ($drop_x,$drop_y)"
    _invalidate_ocr_cache
    local platform display wid
    platform=$(session_get platform)
    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$sx" "$sy"
            DISPLAY="$display" xdotool mousedown 1
            sleep 0.1
            DISPLAY="$display" xdotool mousemove --window "$wid" $((sx + (sx > drop_x ? -15 : 15))) "$sy"
            sleep 0.1
            DISPLAY="$display" xdotool mousemove --window "$wid" "$drop_x" "$drop_y"
            sleep 0.3
            DISPLAY="$display" xdotool mouseup 1
            ;;
        darwin)
            cliclick "dd:$sx,$sy"
            sleep 0.1
            cliclick "dm:$drop_x,$drop_y"
            sleep 0.3
            cliclick "du:$drop_x,$drop_y"
            ;;
    esac
    _invalidate_ocr_cache
    sleep 0.3
}

cmd_click() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop click X Y"
    require_session
    _invalidate_ocr_cache
    local x="$1" y="$2"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x" "$y"
            DISPLAY="$display" xdotool click 1
            ;;
        darwin)
            cliclick "c:$x,$y"
            ;;
    esac
    sleep 0.05
}

cmd_double_click() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop double-click X Y"
    require_session
    _invalidate_ocr_cache
    local x="$1" y="$2"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x" "$y"
            DISPLAY="$display" xdotool click --repeat 2 --delay 100 1
            ;;
        darwin)
            cliclick "dc:$x,$y"
            ;;
    esac
    sleep 0.05
}

cmd_right_click() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop right-click X Y"
    require_session
    _invalidate_ocr_cache
    local x="$1" y="$2"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x" "$y"
            DISPLAY="$display" xdotool click 3
            ;;
        darwin)
            cliclick "rc:$x,$y"
            ;;
    esac
    sleep 0.05
}

cmd_drag() {
    [[ $# -ge 4 ]] || die "Usage: visual-qa-desktop drag X1 Y1 X2 Y2 [--steps N]"
    require_session
    _invalidate_ocr_cache
    local x1="$1" y1="$2" x2="$3" y2="$4"
    shift 4
    local steps=20
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --steps) steps="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x1" "$y1"
            DISPLAY="$display" xdotool mousedown 1

            local i=0
            while [[ $i -le $steps ]]; do
                local cx=$(( x1 + (x2 - x1) * i / steps ))
                local cy=$(( y1 + (y2 - y1) * i / steps ))
                DISPLAY="$display" xdotool mousemove --window "$wid" "$cx" "$cy"
                sleep 0.01
                i=$((i + 1))
            done

            DISPLAY="$display" xdotool mouseup 1
            ;;
        darwin)
            cliclick "dd:$x1,$y1" "dm:$x2,$y2" "du:$x2,$y2"
            ;;
    esac
    sleep 0.1
}

cmd_mousedown() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop mousedown X Y [--button N]"
    require_session
    _invalidate_ocr_cache
    local x="$1" y="$2"
    shift 2
    local button=1
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --button) button="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x" "$y"
            DISPLAY="$display" xdotool mousedown "$button"
            ;;
        darwin)
            cliclick "dd:$x,$y"
            ;;
    esac
    sleep 0.05
}

cmd_mouseup() {
    require_session
    _invalidate_ocr_cache
    local platform display
    platform=$(session_get platform)
    local button=1
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --button) button="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    case "$platform" in
        linux)
            display=$(session_display)
            DISPLAY="$display" xdotool mouseup "$button"
            ;;
        darwin)
            cliclick "du:0,0"
            ;;
    esac
    sleep 0.05
}

cmd_move() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop move X Y"
    require_session
    local x="$1" y="$2"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool mousemove --window "$wid" "$x" "$y"
            ;;
        darwin)
            cliclick "m:$x,$y"
            ;;
    esac
    sleep 0.05
}

cmd_key() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop key KEY"
    require_session
    _invalidate_ocr_cache
    local key="$1"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool key --window "$wid" "$key"
            ;;
        darwin)
            osascript -e "tell application \"System Events\" to keystroke \"$key\""
            ;;
    esac
    sleep 0.05
}

cmd_type() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop type \"text\""
    require_session
    _invalidate_ocr_cache
    local text="$1"
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            DISPLAY="$display" xdotool type --window "$wid" "$text"
            ;;
        darwin)
            osascript -e "tell application \"System Events\" to keystroke \"$text\""
            ;;
    esac
    sleep 0.05
}

cmd_resize_pane() {
    [[ $# -ge 3 ]] || die "Usage: visual-qa-desktop resize-pane \"pane A\" \"pane B\" DELTA_PX"
    require_session
    local pane_a_title="$1" pane_b_title="$2" delta="$3"

    local before_a before_b boundary
    before_a=$(_unique_pane_json "$pane_a_title")
    before_b=$(_unique_pane_json "$pane_b_title")
    boundary=$(_shared_boundary_json "$before_a" "$before_b")
    [[ -n "$boundary" ]] || die "Panes do not share a resize handle: \"$pane_a_title\" / \"$pane_b_title\""

    local axis start_x start_y end_x end_y segment_start segment_end map_json split_json
    axis=$(echo "$boundary" | jq -r '.axis')
    segment_start=$(echo "$boundary" | jq -r '.segment_start')
    segment_end=$(echo "$boundary" | jq -r '.segment_end')
    map_json=$(_map_json)
    split_json=$(_matching_split_json "$map_json" "$axis" "$before_a" "$before_b")

    local handle_x="" handle_y="" split_rect_x="" split_rect_y="" split_rect_w="" split_rect_h=""
    if [[ -n "$split_json" && "$split_json" != "null" ]]; then
        split_rect_x=$(echo "$split_json" | jq -r '.rect.x')
        split_rect_y=$(echo "$split_json" | jq -r '.rect.y')
        split_rect_w=$(echo "$split_json" | jq -r '.rect.width')
        split_rect_h=$(echo "$split_json" | jq -r '.rect.height')
    fi

    if [[ "$axis" == "vertical" ]]; then
        local window_width
        if [[ -n "$split_json" && "$split_json" != "null" ]]; then
            handle_x=$(echo "$split_json" | jq -r '.rect.x + (.rect.width * .ratio)')
        else
            handle_x=$(echo "$boundary" | jq -r '.handle_x')
        fi
        window_width=$(echo "$map_json" | jq -r '.window_rect.width')
        start_x=$(_round_int "$handle_x")
        end_x=$(_clamp_int $(( start_x + delta )) 5 $(( $(_round_int "$window_width") - 5 )))
    else
        local window_height
        if [[ -n "$split_json" && "$split_json" != "null" ]]; then
            handle_y=$(echo "$split_json" | jq -r '.rect.y + (.rect.height * .ratio)')
        else
            handle_y=$(echo "$boundary" | jq -r '.handle_y')
        fi
        window_height=$(echo "$map_json" | jq -r '.window_rect.height')
        start_y=$(_round_int "$handle_y")
        end_y=$(_clamp_int $(( start_y + delta )) 5 $(( $(_round_int "$window_height") - 5 )))
    fi

    local before_handle after_handle after_a after_b after_boundary
    if [[ "$axis" == "vertical" ]]; then
        before_handle=$(echo "$boundary" | jq -r '.handle_x')
    else
        before_handle=$(echo "$boundary" | jq -r '.handle_y')
    fi

    local moved=false attempt_index
    for attempt_index in 1 2 3; do
        if [[ "$axis" == "vertical" ]]; then
            case "$attempt_index" in
                1) start_y=$(_round_int "$(echo "$split_rect_y $split_rect_h" | awk '{ print $1 + ($2 / 2) }')") ;;
                2) start_y=$(_round_int "$(echo "$split_rect_y $split_rect_h" | awk '{ print $1 + ($2 * 0.25) }')") ;;
                3) start_y=$(_round_int "$(echo "$split_rect_y $split_rect_h" | awk '{ print $1 + ($2 * 0.75) }')") ;;
            esac
            [[ -z "$split_rect_h" ]] && start_y=$(_round_int "$(awk -v a="$segment_start" -v b="$segment_end" 'BEGIN { print (a + b) / 2 }')")
            end_y="$start_y"
        else
            case "$attempt_index" in
                1) start_x=$(_round_int "$(echo "$split_rect_x $split_rect_w" | awk '{ print $1 + ($2 / 2) }')") ;;
                2) start_x=$(_round_int "$(echo "$split_rect_x $split_rect_w" | awk '{ print $1 + ($2 * 0.25) }')") ;;
                3) start_x=$(_round_int "$(echo "$split_rect_x $split_rect_w" | awk '{ print $1 + ($2 * 0.75) }')") ;;
            esac
            [[ -z "$split_rect_w" ]] && start_x=$(_round_int "$(awk -v a="$segment_start" -v b="$segment_end" 'BEGIN { print (a + b) / 2 }')")
            end_x="$start_x"
        fi

        local platform display wid
        platform=$(session_get platform)
        case "$platform" in
            linux)
                display=$(session_display)
                wid=$(session_get window_id)
                DISPLAY="$display" xdotool windowfocus "$wid" 2>/dev/null || true
                DISPLAY="$display" xdotool mousemove --window "$wid" "$start_x" "$start_y"
                DISPLAY="$display" xdotool mousedown 1
                sleep 0.1
                DISPLAY="$display" xdotool mousemove --window "$wid" "$end_x" "$end_y"
                sleep 0.1
                DISPLAY="$display" xdotool mouseup 1
                ;;
            darwin)
                cliclick "dd:$start_x,$start_y"
                sleep 0.1
                cliclick "dm:$end_x,$end_y"
                sleep 0.1
                cliclick "du:$end_x,$end_y"
                ;;
        esac
        _invalidate_ocr_cache
        sleep 0.3

        after_a=$(_unique_pane_json "$pane_a_title")
        after_b=$(_unique_pane_json "$pane_b_title")
        after_boundary=$(_shared_boundary_json "$after_a" "$after_b")
        [[ -n "$after_boundary" ]] || die "Resize succeeded but panes are no longer adjacent: \"$pane_a_title\" / \"$pane_b_title\""

        if [[ "$axis" == "vertical" ]]; then
            after_handle=$(echo "$after_boundary" | jq -r '.handle_x')
        else
            after_handle=$(echo "$after_boundary" | jq -r '.handle_y')
        fi

        if ! _float_close_enough "$before_handle" "$after_handle" 4; then
            moved=true
            break
        fi
    done

    [[ "$moved" == "true" ]] || die "Resize handle did not move for \"$pane_a_title\" / \"$pane_b_title\""

    jq -n \
        --arg axis "$axis" \
        --argjson before "$before_handle" \
        --argjson after "$after_handle" \
        --arg start_x "$start_x" \
        --arg start_y "$start_y" \
        --arg end_x "$end_x" \
        --arg end_y "$end_y" \
        '{
            axis: $axis,
            before_handle: $before,
            after_handle: $after,
            drag: {
                start_x: ($start_x | tonumber),
                start_y: ($start_y | tonumber),
                end_x: ($end_x | tonumber),
                end_y: ($end_y | tonumber)
            }
        }'
}

cmd_assert() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop assert <subcommand> [...]"
    require_session

    local subcmd="$1"; shift
    case "$subcmd" in
        pane-count)
            [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop assert pane-count N"
            local expected actual
            expected="$1"
            actual=$(echo "$(_map_json)" | jq '.panes | length')
            [[ "$actual" -eq "$expected" ]] ||
                die "Expected $expected panes, found $actual"
            ;;
        window-size)
            [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop assert window-size WIDTH HEIGHT"
            local expected_w expected_h actual_w actual_h
            expected_w="$1"; expected_h="$2"
            actual_w=$(echo "$(_map_json)" | jq '.window_rect.width | round')
            actual_h=$(echo "$(_map_json)" | jq '.window_rect.height | round')
            [[ "$actual_w" -eq "$expected_w" && "$actual_h" -eq "$expected_h" ]] ||
                die "Expected window ${expected_w}x${expected_h}, found ${actual_w}x${actual_h}"
            ;;
        tab-present)
            [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop assert tab-present \"TITLE\""
            local title count
            title="$1"
            count=$(echo "$(_map_json)" | jq --arg title "$title" '[.panes[].titles[] | select(. == $title)] | length')
            [[ "$count" -gt 0 ]] || die "Tab not present in live map: \"$title\""
            ;;
        tab-in-pane)
            [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop assert tab-in-pane \"TAB\" \"PANE TITLE\""
            local tab_title pane_title pane_json has_tab
            tab_title="$1"; pane_title="$2"
            pane_json=$(_unique_pane_json "$pane_title")
            has_tab=$(echo "$pane_json" | jq --arg title "$tab_title" '(.titles | index($title)) != null')
            [[ "$has_tab" == "true" ]] ||
                die "Tab \"$tab_title\" not found in pane \"$pane_title\""
            ;;
        tab-order)
            [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop assert tab-order \"PANE\" \"TAB A\" \"TAB B\" [...]"
            local pane_spec pane_json expected actual
            pane_spec="$1"
            shift
            pane_json=$(_unique_pane_json "$pane_spec")
            expected=$(printf '%s\n' "$@" | jq -R . | jq -s .)
            actual=$(echo "$pane_json" | jq '.titles')
            [[ "$actual" == "$expected" ]] ||
                die "Expected tab order $expected in pane \"$pane_spec\", found $actual"
            ;;
        pane-left-of|pane-right-of|pane-above|pane-below)
            [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop assert $subcmd \"PANE A\" \"PANE B\""
            local pane_a pane_b rect_a rect_b
            pane_a=$(_unique_pane_json "$1")
            pane_b=$(_unique_pane_json "$2")
            rect_a=$(echo "$pane_a" | jq '.rect')
            rect_b=$(echo "$pane_b" | jq '.rect')
            case "$subcmd" in
                pane-left-of)
                    jq -n -e --argjson a "$rect_a" --argjson b "$rect_b" '$a.x < $b.x' >/dev/null ||
                        die "Pane \"$1\" is not left of \"$2\""
                    ;;
                pane-right-of)
                    jq -n -e --argjson a "$rect_a" --argjson b "$rect_b" '$a.x > $b.x' >/dev/null ||
                        die "Pane \"$1\" is not right of \"$2\""
                    ;;
                pane-above)
                    jq -n -e --argjson a "$rect_a" --argjson b "$rect_b" '$a.y < $b.y' >/dev/null ||
                        die "Pane \"$1\" is not above \"$2\""
                    ;;
                pane-below)
                    jq -n -e --argjson a "$rect_a" --argjson b "$rect_b" '$a.y > $b.y' >/dev/null ||
                        die "Pane \"$1\" is not below \"$2\""
                    ;;
            esac
            ;;
        maximized)
            [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop assert maximized \"PANE\""
            local expected_pane map_json
            expected_pane="$1"
            map_json=$(_map_json)
            jq -e --arg pane "$expected_pane" '
                (.maximized_pane_id == $pane) or (.maximized_active_title == $pane)
            ' <<<"$map_json" >/dev/null ||
                die "Pane \"$expected_pane\" is not maximized"
            ;;
        not-maximized)
            local map_json
            map_json=$(_map_json)
            jq -e '.maximized_pane_id == null and .maximized_active_title == null' <<<"$map_json" >/dev/null ||
                die "Expected no maximized pane"
            ;;
        no-debug-overlay)
            local tsv
            tsv=$(_get_ocr_tsv)
            if echo "$tsv" | grep -qiE 'Press|F12|debug metrics'; then
                die "Detected debug overlay text in OCR output"
            fi
            ;;
        *)
            die "Unknown assert subcommand: $subcmd"
            ;;
    esac
}

cmd_drop_tab() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop drop-tab \"SOURCE\" (--edge EDGE | --pane PANE --region REGION [--after TAB])"
    require_session

    local src_name="$1"
    shift
    local edge="" pane_spec="" region="" after_tab=""

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --edge) edge="$2"; shift 2 ;;
            --pane) pane_spec="$2"; shift 2 ;;
            --region) region="$2"; shift 2 ;;
            --after) after_tab="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    local drop_x drop_y target_json
    if [[ -n "$edge" ]]; then
        target_json=$(_app_edge_target_json "$edge")
        [[ -n "$target_json" && "$target_json" != "null" ]] || die "Unknown edge target: $edge"
        read -r drop_x drop_y <<< "$(_center_of_rect "$(echo "$target_json" | jq -c '.rect')")"
    else
        [[ -n "$pane_spec" && -n "$region" ]] ||
            die "Usage: visual-qa-desktop drop-tab \"SOURCE\" (--edge EDGE | --pane PANE --region REGION [--after TAB])"
        target_json=$(_pane_drop_target_json "$pane_spec" "$region")
        [[ -n "$target_json" && "$target_json" != "null" ]] || die "Pane target not found: \"$pane_spec\" region \"$region\""

        if [[ "$region" == "tabbar" && -n "$after_tab" ]]; then
            local pane_json anchor_x anchor_y min_drop max_drop
            pane_json=$(_unique_pane_json "$pane_spec")
            anchor_x=$(cmd_locate "$after_tab" 2>/dev/null | cut -d' ' -f1) || die "Anchor tab not found: \"$after_tab\""
            anchor_y=$(cmd_locate "$after_tab" 2>/dev/null | cut -d' ' -f2) || die "Anchor tab not found: \"$after_tab\""
            min_drop=$(_round_int "$(echo "$pane_json" | jq -r '.tab_bar_rect.x + 20')")
            max_drop=$(_round_int "$(echo "$pane_json" | jq -r '.tab_bar_rect.x + .tab_bar_rect.width - 20')")
            drop_x=$(_clamp_int $(( anchor_x + 80 )) "$min_drop" "$max_drop")
            drop_y="$anchor_y"
        else
            read -r drop_x drop_y <<< "$(_center_of_rect "$(echo "$target_json" | jq -c '.rect')")"
        fi
    fi

    _perform_tab_drag "$src_name" "$drop_x" "$drop_y"
}

cmd_reorder_tab() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop reorder-tab \"SOURCE\" \"ANCHOR\""
    local src_name="$1" anchor="$2"
    local pane_json pane_id
    pane_json=$(_unique_pane_json "$anchor")
    pane_id=$(echo "$pane_json" | jq -r '.pane_id')
    cmd_drop_tab "$src_name" --pane "$pane_id" --region tabbar --after "$anchor"
}

cmd_tab_transfer() {
    [[ $# -ge 2 ]] || die "Usage: visual-qa-desktop tab-transfer \"source tab\" \"target tab\""
    local src_name="$1" tgt_name="$2"
    local pane_json pane_id
    pane_json=$(_unique_pane_json "$tgt_name")
    pane_id=$(echo "$pane_json" | jq -r '.pane_id')
    cmd_drop_tab "$src_name" --pane "$pane_id" --region tabbar --after "$tgt_name"
}

cmd_maximize() {
    require_session
    local tab_name="${1:-}"

    local tab_pos
    if [[ -n "$tab_name" ]]; then
        tab_pos=$(cmd_locate "$tab_name" 2>/dev/null) || die "Tab not found: \"$tab_name\""
    else
        tab_pos=$(cmd_locate --all --y-range 25 70 2>/dev/null | head -1)
        [[ -n "$tab_pos" ]] || die "No tabs found"
    fi

    local tx ty
    tx=$(echo "$tab_pos" | cut -d' ' -f1); ty=$(echo "$tab_pos" | cut -d' ' -f2)

    cmd_right_click "$tx" "$ty"
    sleep 0.3
    local max_pos
    max_pos=$(cmd_locate "Maximize" 2>/dev/null) || die "Maximize menu item not found — pane may already be maximized"
    cmd_click $(echo "$max_pos")
    sleep 0.3
}

cmd_restore() {
    require_session

    local tab_pos
    tab_pos=$(cmd_locate --all --y-range 25 70 2>/dev/null | head -1)
    [[ -n "$tab_pos" ]] || die "No tabs found"

    local tx ty
    tx=$(echo "$tab_pos" | cut -d' ' -f1); ty=$(echo "$tab_pos" | cut -d' ' -f2)

    cmd_right_click "$tx" "$ty"
    sleep 0.3
    local rest_pos
    rest_pos=$(cmd_locate "Restore" 2>/dev/null) || die "Restore menu item not found — layout may not be maximized"
    cmd_click $(echo "$rest_pos")
    sleep 0.3
}

cmd_cleanup() {
    local screenshots_root="$ROOT_DIR/testdata/screenshots"
    local keep="${1:-3}"

    if [[ ! -d "$screenshots_root" ]]; then
        info "No screenshots directory"
        return 0
    fi

    local dirs
    dirs=$(ls -1d "$screenshots_root"/20* 2>/dev/null | sort -r)
    local count=0
    local removed=0
    while IFS= read -r dir; do
        [[ -z "$dir" ]] && continue
        count=$((count + 1))
        if [[ $count -gt $keep ]]; then
            rm -rf "$dir"
            removed=$((removed + 1))
        fi
    done <<< "$dirs"
    info "Cleaned up $removed old screenshot runs (kept $keep)"
}
