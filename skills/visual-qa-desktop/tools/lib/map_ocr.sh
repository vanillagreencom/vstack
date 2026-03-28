_map_path() {
    local path
    path=$(session_get map_path)
    [[ -n "$path" ]] || die "No live visual-qa-desktop map configured for this session"
    echo "$path"
}

_map_json() {
    require_session
    local path
    path=$(_map_path)
    _wait_for_file "$path" 30 || die "Live visual-qa-desktop map not ready: $path"
    cat "$path"
}

_pane_matches_json() {
    local title="$1"
    _map_json | jq -c --arg title "$title" '
        [.panes[] | select((.titles | index($title)) != null or .active_title == $title)]
    '
}

_unique_pane_json() {
    local title="$1"
    local map_json
    map_json=$(_map_json)

    if [[ "$title" == Pane\(* ]]; then
        local by_id
        by_id=$(echo "$map_json" | jq -c --arg pane_id "$title" '[.panes[] | select(.pane_id == $pane_id)]')
        if [[ "$(echo "$by_id" | jq 'length')" -eq 1 ]]; then
            echo "$by_id" | jq -c '.[0]'
            return 0
        fi
    fi

    local matches count
    matches=$(echo "$map_json" | jq -c --arg title "$title" '
        [.panes[] | select((.titles | index($title)) != null or .active_title == $title)]
    ')
    count=$(echo "$matches" | jq 'length')

    if [[ "$count" -eq 0 ]]; then
        die "Pane not found in live map: \"$title\""
    fi

    if [[ "$count" -gt 1 ]]; then
        die "Pane title is ambiguous in live map: \"$title\""
    fi

    echo "$matches" | jq -c '.[0]'
}

_pane_drop_target_json() {
    local pane_spec="$1" region="$2"
    local pane_json
    pane_json=$(_unique_pane_json "$pane_spec")
    echo "$pane_json" | jq -c --arg region "$region" '
        .drop_targets[] | select(.name == $region)
    '
}

_app_edge_target_json() {
    local edge="$1"
    _map_json | jq -c --arg name "edge-$edge" '.app_edge_targets[] | select(.name == $name)'
}

_center_of_rect() {
    local rect_json="$1"
    jq -r '[((.x + (.width / 2))|round), ((.y + (.height / 2))|round)] | @tsv' <<<"$rect_json"
}

_invalidate_ocr_cache() {
    rm -f "$_OCR_CACHE_FILE" "$_OCR_CACHE_STAMP"
}

_get_ocr_tsv() {
    local display wid scale platform
    display=$(session_display)
    wid=$(session_get window_id)
    scale=$(session_get scale)
    platform=$(session_get platform)
    [[ -z "$scale" || "$scale" == "0" ]] && scale=1

    if [[ -f "$_OCR_CACHE_FILE" && -f "$_OCR_CACHE_STAMP" ]]; then
        cat "$_OCR_CACHE_FILE"
        return 0
    fi

    local tsv
    case "$platform" in
        linux)
            tsv=$(DISPLAY="$display" maim -i "$wid" 2>/dev/null | \
                magick - -resize 200% png:- 2>/dev/null | \
                tesseract stdin stdout --psm 11 --oem 1 tsv 2>/dev/null)
            ;;
        darwin)
            local tmp_capture
            tmp_capture=$(mktemp /tmp/visual-qa-desktop-ocr.XXXXXX)
            if ! screencapture -x -l "$wid" "$tmp_capture" 2>/dev/null; then
                screencapture -x "$tmp_capture" 2>/dev/null || true
            fi
            tsv=$(magick "$tmp_capture" -resize 200% png:- 2>/dev/null | \
                tesseract stdin stdout --psm 11 --oem 1 tsv 2>/dev/null)
            rm -f "$tmp_capture"
            ;;
        *)
            die "OCR unsupported on platform: $platform"
            ;;
    esac

    if [[ -z "$tsv" ]]; then
        die "OCR failed — check capture, ImageMagick, and tesseract dependencies"
    fi

    echo "$tsv" > "$_OCR_CACHE_FILE"
    touch "$_OCR_CACHE_STAMP"
    echo "$tsv"
}

