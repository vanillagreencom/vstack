cmd_setup() {
    local os
    os="$(uname -s)"
    echo "Platform: $os"

    local missing=()
    local available=()
    local install_pkgs=()

    check_dep() {
        local cmd_name="$1"
        local display_name="$2"
        local package_name="$3"
        if command -v "$cmd_name" &>/dev/null; then
            available+=("$display_name")
        else
            missing+=("$display_name")
            install_pkgs+=("$package_name")
        fi
    }

    case "$os" in
        Linux)
            check_dep Xvfb Xvfb xorg-server-xvfb
            check_dep xdotool xdotool xdotool
            check_dep maim maim maim
            check_dep ffmpeg ffmpeg ffmpeg
            check_dep jq jq jq
            check_dep tesseract tesseract tesseract
            check_dep magick ImageMagick imagemagick

            if command -v tesseract &>/dev/null &&
                ! tesseract --list-langs 2>/dev/null | grep -qx "eng"; then
                missing+=("tesseract-data-eng")
                install_pkgs+=("tesseract-data-eng")
            fi

            local has_swrast=false
            if vulkaninfo 2>/dev/null | grep -qi "lavapipe\|llvmpipe\|SwiftShader" 2>/dev/null; then
                has_swrast=true
                available+=("vulkan-swrast")
            elif pacman -Qs vulkan-swrast &>/dev/null 2>/dev/null; then
                has_swrast=true
                available+=("vulkan-swrast")
            else
                missing+=("vulkan-swrast")
            fi

            echo "Available: ${available[*]:-none}"
            if [[ ${#missing[@]} -gt 0 ]]; then
                echo "Missing: ${missing[*]}"
                echo ""
                echo "Install with:"
                echo "  sudo pacman -S ${install_pkgs[*]}"
                return 1
            fi

            echo "Status: READY"
            echo "Mode: xvfb-isolated (your mouse/keyboard untouched)"
            ;;
        Darwin)
            check_dep cliclick cliclick cliclick
            check_dep ffmpeg ffmpeg ffmpeg
            check_dep jq jq jq
            check_dep tesseract tesseract tesseract
            check_dep magick ImageMagick imagemagick

            if ! command -v screencapture &>/dev/null; then
                missing+=("screencapture")
            else
                available+=("screencapture")
            fi

            echo "Available: ${available[*]:-none}"
            if [[ ${#missing[@]} -gt 0 ]]; then
                echo "Missing: ${missing[*]}"
                echo ""
                echo "Install with:"
                echo "  brew install ${install_pkgs[*]}"
                if [[ " ${missing[*]} " == *" screencapture "* ]]; then
                    echo "  screencapture should be present on macOS by default"
                fi
                return 1
            fi

            echo "Status: READY (with limitations)"
            echo "Mode: direct (will take over mouse — confirmation required)"
            warn "macOS cannot isolate input. Your mouse/keyboard will be used."
            ;;
        *)
            die "Unsupported platform: $os"
            ;;
    esac
}

cmd_start() {
    local size="1920x1080" build=false depth=24 scale=1 layout=""
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --size) size="$2"; shift 2 ;;
            --build) build=true; shift ;;
            --scale) scale="$2"; shift 2 ;;
            --hidpi) scale=2; shift ;;
            --layout) layout="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    if [[ -n "$layout" ]] && [[ ! -f "$layout" ]]; then
        die "Layout fixture not found: $layout"
    fi
    if [[ -n "$layout" ]] && [[ "$VQA_SUPPORTS_LAYOUT" != "true" ]]; then
        die "VQA_TARGET=${VQA_TARGET} does not support --layout"
    fi

    if [[ -f "$SESSION_FILE" ]]; then
        warn "Cleaning up stale session"
        cmd_stop 2>/dev/null || true
    fi

    _new_screenshot_dir
    mkdir -p "$SCREENSHOT_DIR"
    _invalidate_ocr_cache

    local os
    os="$(uname -s)"

    case "$os" in
        Linux) _start_linux "$size" "$depth" "$build" "$scale" "$layout" ;;
        Darwin) _start_mac "$size" "$build" "$scale" "$layout" ;;
        *) die "Unsupported platform: $os" ;;
    esac
}

