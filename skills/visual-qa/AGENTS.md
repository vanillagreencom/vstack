# Visual QA

**Version 1.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when performing
> visual QA testing. Humans may also find it useful, but guidance here
> is optimized for automation and consistency by AI-assisted workflows.

---

## Abstract

Automated visual testing framework that launches applications in isolated virtual displays (Xvfb on Linux), provides mouse/keyboard interaction primitives, OCR-based element targeting, and golden baseline regression detection. Configurable for any GUI application via `visual-qa.conf`.

## Configuration

Create `visual-qa.conf` in project root to configure for your application:

```bash
# visual-qa.conf
VQA_APP_BINARY="target/debug/my-app"
VQA_BUILD_CMD="cargo build -p my-app --features dev"
VQA_WINDOW_TITLE="My Application"
VQA_SCREENSHOT_ENV_KEY="MY_APP_SCREENSHOT_DIR"
VQA_LAYOUT_ENV_KEY="MY_APP_LAYOUT_FIXTURE"
VQA_MAP_ENV_KEY="MY_APP_MAP_PATH"
VQA_DEFAULT_LAYOUT_ENV="MY_APP_DEFAULT_LAYOUT=1"
```

Without config, defaults assume a standard Cargo project layout.

## Quick Reference

| User request | Tool |
|-------------|------|
| "Take a screenshot" | `scripts/screenshot --no-build` → Read the PNG |
| "Screenshot with layout" | `scripts/screenshot --no-build --layout path/to/layout.ron` |
| "Show pane/split map" | `scripts/visual-qa map` |
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
scripts/visual-qa start --layout path/to/layout.ron # Specific layout
scripts/visual-qa start                             # Reuse existing binary
```

### 3. Map the Layout

```bash
scripts/visual-qa map              # Pane rectangles, split handles, tab membership
scripts/visual-qa locate --all     # All visible text with positions
```

### 4. Interact + Verify

```bash
scripts/visual-qa click $(scripts/visual-qa locate "Chart")
scripts/visual-qa screenshot
scripts/visual-qa map    # Re-map after layout-changing actions
```

**CRITICAL**: After layout-changing actions, call `map` again. Never batch multiple layout-changing actions with pre-computed coordinates.

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

**NEVER visually estimate coordinates from screenshots.** Use `locate` for text targets and `map` for pane geometry.

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

## Coordinate System

All coordinates are physical pixels. Screenshots, xdotool, and `locate` share the same coordinate space — no conversion needed at any scale factor.

## Prerequisites

Linux: xdotool, maim, ffmpeg, ImageMagick, Xvfb, vulkan-swrast, tesseract (+ eng data)
macOS: cliclick, ffmpeg, ImageMagick, tesseract (screencapture built-in)

Run `scripts/visual-qa setup` to check dependencies.
