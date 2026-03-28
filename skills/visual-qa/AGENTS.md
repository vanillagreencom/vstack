# Visual QA

**Version 1.1.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when performing
> visual QA testing. Humans may also find it useful, but guidance here
> is optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

Automated visual testing framework that launches applications in isolated virtual displays (Xvfb on Linux), provides mouse/keyboard interaction primitives, OCR-based element targeting, and golden baseline regression detection. Configurable for any GUI application via `visual-qa.conf`.

## Configuration

Create `visual-qa.conf` in your project root (required — tools fail without it):

```bash
# visual-qa.conf — Required variables
VQA_APP_BINARY="$ROOT_DIR/target/debug/my-app"
VQA_BUILD_CMD="cargo build -p my-app --features dev"
VQA_WINDOW_TITLE="My Application"
VQA_SCREENSHOT_ENV_KEY="MY_APP_SCREENSHOT_DIR"

# Optional — for apps with layout fixtures and live map support
VQA_TARGET_NAME="my-app"
VQA_LAYOUT_ENV_KEY="MY_APP_LAYOUT_FIXTURE"
VQA_MAP_ENV_KEY="MY_APP_MAP_PATH"
VQA_DEFAULT_LAYOUT_ENV="MY_APP_DEFAULT_LAYOUT=1"

# Optional explicit capability overrides
VQA_SUPPORTS_LAYOUT="true"
VQA_SUPPORTS_MAP="true"
VQA_SUPPORTS_BASELINE="true"
VQA_AUTO_WATCH="true"

# Optional baseline directories
VQA_BASELINE_FIXTURES_DIR="$ROOT_DIR/testdata/golden/layouts"
VQA_BASELINE_GOLDEN_DIR="$ROOT_DIR/testdata/golden/screenshots/linux"
VQA_BASELINE_FIXTURE_GLOB="*.fixture"
```

Capability defaults:
- `VQA_SUPPORTS_LAYOUT=true` when `VQA_LAYOUT_ENV_KEY` is set, otherwise `false`
- `VQA_SUPPORTS_MAP=true` when `VQA_MAP_ENV_KEY` is set, otherwise `false`
- `VQA_SUPPORTS_BASELINE` defaults to layout support

### Multi-Target

For projects with multiple testable binaries, branch on `VQA_TARGET` in your config:

```bash
# Default target
VQA_TARGET_NAME="app"
VQA_APP_BINARY="$ROOT_DIR/target/debug/my-app"
VQA_BUILD_CMD="cargo build -p my-app --features dev"
VQA_WINDOW_TITLE="My App"
VQA_SCREENSHOT_ENV_KEY="MY_APP_SCREENSHOT_DIR"
VQA_LAYOUT_ENV_KEY="MY_APP_LAYOUT_FIXTURE"
VQA_MAP_ENV_KEY="MY_APP_MAP_PATH"
VQA_DEFAULT_LAYOUT_ENV="MY_APP_DEFAULT_LAYOUT=1"
VQA_SUPPORTS_LAYOUT="true"
VQA_SUPPORTS_MAP="true"
VQA_SUPPORTS_BASELINE="true"

if [[ "${VQA_TARGET:-}" == "viewer" ]]; then
    VQA_TARGET_NAME="viewer"
    VQA_APP_BINARY="$ROOT_DIR/target/debug/my-viewer"
    VQA_BUILD_CMD="cargo build -p my-viewer"
    VQA_WINDOW_TITLE="Component Viewer"
    VQA_SCREENSHOT_ENV_KEY="MY_VIEWER_SCREENSHOT_DIR"
    VQA_LAYOUT_ENV_KEY=""
    VQA_MAP_ENV_KEY=""
    VQA_DEFAULT_LAYOUT_ENV=""
    VQA_SUPPORTS_LAYOUT="false"
    VQA_SUPPORTS_MAP="false"
    VQA_SUPPORTS_BASELINE="false"
fi
```

Select target: `scripts/visual-qa --target viewer start --build` or `VQA_TARGET=viewer scripts/screenshot`.
Compatibility aliases also exist for repos that already call `tools/visual-qa` and `tools/screenshot`.

## Quick Reference

| User request | Tool |
|-------------|------|
| "Take a screenshot" | `scripts/screenshot --no-build` → Read the PNG |
| "Screenshot with layout" | `scripts/screenshot --no-build --layout path/to/layout-fixture` |
| "Show pane/split map" | `scripts/visual-qa map` on targets with live-map support |
| "Test if drag/resize works" | Start interactive session |
| "Did the UI change?" | `scripts/visual-qa baseline check` |
| "Capture baselines" | `scripts/visual-qa baseline capture` |
| "Full visual QA" | Interactive session with full checklist |

## Interactive Session Lifecycle

### 1. Confirm

Ask user before launching. On Linux: "I'll launch in an isolated virtual display (Xvfb). Your mouse/keyboard won't be affected." On macOS: warn about mouse takeover.

### 2. Start