_start_linux() {
    local size="$1" depth="$2" build="$3" scale="${4:-1}" layout="${5:-}"
    local width height
    width="${size%x*}"
    height="${size#*x}"

    local phys_width=$((width * scale))
    local phys_height=$((height * scale))
    local dpi=$((96 * scale))

    local display
    display=$(find_free_display)
    if [[ "$scale" -gt 1 ]]; then
        info "Using virtual display :$display (${phys_width}x${phys_height}x${depth}, scale=${scale}x, logical ${width}x${height})"
    else
        info "Using virtual display :$display (${width}x${height}x${depth})"
    fi

    local session_log_dir="$SCREENSHOT_DIR/session-logs"
    mkdir -p "$session_log_dir"
    local xvfb_log="$session_log_dir/xvfb.log"
    local app_log="$session_log_dir/app.log"
    local map_path=""
    if [[ "$VQA_SUPPORTS_MAP" == "true" ]]; then
        map_path="$SCREENSHOT_DIR/live-map.json"
    fi

    trap 'cmd_stop 2>/dev/null; exit 130' INT TERM
    nohup setsid Xvfb ":$display" -screen 0 "${phys_width}x${phys_height}x${depth}" -dpi "$dpi" -ac -nolisten tcp \
        >"$xvfb_log" 2>&1 < /dev/null &
    local xvfb_pid=$!
    sleep 1

    if ! kill -0 "$xvfb_pid" 2>/dev/null; then
        die "Xvfb failed to start"
    fi
    info "Xvfb started (PID $xvfb_pid)"

    local binary="$VQA_APP_BINARY"
    if [[ "$build" == "true" ]]; then
        info "Building ${VQA_TARGET_NAME}..."
        if ! build_target 2>&1; then
            kill "$xvfb_pid" 2>/dev/null
            die "Build failed"
        fi
    fi

    if [[ ! -x "$binary" ]]; then
        kill "$xvfb_pid" 2>/dev/null
        die "Binary not found: $binary (use --build to compile)"
    fi

    info "Launching ${VQA_TARGET_NAME} in virtual display..."
    local launch_env=(
        -u WAYLAND_DISPLAY
        -u XDG_SESSION_TYPE
        "DISPLAY=:$display"
        "WINIT_UNIX_BACKEND=x11"
        "WINIT_X11_SCALE_FACTOR=$scale"
        "WGPU_BACKEND=${WGPU_BACKEND:-vulkan}"
    )
    if [[ -n "$layout" ]]; then
        launch_env+=("${VQA_LAYOUT_ENV_KEY}=$layout")
    elif [[ -n "$VQA_DEFAULT_LAYOUT_ENV" ]]; then
        local default_env=()
        read -r -a default_env <<< "$VQA_DEFAULT_LAYOUT_ENV"
        launch_env+=("${default_env[@]}")
    fi
    if [[ -n "$VQA_MAP_ENV_KEY" ]] && [[ -n "$map_path" ]]; then
        launch_env+=("${VQA_MAP_ENV_KEY}=$map_path")
    fi
    nohup setsid env "${launch_env[@]}" "$binary" >"$app_log" 2>&1 < /dev/null &
    local app_pid=$!
    sleep 2

    if ! kill -0 "$app_pid" 2>/dev/null; then
        kill "$xvfb_pid" 2>/dev/null
        die "${VQA_TARGET_NAME} failed to start in virtual display"
    fi

    local wid
    wid=$(wait_for_window "$display" "$VQA_WINDOW_TITLE" 30) || {
        kill "$app_pid" 2>/dev/null
        kill "$xvfb_pid" 2>/dev/null
        die "Window '$VQA_WINDOW_TITLE' not found within 15s"
    }
    info "Window found (ID: $wid)"

    DISPLAY=":$display" xdotool windowfocus "$wid" 2>/dev/null || true
    DISPLAY=":$display" xdotool windowsize "$wid" "$phys_width" "$phys_height" 2>/dev/null || true
    DISPLAY=":$display" xdotool windowmove "$wid" 0 0 2>/dev/null || true
    sleep 0.5

    local geom
    geom=$(DISPLAY=":$display" xdotool getwindowgeometry --shell "$wid" 2>/dev/null)
    local win_x win_y win_w win_h
    win_x=$(echo "$geom" | grep "^X=" | cut -d= -f2)
    win_y=$(echo "$geom" | grep "^Y=" | cut -d= -f2)
    win_w=$(echo "$geom" | grep "^WIDTH=" | cut -d= -f2)
    win_h=$(echo "$geom" | grep "^HEIGHT=" | cut -d= -f2)

    cat > "$SESSION_FILE" <<EOF
{
    "display": $display,
    "xvfb_pid": $xvfb_pid,
    "app_pid": $app_pid,
    "window_id": $wid,
    "window_x": ${win_x:-0},
    "window_y": ${win_y:-0},
    "window_width": ${win_w:-$width},
    "window_height": ${win_h:-$height},
    "scale": $scale,
    "platform": "linux",
    "mode": "xvfb-isolated",
    "layout_fixture": "$layout",
    "map_path": "$map_path",
    "screenshot_dir": "$SCREENSHOT_DIR",
    "screenshot_count": 0
}
EOF

    echo "Session started successfully."
    cat "$SESSION_FILE"

    info "Pre-warming OCR cache..."
    _get_ocr_tsv > /dev/null 2>&1 || true

    if [[ "$VQA_AUTO_WATCH" == "true" ]]; then
        cmd_watch
    fi
}