cmd_locate() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop locate \"text\" [--all] [--near X Y]"
    require_session

    local search="" opt_all=false near_x="" near_y="" y_min="" y_max="" min_conf=50
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --all)     opt_all=true; shift ;;
            --near)    near_x="$2"; near_y="$3"; shift 3 ;;
            --y-range) y_min="$2"; y_max="$3"; shift 3 ;;
            --conf)    min_conf="$2"; shift 2 ;;
            *)
                if [[ -z "$search" ]]; then
                    search="$1"
                else
                    search="$search $1"
                fi
                shift ;;
        esac
    done

    local ocr_scale=2
    local tsv
    tsv=$(_get_ocr_tsv)

    if [[ "$opt_all" == "true" && -z "$search" ]]; then
        echo "$tsv" | awk -F'\t' -v y_lo="$y_min" -v y_hi="$y_max" -v mc="$min_conf" '
            NR > 1 && $12 != "" && $11+0 >= mc {
                x = int(($7 + $9/2) / '"$ocr_scale"')
                y = int(($8 + $10/2) / '"$ocr_scale"')
                if (y_lo != "" && (y < y_lo+0 || y > y_hi+0)) next
                printf "%d %d %s\n", x, y, $12
            }
        '
        return 0
    fi

    [[ -n "$search" ]] || die "Usage: visual-qa-desktop locate \"text\" [--all] [--near X Y]"

    local matches
    matches=$(echo "$tsv" | awk -F'\t' -v search="$search" -v os="$ocr_scale" -v y_lo="$y_min" -v y_hi="$y_max" -v mc="$min_conf" '
    BEGIN {
        s = tolower(search)
        n_search_words = split(s, search_words, " ")
    }
    NR > 1 && $12 != "" {
        conf = $11 + 0
        if (conf < mc) next

        block = $3 + 0
        line = $5 + 0
        word_idx = $6 + 0

        wkey = block ":" line ":" word_idx
        word_text[wkey] = tolower($12)
        word_left[wkey] = $7 + 0
        word_top[wkey] = $8 + 0
        word_width[wkey] = $9 + 0
        word_height[wkey] = $10 + 0

        lkey = block ":" line
        if (!(lkey in line_count)) {
            line_count[lkey] = 0
            lines[++num_lines] = lkey
        }
        line_count[lkey]++
        line_words[lkey ":" line_count[lkey]] = wkey
    }
    END {
        found = 0
        for (li = 1; li <= num_lines; li++) {
            lk = lines[li]
            wc = line_count[lk]
            for (wi = 1; wi <= wc - n_search_words + 1; wi++) {
                match_ok = 1
                for (si = 1; si <= n_search_words; si++) {
                    wk = line_words[lk ":" (wi + si - 1)]
                    if (word_text[wk] != search_words[si]) {
                        match_ok = 0
                        break
                    }
                }
                if (match_ok) {
                    first_wk = line_words[lk ":" wi]
                    last_wk = line_words[lk ":" (wi + n_search_words - 1)]
                    min_left = word_left[first_wk]
                    min_top = word_top[first_wk]
                    max_right = word_left[last_wk] + word_width[last_wk]
                    max_bottom = 0
                    for (si = 1; si <= n_search_words; si++) {
                        wk = line_words[lk ":" (wi + si - 1)]
                        t = word_top[wk]
                        b = t + word_height[wk]
                        if (t < min_top) min_top = t
                        if (b > max_bottom) max_bottom = b
                    }
                    cx = int((min_left + max_right) / 2 / os)
                    cy = int((min_top + max_bottom) / 2 / os)
                    if (y_lo != "" && (cy < y_lo+0 || cy > y_hi+0)) continue
                    found++
                    print cx " " cy
                }
            }
        }
        if (found == 0) exit 1
    }
    ')

    if [[ -z "$matches" ]]; then
        die "Text not found: \"$search\""
    fi

    if [[ "$opt_all" == "true" ]]; then
        echo "$matches"
        return 0
    fi

    if [[ -n "$near_x" && -n "$near_y" ]]; then
        echo "$matches" | awk -v nx="$near_x" -v ny="$near_y" '
        {
            dx = $1 - nx; dy = $2 - ny
            d = dx*dx + dy*dy
            if (NR == 1 || d < best_d) { best_d = d; best = $0 }
        }
        END { print best }
        '
        return 0
    fi

    echo "$matches" | sort -t' ' -k2 -n | head -1
}