```bash
scripts/visual-qa start --build                    # First time or after code changes
scripts/visual-qa start --layout path/to/layout-fixture # Specific layout
scripts/visual-qa start                             # Reuse existing binary
```

### 3. Map the Layout

```bash
scripts/visual-qa map              # Pane rectangles, split handles, tab membership
scripts/visual-qa locate --all     # All visible text with positions
```

If the selected target does not expose a live map, skip `map` and rely on `locate`, `status`, `click`, and `screenshot`. Do not use pane helpers on screenshot/OCR-only targets.

### 4. Interact + Verify

```bash
scripts/visual-qa click $(scripts/visual-qa locate "Chart")
scripts/visual-qa screenshot
scripts/visual-qa map    # Re-map after layout-changing actions
```

**CRITICAL**: After layout-changing actions, call `map` again on map-capable targets. Never batch multiple layout-changing actions with pre-computed coordinates.

### 5. Cleanup (always)

```bash
scripts/visual-qa stop
```

## Commands

| Command | Purpose |
|---------|---------|
| `start [--build] [--layout FILE]` | Launch app in virtual display |
| `stop` | Cleanup session, kill processes |
| `screenshot [--output PATH]` | Capture current state |
| `click X Y` | Left click |
| `right-click X Y` | Right click |
| `double-click X Y` | Double click |
| `drag X1 Y1 X2 Y2 [--steps N]` | Complete drag (press → move → release) |
| `mousedown X Y` | Press and hold |
| `move X Y` | Move cursor |
| `mouseup` | Release mouse |
| `key KEY` | Press key (xdotool names) |
| `map` | Live pane/split geometry JSON |
| `locate "text" [--all] [--near X Y] [--y-range MIN MAX]` | OCR element targeting |
| `cursor-pos` | Current cursor position |
| `resize-pane "A" "B" DELTA_PX` | Resize adjacent panes |
| `tab-transfer "source" "target"` | Drag tab between panes |
| `maximize ["tab"]` | Right-click → Maximize pane |
| `restore` | Right-click → Restore layout |
| `assert pane-count N` | Verify pane count |
| `assert window-size W H` | Verify window size |
| `assert tab-present "Title"` | Verify tab exists |
| `assert tab-in-pane "Tab" "Pane"` | Verify tab membership |
| `baseline capture` | Save golden reference |
| `baseline check` | Compare against golden baselines |
| `status` | Session health JSON |
| `doctor` | Dependency self-check |
| `setup` | Check/install prerequisites |
| `cleanup [N]` | Remove old screenshot runs |

## Coordinate Discovery

**NEVER visually estimate coordinates from screenshots.** Use `locate` for text targets and `map` for pane geometry when the target exposes a live map.

```bash
# Find element by OCR
scripts/visual-qa locate "Chart"                           # Returns "X Y"
scripts/visual-qa locate "Chart" --near 500 400            # Disambiguate
scripts/visual-qa locate "Chart" --y-range 25 70           # Tab bar only
scripts/visual-qa locate --all                              # Dump all text

# Click on found element
scripts/visual-qa click $(scripts/visual-qa locate "Chart")

# Drag between elements
scripts/visual-qa drag $(scripts/visual-qa locate "Watchlist") $(scripts/visual-qa locate "Positions")
```

**Tab labels vs body text**: Use `--y-range` to constrain search to tab bar areas when tab text matches body content.

## Mid-Drag Screenshots

```bash
scripts/visual-qa mousedown X Y     # Press on tab label
scripts/visual-qa move X2 Y2        # Move 10+ px to start drag
scripts/visual-qa move X3 Y3        # Move to target drop zone
scripts/visual-qa screenshot         # Capture overlay state
scripts/visual-qa mouseup            # Release
```

## Timing

- **Between actions**: `sleep 0.1` sufficient (tool adds 50ms internally)
- **After layout changes**: `sleep 0.3` for re-render before next locate
- **After context menu open**: `sleep 0.3` for menu to appear
- **Chain actions** in one bash call to avoid per-invocation overhead

## Baselines

```bash
scripts/visual-qa baseline capture    # Save golden reference
scripts/visual-qa baseline check      # Compare against baselines
```

Baseline capture/check require `VQA_SUPPORTS_BASELINE=true`. Disable baseline support for screenshot-only or viewer-only targets that do not have stable fixture coverage. Fixture discovery for `baseline capture/check` is configurable via `VQA_BASELINE_FIXTURE_GLOB`; the default is `*.ron` for backward compatibility.

Video recording is Linux-first today. `record start|stop` works under Xvfb on Linux; macOS currently reports recording as unsupported instead of pretending to capture video.

## Coordinate System

All coordinates are physical pixels. Screenshots, xdotool, and `locate` share the same coordinate space — no conversion needed at any scale factor.

## Prerequisites

Linux: xdotool, maim, ffmpeg, ImageMagick, Xvfb, vulkan-swrast, tesseract (+ eng data)
macOS: cliclick, ffmpeg, ImageMagick, tesseract (screencapture built-in)

Run `scripts/visual-qa setup` to check dependencies.