_start_mac() {
    local size="$1" build="$2" scale="${3:-1}" layout="${4:-}"
    local width height
    width="${size%x*}"
    height="${size#*x}"

    warn "macOS mode: will use your real mouse/keyboard"

    local binary="$VQA_APP_BINARY"
    local map_path=""
    if [[ "$VQA_SUPPORTS_MAP" == "true" ]]; then
        map_path="$SCREENSHOT_DIR/live-map.json"
    fi
    if [[ "$build" == "true" ]]; then
        info "Building ${VQA_TARGET_NAME}..."
        if ! build_target 2>&1; then
            die "Build failed"
        fi
    fi

    if [[ ! -x "$binary" ]]; then
        die "Binary not found: $binary (use --build to compile)"
    fi

    local launch_env=()
    if [[ -n "$layout" ]]; then
        launch_env+=("${VQA_LAYOUT_ENV_KEY}=$layout")
    elif [[ -n "$VQA_DEFAULT_LAYOUT_ENV" ]]; then
        local default_env=()
        read -r -a default_env <<< "$VQA_DEFAULT_LAYOUT_ENV"
        launch_env+=("${default_env[@]}")
    fi
    if [[ -n "$VQA_MAP_ENV_KEY" ]] && [[ -n "$map_path" ]]; then
        launch_env+=("${VQA_MAP_ENV_KEY}=$map_path")
    fi
    env "${launch_env[@]}" "$binary" &
    local app_pid=$!
    sleep 3

    if ! kill -0 "$app_pid" 2>/dev/null; then
        die "${VQA_TARGET_NAME} failed to start"
    fi

    local wid
    wid=$(osascript -e "tell application \"System Events\" to get id of first window of (first process whose name is \"$VQA_PROCESS_NAME\")" 2>/dev/null || echo "unknown")

    cat > "$SESSION_FILE" <<EOF
{
    "display": 0,
    "xvfb_pid": 0,
    "app_pid": $app_pid,
    "window_id": "$wid",
    "window_x": 0,
    "window_y": 0,
    "window_width": $width,
    "window_height": $height,
    "scale": $scale,
    "platform": "darwin",
    "mode": "direct",
    "layout_fixture": "$layout",
    "map_path": "$map_path",
    "screenshot_dir": "$SCREENSHOT_DIR",
    "screenshot_count": 0
}
EOF

    echo "Session started (macOS direct mode)."
    cat "$SESSION_FILE"
}