cmd_cursor_pos() {
    require_session
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            local mouse_info
            mouse_info=$(DISPLAY="$display" xdotool getmouselocation --shell 2>/dev/null)
            local mx my
            mx=$(echo "$mouse_info" | grep "^X=" | cut -d= -f2)
            my=$(echo "$mouse_info" | grep "^Y=" | cut -d= -f2)
            local win_x win_y
            win_x=$(session_get window_x)
            win_y=$(session_get window_y)
            echo "$(( mx - win_x )) $(( my - win_y ))"
            ;;
        darwin)
            local pos
            pos=$(cliclick p 2>/dev/null | tr ',' ' ')
            echo "$pos"
            ;;
    esac
}

_shared_boundary_json() {
    local pane_a="$1" pane_b="$2"
    jq -n --argjson a "$pane_a" --argjson b "$pane_b" '
        def abs_value(v): if v < 0 then -v else v end;
        def max_value(left; right): if left > right then left else right end;
        def min_value(left; right): if left < right then left else right end;
        ($a.rect.x + $a.rect.width) as $a_right |
        ($b.rect.x + $b.rect.width) as $b_right |
        ($a.rect.y + $a.rect.height) as $a_bottom |
        ($b.rect.y + $b.rect.height) as $b_bottom |
        if abs_value($a_right - $b.rect.x) <= 8 then
            {
                axis: "vertical",
                handle_x: (($a_right + $b.rect.x) / 2),
                segment_start: max_value($a.rect.y; $b.rect.y),
                segment_end: min_value($a_bottom; $b_bottom)
            }
        elif abs_value($b_right - $a.rect.x) <= 8 then
            {
                axis: "vertical",
                handle_x: (($b_right + $a.rect.x) / 2),
                segment_start: max_value($a.rect.y; $b.rect.y),
                segment_end: min_value($a_bottom; $b_bottom)
            }
        elif abs_value($a_bottom - $b.rect.y) <= 8 then
            {
                axis: "horizontal",
                handle_y: (($a_bottom + $b.rect.y) / 2),
                segment_start: max_value($a.rect.x; $b.rect.x),
                segment_end: min_value($a_right; $b_right)
            }
        elif abs_value($b_bottom - $a.rect.y) <= 8 then
            {
                axis: "horizontal",
                handle_y: (($b_bottom + $a.rect.y) / 2),
                segment_start: max_value($a.rect.x; $b.rect.x),
                segment_end: min_value($a_right; $b_right)
            }
        else
            empty
        end
    '
}

_matching_split_json() {
    local map_json="$1" axis="$2" pane_a="$3" pane_b="$4"
    echo "$map_json" | jq -c \
        --arg axis "$axis" \
        --argjson a "$pane_a" \
        --argjson b "$pane_b" '
        def center_x(rect): rect.x + (rect.width / 2);
        def center_y(rect): rect.y + (rect.height / 2);
        [
            .splits[]
            | select(.axis == $axis)
            | select(center_x($a.rect) >= .rect.x and center_x($a.rect) <= (.rect.x + .rect.width))
            | select(center_y($a.rect) >= .rect.y and center_y($a.rect) <= (.rect.y + .rect.height))
            | select(center_x($b.rect) >= .rect.x and center_x($b.rect) <= (.rect.x + .rect.width))
            | select(center_y($b.rect) >= .rect.y and center_y($b.rect) <= (.rect.y + .rect.height))
        ]
        | sort_by(.rect.width * .rect.height)
        | .[0]
    '
}
