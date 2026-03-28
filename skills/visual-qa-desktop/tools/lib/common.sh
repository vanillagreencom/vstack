_resolve_screenshot_dir() {
    [[ -n "$SCREENSHOT_DIR" ]] && return 0
    if [[ -z "${VISUAL_QA_SCREENSHOT_DIR:-}" ]] && [[ -f "$SESSION_FILE" ]]; then
        SCREENSHOT_DIR="$(jq -r '.screenshot_dir // empty' "$SESSION_FILE" 2>/dev/null)"
    fi
    if [[ -z "$SCREENSHOT_DIR" ]]; then
        local run_id="${VISUAL_QA_RUN_ID:-$(date +%Y%m%d-%H%M%S)}"
        SCREENSHOT_DIR="${VISUAL_QA_SCREENSHOT_DIR:-$ROOT_DIR/testdata/screenshots/$run_id}"
    fi
}

_new_screenshot_dir() {
    local run_id="${VISUAL_QA_RUN_ID:-$(date +%Y%m%d-%H%M%S)}"
    SCREENSHOT_DIR="${VISUAL_QA_SCREENSHOT_DIR:-$ROOT_DIR/testdata/screenshots/$run_id}"
}

die() { echo "ERROR: $*" >&2; exit 1; }
info() { echo "INFO: $*" >&2; }
warn() { echo "WARN: $*" >&2; }

build_target() {
    (cd "$ROOT_DIR" && bash -lc "$VQA_BUILD_CMD")
}

_SESSION_CACHE=""

session_get() {
    [[ -f "$SESSION_FILE" ]] || die "No active session. Run: tools/visual-qa-desktop start"
    if [[ -z "$_SESSION_CACHE" ]]; then
        _SESSION_CACHE=$(cat "$SESSION_FILE")
    fi
    echo "$_SESSION_CACHE" | jq -r --arg field "$1" '.[$field] // empty'
}

session_display() {
    echo ":$(session_get display)"
}

require_session() {
    [[ -f "$SESSION_FILE" ]] || die "No active session. Run: tools/visual-qa-desktop start"
    if [[ -z "$_SESSION_CACHE" ]]; then
        _SESSION_CACHE=$(cat "$SESSION_FILE")
    fi
    local display
    display=$(echo "$_SESSION_CACHE" | jq -r '.display // empty')
    [[ -n "$display" ]] || die "Session file corrupt — missing display"
}

_wait_for_file() {
    local path="$1" max_attempts="${2:-30}"
    local attempt=0
    while [[ $attempt -lt $max_attempts ]]; do
        [[ -f "$path" ]] && return 0
        sleep 0.1
        attempt=$((attempt + 1))
    done
    return 1
}

_round_int() {
    awk -v value="$1" 'BEGIN { printf "%.0f\n", value }'
}

_clamp_int() {
    local value="$1" min="$2" max="$3"
    if [[ "$value" -lt "$min" ]]; then
        echo "$min"
    elif [[ "$value" -gt "$max" ]]; then
        echo "$max"
    else
        echo "$value"
    fi
}

_float_close_enough() {
    local left="$1" right="$2" tolerance="${3:-8}"
    awk -v left="$left" -v right="$right" -v tolerance="$tolerance" '
    BEGIN {
        diff = left - right
        if (diff < 0) diff = -diff
        exit(diff <= tolerance ? 0 : 1)
    }'
}

wait_for_window() {
    local display="$1" name="$2" max_attempts="${3:-20}"
    local attempt=0
    while [[ $attempt -lt $max_attempts ]]; do
        local wid
        wid=$(DISPLAY=":$display" xdotool search --name "$name" 2>/dev/null | head -1)
        if [[ -n "$wid" ]]; then
            echo "$wid"
            return 0
        fi
        sleep 0.5
        attempt=$((attempt + 1))
    done
    return 1
}

find_free_display() {
    local d=90
    while [[ $d -lt 200 ]]; do
        if [[ ! -e "/tmp/.X${d}-lock" ]] && [[ ! -e "/tmp/.X11-unix/X${d}" ]]; then
            echo "$d"
            return 0
        fi
        d=$((d + 1))
    done
    die "No free display number found (checked :90-:199)"
}