cmd_screenshot() {
    require_session
    _invalidate_ocr_cache
    _resolve_screenshot_dir
    local output=""
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --output) output="$2"; shift 2 ;;
            *) die "Unknown option: $1" ;;
        esac
    done

    if [[ -z "$output" ]]; then
        mkdir -p "$SCREENSHOT_DIR"
        local count
        count=$(jq -r '.screenshot_count // 0' "$SESSION_FILE")
        count=$((count + 1))
        output="$SCREENSHOT_DIR/screenshot_$(printf '%03d' "$count").png"
        jq ".screenshot_count = $count" "$SESSION_FILE" > "$SESSION_FILE.tmp" && mv "$SESSION_FILE.tmp" "$SESSION_FILE"
    fi

    local platform
    platform=$(session_get platform)
    local display wid

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            if command -v maim &>/dev/null; then
                DISPLAY="$display" maim -i "$wid" "$output" 2>/dev/null || \
                DISPLAY="$display" maim "$output" 2>/dev/null || \
                DISPLAY="$display" import -window root "$output"
            else
                DISPLAY="$display" import -window root "$output"
            fi
            ;;
        darwin)
            screencapture -x "$output"
            ;;
    esac

    if [[ -f "$output" ]]; then
        echo "$output"
    else
        die "Screenshot failed"
    fi
}

cmd_window_info() {
    require_session
    local platform display wid
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            wid=$(session_get window_id)
            local geom
            geom=$(DISPLAY="$display" xdotool getwindowgeometry --shell "$wid" 2>/dev/null)
            local win_x win_y win_w win_h
            win_x=$(echo "$geom" | grep "^X=" | cut -d= -f2)
            win_y=$(echo "$geom" | grep "^Y=" | cut -d= -f2)
            win_w=$(echo "$geom" | grep "^WIDTH=" | cut -d= -f2)
            win_h=$(echo "$geom" | grep "^HEIGHT=" | cut -d= -f2)
            jq -n \
                --arg wid "$wid" \
                --arg x "${win_x:-0}" \
                --arg y "${win_y:-0}" \
                --arg w "${win_w:-0}" \
                --arg h "${win_h:-0}" \
                '{window_id: $wid, x: ($x|tonumber), y: ($y|tonumber), width: ($w|tonumber), height: ($h|tonumber)}'
            ;;
        darwin)
            cat "$SESSION_FILE"
            ;;
    esac
}

cmd_map() {
    echo "$(_map_json)" | jq '.'
}

cmd_status() {
    if [[ ! -f "$SESSION_FILE" ]]; then
        jq -n '{active: false}'
        return 0
    fi

    local session_json platform display wid
    session_json=$(cat "$SESSION_FILE")
    platform=$(echo "$session_json" | jq -r '.platform // empty')
    display=$(echo "$session_json" | jq -r '.display // empty')
    wid=$(echo "$session_json" | jq -r '.window_id // empty')

    local app_pid viewer_pid xvfb_pid ffmpeg_pid map_path
    app_pid=$(echo "$session_json" | jq -r '.app_pid // empty')
    viewer_pid=$(echo "$session_json" | jq -r '.viewer_pid // empty')
    xvfb_pid=$(echo "$session_json" | jq -r '.xvfb_pid // empty')
    ffmpeg_pid=$(echo "$session_json" | jq -r '.ffmpeg_pid // empty')
    map_path=$(echo "$session_json" | jq -r '.map_path // empty')

    local app_alive viewer_alive xvfb_alive recording_alive display_reachable map_ready pane_count
    app_alive=$([[ -n "$app_pid" ]] && kill -0 "$app_pid" 2>/dev/null && echo true || echo false)
    viewer_alive=$([[ -n "$viewer_pid" ]] && kill -0 "$viewer_pid" 2>/dev/null && echo true || echo false)
    xvfb_alive=$([[ -n "$xvfb_pid" && "$xvfb_pid" != "0" ]] && kill -0 "$xvfb_pid" 2>/dev/null && echo true || echo false)
    recording_alive=$([[ -n "$ffmpeg_pid" ]] && kill -0 "$ffmpeg_pid" 2>/dev/null && echo true || echo false)

    if [[ "$platform" == "linux" ]]; then
        display_reachable=$(
            DISPLAY=":$display" xdotool getwindowgeometry --shell "$wid" >/dev/null 2>&1 &&
                echo true || echo false
        )
    else
        display_reachable=true
    fi

    if [[ -n "$map_path" ]] && [[ -f "$map_path" ]]; then
        map_ready=true
        pane_count=$(jq '.panes | length' "$map_path" 2>/dev/null || echo 0)
    else
        map_ready=false
        pane_count=0
    fi

    echo "$session_json" | jq \
        --argjson active true \
        --argjson app_alive "$app_alive" \
        --argjson viewer_alive "$viewer_alive" \
        --argjson xvfb_alive "$xvfb_alive" \
        --argjson recording_alive "$recording_alive" \
        --argjson display_reachable "$display_reachable" \
        --arg map_path "$map_path" \
        --argjson map_ready "$map_ready" \
        --argjson pane_count "$pane_count" \
        '. + {
            active: $active,
            health: {
                app_alive: $app_alive,
                viewer_alive: $viewer_alive,
                xvfb_alive: $xvfb_alive,
                recording_alive: $recording_alive,
                display_reachable: $display_reachable
            },
            map: {
                path: $map_path,
                ready: $map_ready,
                pane_count: $pane_count
            }
        }'
}

