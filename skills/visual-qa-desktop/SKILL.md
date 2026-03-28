---
name: visual-qa-desktop
description: Visual QA framework — screenshot capture, interactive testing in virtual displays, OCR-based element targeting, and baseline regression detection. Use when asked to take a screenshot, test UI interactions, verify drag/drop behavior, capture visual baselines, or run regression checks.
license: MIT
user-invocable: true
metadata:
  author: vanillagreen
  version: "1.1.0"
---

# Visual QA Desktop

Automated visual testing in isolated virtual displays with screenshot capture, OCR-based element targeting, and baseline regression detection.

## When to Apply

Reference these guidelines when:
- Taking screenshots of a running application
- Testing UI interactions (click, drag, resize)
- Verifying visual behavior after code changes
- Capturing or comparing golden baselines
- Running interactive test sessions

## Entry Points

```bash
scripts/screenshot [--target NAME] [--no-build] [--timeout N] [--layout FILE]   # Single screenshot
scripts/visual-qa-desktop [--target NAME] <command> [args...]                            # Full interactive session
# Compatibility aliases for repos that already call tools/
tools/screenshot ...
tools/visual-qa-desktop ...
```

## Quick Reference — Pick the Right Tool

| User request | What to do |
|-------------|-----------|
| "Take a screenshot" | `scripts/screenshot --no-build` → Read the PNG |
| "Show pane/split map" | `scripts/visual-qa-desktop map` if the selected target exposes a live map |
| "Test if drag/resize works" | Start interactive session (see below) |
| "Did the UI change?" | `scripts/visual-qa-desktop baseline check` |
| "Capture baselines" | `scripts/visual-qa-desktop baseline capture` |
| "Full visual QA" | Interactive session with full checklist |

## Interactive Session Lifecycle

1. **Start**: `scripts/visual-qa-desktop start [--build] [--layout FILE]`
2. **Inspect capabilities**:
   - If the target supports live map geometry, use `scripts/visual-qa-desktop map` + `scripts/visual-qa-desktop locate --all`
   - If the target is screenshot/OCR-only, skip `map` and use `locate`, `status`, `click`, `screenshot`
3. **Interact + verify**: click/drag/screenshot, re-map after layout changes only when map support exists
4. **Cleanup**: `scripts/visual-qa-desktop stop`

## Commands

| Command | Purpose |
|---------|---------|
| `start [--build]` | Launch app in virtual display |
| `stop` | Cleanup session |
| `screenshot` | Capture current state |
| `click X Y` / `right-click` / `double-click` | Mouse actions |
| `drag X1 Y1 X2 Y2` | Complete drag (press → move → release) |
| `mousedown` / `move` / `mouseup` | Granular mouse control |
| `key KEY` | Keyboard input |
| `map` | Live pane/split geometry JSON |
| `locate "text" [--all] [--near X Y]` | OCR-based element targeting |
| `resize-pane "A" "B" DELTA` | Resize adjacent panes |
| `tab-transfer "src" "tgt"` | Drag tab between panes |
| `maximize` / `restore` | Context menu pane actions |
| `assert pane-count N` | Verify pane count |
| `assert tab-present "Title"` | Verify tab exists |
| `baseline capture` / `check` | Golden baseline management |
| `status` / `doctor` / `setup` | Session health and dependencies |

## Configuration

Requires `visual-qa.conf` in project root. Tools fail with a clear error if required variables are missing.

| Variable | Required | Purpose |
|----------|----------|---------|
| `VQA_APP_BINARY` | Yes | Path to app binary |
| `VQA_BUILD_CMD` | Yes | Build command |
| `VQA_WINDOW_TITLE` | Yes | Window title for xdotool detection |
| `VQA_SCREENSHOT_ENV_KEY` | Yes | Env var name the app reads for screenshot output dir |
| `VQA_LAYOUT_ENV_KEY` | No | Env var name for layout fixture path |
| `VQA_MAP_ENV_KEY` | No | Env var name for live map JSON path |
| `VQA_DEFAULT_LAYOUT_ENV` | No | Default layout env assignment |
| `VQA_TARGET_NAME` | No | Human-readable target label for logs/errors |
| `VQA_SUPPORTS_LAYOUT` | No | Force layout capability on/off per target |
| `VQA_SUPPORTS_MAP` | No | Force live-map capability on/off per target |
| `VQA_SUPPORTS_BASELINE` | No | Force baseline capability on/off per target |
| `VQA_AUTO_WATCH` | No | Auto-open the live viewer after `start` on Linux |
| `VQA_BASELINE_FIXTURES_DIR` | No | Directory of baseline layout fixtures |
| `VQA_BASELINE_GOLDEN_DIR` | No | Directory of captured baseline PNGs |
| `VQA_BASELINE_FIXTURE_GLOB` | No | Glob used to discover baseline fixtures |

**Multi-target**: Use `--target NAME` or `VQA_TARGET=NAME` to select a named target. Branch on `VQA_TARGET` in your `visual-qa.conf`.

**Capability defaults**:
- Layout support defaults to `true` when `VQA_LAYOUT_ENV_KEY` is set, otherwise `false`
- Map support defaults to `true` when `VQA_MAP_ENV_KEY` is set, otherwise `false`
- Baseline support defaults to layout support, but can be disabled for screenshot-only targets

## Rules

### Coordinate Discovery (CRITICAL)

- **Never visually estimate coordinates from screenshots** — always use `locate` or `map`
- **Re-map after layout changes** — after any click/drag/key, call `map` again before using coordinates, but only on targets that expose a live map
- **Use `--y-range` for tab labels** — tab labels share text with body content; constrain OCR search

### Capability Routing (HIGH)

- **Do not assume every target exposes every command** — screenshot/OCR targets may not support `map`, pane helpers, layout fixtures, or baselines
- **Branch by target capability, not by hope** — use `VQA_SUPPORTS_LAYOUT`, `VQA_SUPPORTS_MAP`, and `VQA_SUPPORTS_BASELINE`
- **Prefer project routing helpers when present** — if the project provides a visual-QA target selection command, follow it before running tests

### Session Discipline (HIGH)

- **Always stop on exit** — `scripts/visual-qa-desktop stop` even on failure
- **Targeted scope by default** — test only what's relevant to the change, not the full checklist
- **Confirm with user on Linux** — "I'll launch in an isolated virtual display"

## Prerequisites

Linux: xdotool, maim, ffmpeg, ImageMagick, Xvfb, vulkan-swrast, tesseract (+ eng data)
macOS: cliclick, ffmpeg, ImageMagick, tesseract (screencapture built-in)

Run `scripts/visual-qa-desktop setup` to check.

## Full Compiled Document

For the complete guide with interaction reference, timing rules, and mid-drag screenshots: `AGENTS.md`