cmd_doctor() {
    local setup_status=0 setup_output status_json
    setup_output=$(cmd_setup 2>&1) || setup_status=$?
    echo "$setup_output"
    echo ""

    status_json=$(cmd_status)
    echo "$status_json"

    if [[ $setup_status -ne 0 ]]; then
        return 1
    fi

    if [[ "$(echo "$status_json" | jq -r '.active')" != "true" ]]; then
        return 0
    fi

    echo "$status_json" | jq -e '
        .health.app_alive
        and .health.display_reachable
        and (.health.xvfb_alive or .platform != "linux")
        and (.map.ready or (.map.path == ""))
    ' >/dev/null
}

cmd_record() {
    [[ $# -ge 1 ]] || die "Usage: visual-qa-desktop record start|stop [--output PATH]"
    require_session
    _resolve_screenshot_dir
    local subcmd="$1"; shift

    case "$subcmd" in
        start)
            mkdir -p "$SCREENSHOT_DIR"
            local output="$SCREENSHOT_DIR/recording.mp4"
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --output) output="$2"; shift 2 ;;
                    *) die "Unknown option: $1" ;;
                esac
            done

            local platform display
            platform=$(session_get platform)
            local win_w win_h
            win_w=$(session_get window_width)
            win_h=$(session_get window_height)

            case "$platform" in
                linux)
                    display=$(session_display)
                    local session_log_dir record_log
                    session_log_dir="$(session_get screenshot_dir)/session-logs"
                    mkdir -p "$session_log_dir"
                    record_log="$session_log_dir/recording.log"
                    nohup setsid ffmpeg -f x11grab -video_size "${win_w}x${win_h}" \
                        -framerate 30 -i "$display" \
                        -c:v libx264 -preset ultrafast -pix_fmt yuv420p \
                        "$output" >"$record_log" 2>&1 < /dev/null &
                    local ffmpeg_pid=$!
                    jq ".ffmpeg_pid = $ffmpeg_pid | .recording_path = \"$output\"" \
                        "$SESSION_FILE" > "$SESSION_FILE.tmp" && mv "$SESSION_FILE.tmp" "$SESSION_FILE"
                    info "Recording to $output (PID $ffmpeg_pid)"
                    ;;
                darwin)
                    screencapture -v "$output" &
                    local sc_pid=$!
                    jq ".ffmpeg_pid = $sc_pid | .recording_path = \"$output\"" \
                        "$SESSION_FILE" > "$SESSION_FILE.tmp" && mv "$SESSION_FILE.tmp" "$SESSION_FILE"
                    info "Recording to $output (PID $sc_pid)"
                    ;;
            esac
            ;;
        stop)
            local ffmpeg_pid
            ffmpeg_pid=$(session_get ffmpeg_pid)
            if [[ -n "$ffmpeg_pid" ]] && kill -0 "$ffmpeg_pid" 2>/dev/null; then
                kill -INT "$ffmpeg_pid" 2>/dev/null
                local i=0
                while kill -0 "$ffmpeg_pid" 2>/dev/null && [[ $i -lt 20 ]]; do
                    sleep 0.1
                    i=$((i + 1))
                done
                local path
                path=$(session_get recording_path)
                if [[ ! -f "$path" ]]; then
                    die "Recording file missing after stop: $path"
                fi
                local size_bytes
                size_bytes=$(wc -c < "$path" | tr -d ' ')
                if [[ "$size_bytes" -eq 0 ]]; then
                    die "Recording file is empty: $path"
                fi
                info "Recording saved: $path"
                if command -v ffprobe &>/dev/null; then
                    local recording_meta
                    recording_meta=$(ffprobe -v error \
                        -show_entries format=duration,size \
                        -of default=noprint_wrappers=1 \
                        "$path" 2>/dev/null || true)
                    [[ -n "$recording_meta" ]] && printf '%s\n' "$recording_meta" >&2
                else
                    info "Recording size bytes: $size_bytes"
                fi
                echo "$path"
            else
                warn "No active recording"
            fi
            ;;
        *)
            die "Unknown record subcommand: $subcmd (use start|stop)"
            ;;
    esac
}

cmd_watch() {
    require_session
    local platform display
    platform=$(session_get platform)

    case "$platform" in
        linux)
            display=$(session_display)
            local win_w win_h
            win_w=$(session_get window_width)
            win_h=$(session_get window_height)
            info "Opening live viewer (read-only — your mouse won't affect the virtual display)"
            info "Close the viewer window to stop watching."
            local session_log_dir viewer_log
            session_log_dir="$(session_get screenshot_dir)/session-logs"
            mkdir -p "$session_log_dir"
            viewer_log="$session_log_dir/viewer.log"
            nohup setsid ffplay -f x11grab -video_size "${win_w}x${win_h}" \
                -framerate 15 -i "$display" \
                -window_title "Visual QA — Live View (read-only)" \
                -loglevel quiet >"$viewer_log" 2>&1 < /dev/null &
            local viewer_pid=$!
            jq ".viewer_pid = $viewer_pid" "$SESSION_FILE" > "$SESSION_FILE.tmp" && mv "$SESSION_FILE.tmp" "$SESSION_FILE"
            info "Viewer started (PID $viewer_pid)"
            ;;
        darwin)
            warn "macOS: live viewer not available (app is on your real display already)"
            ;;
    esac
}

cmd_stop() {
    if [[ ! -f "$SESSION_FILE" ]]; then
        info "No active session"
        return 0
    fi

    _invalidate_ocr_cache

    _kill_and_wait() {
        local pid="$1" sig="${2:-TERM}" label="${3:-process}"
        if [[ -n "$pid" ]] && [[ "$pid" != "0" ]] && kill -0 "$pid" 2>/dev/null; then
            kill "-$sig" "$pid" 2>/dev/null
            local i=0
            while kill -0 "$pid" 2>/dev/null && [[ $i -lt 20 ]]; do
                sleep 0.1
                i=$((i + 1))
            done
            if kill -0 "$pid" 2>/dev/null; then
                kill -9 "$pid" 2>/dev/null || true
            fi
            info "$label stopped"
        fi
    }

    local ffmpeg_pid
    ffmpeg_pid=$(jq -r '.ffmpeg_pid // empty' "$SESSION_FILE" 2>/dev/null)
    _kill_and_wait "$ffmpeg_pid" INT "Recording"

    local viewer_pid
    viewer_pid=$(jq -r '.viewer_pid // empty' "$SESSION_FILE" 2>/dev/null)
    _kill_and_wait "$viewer_pid" TERM "Viewer"

    local app_pid
    app_pid=$(jq -r '.app_pid // empty' "$SESSION_FILE" 2>/dev/null)
    _kill_and_wait "$app_pid" TERM "App"

    local xvfb_pid
    xvfb_pid=$(jq -r '.xvfb_pid // empty' "$SESSION_FILE" 2>/dev/null)
    _kill_and_wait "$xvfb_pid" TERM "Xvfb"

    rm -f "$SESSION_FILE"
    info "Session cleaned up"
}
