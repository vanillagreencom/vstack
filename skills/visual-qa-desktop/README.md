# Visual QA Desktop

Automated visual testing in isolated virtual displays with OCR-based element targeting and baseline regression detection.

## Structure

- `scripts/visual-qa-desktop` - Main interactive testing tool (2250+ lines)
- `scripts/screenshot` - Standalone single-screenshot capture
- `tools/visual-qa-desktop` - Compatibility wrapper for repos that already call `tools/...`
- `tools/screenshot` - Compatibility wrapper for repos that already call `tools/...`
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Setup

### 1. Create `visual-qa.conf` (required)

Copy [`visual-qa.conf.example`](./visual-qa.conf.example) to `visual-qa.conf` in your project root, then fill in your app-specific values. The tools will not run without it.

```bash
# visual-qa.conf — Required configuration
#
# These four variables MUST be set. Tools fail with a clear error if missing.

VQA_APP_BINARY="$ROOT_DIR/target/debug/my-app"
VQA_BUILD_CMD="cargo build -p my-app --features dev"
VQA_WINDOW_TITLE="My Application"
VQA_SCREENSHOT_ENV_KEY="MY_APP_SCREENSHOT_DIR"
```

### 2. Optional variables

```bash
# Friendly target label for logs/errors
VQA_TARGET_NAME="my-app"

# Layout fixture support (if your app loads layout files)
VQA_LAYOUT_ENV_KEY="MY_APP_LAYOUT_FIXTURE"

# Live pane/split map (if your app writes geometry JSON for the tool)
VQA_MAP_ENV_KEY="MY_APP_MAP_PATH"

# Default layout env (set when no --layout flag given)
VQA_DEFAULT_LAYOUT_ENV="MY_APP_DEFAULT_LAYOUT=1"

# Optional explicit capability overrides (useful for multi-target repos)
VQA_SUPPORTS_LAYOUT="true"
VQA_SUPPORTS_MAP="true"
VQA_SUPPORTS_BASELINE="true"
VQA_AUTO_WATCH="true"

# Optional baseline directories
VQA_BASELINE_FIXTURES_DIR="$ROOT_DIR/testdata/golden/layouts"
VQA_BASELINE_GOLDEN_DIR="$ROOT_DIR/testdata/golden/screenshots/linux"
VQA_BASELINE_FIXTURE_GLOB="*.fixture"
```

### 3. Optional workflow helpers in `.env.local`

The visual QA tool itself reads `visual-qa.conf`. If your project uses the portable `issue-lifecycle` or `orchestration` workflows, keep the higher-level routing helpers in `.env.local` (or export them directly):

```bash
VISUAL_QA_TARGET_CMD="./scripts/select-visual-target"
VISUAL_QA_FIXTURE="path/to/layout-fixture"
VISUAL_QA_SMOKE_CMD="./scripts/ui-smoke"
VISUAL_QA_BATTERY_CMD="./scripts/ui-battery"
VISUAL_QA_BASELINE_CMD="./scripts/capture-visual-baselines"
```

See the repo-root `.env.local.example` for a ready-to-copy template.

### Required variables

| Variable | Required | Purpose |
|----------|----------|---------|
| `VQA_APP_BINARY` | Yes | Path to the application binary |
| `VQA_BUILD_CMD` | Yes | Command to build the application |
| `VQA_WINDOW_TITLE` | Yes | Window title for xdotool detection |
| `VQA_SCREENSHOT_ENV_KEY` | Yes | Env var name the app reads for screenshot output dir |
| `VQA_LAYOUT_ENV_KEY` | No | Env var name for layout fixture path |
| `VQA_MAP_ENV_KEY` | No | Env var name for live map JSON path |
| `VQA_DEFAULT_LAYOUT_ENV` | No | Env assignment for default layout mode (e.g. `MY_APP_DEFAULT_LAYOUT=1`) |
| `VQA_TARGET_NAME` | No | Human-readable label for logs/errors |
| `VQA_SUPPORTS_LAYOUT` | No | Override automatic layout support detection |
| `VQA_SUPPORTS_MAP` | No | Override automatic map support detection |
| `VQA_SUPPORTS_BASELINE` | No | Override automatic baseline support detection |
| `VQA_AUTO_WATCH` | No | Auto-open the live viewer after `start` on Linux |
| `VQA_BASELINE_FIXTURES_DIR` | No | Override fixture directory for baseline capture/check |
| `VQA_BASELINE_GOLDEN_DIR` | No | Override golden screenshot directory for baseline capture/check |
| `VQA_BASELINE_FIXTURE_GLOB` | No | Glob used to discover baseline fixtures when `--scenarios all` |

Automatic defaults:
- Layout support defaults to `true` when `VQA_LAYOUT_ENV_KEY` is set, otherwise `false`
- Map support defaults to `true` when `VQA_MAP_ENV_KEY` is set, otherwise `false`
- Baseline support defaults to layout support

## Multi-Target Support

Projects with multiple UI binaries can use `VQA_TARGET` to switch targets. Branch on it in `visual-qa.conf`:

```bash
# visual-qa.conf — Multi-target example

# Default target
VQA_TARGET_NAME="my-app"
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
VQA_AUTO_WATCH="true"
VQA_BASELINE_FIXTURE_GLOB="*.fixture"

# Viewer target
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

Select target via env var or `--target` flag:

```bash
VQA_TARGET=viewer scripts/visual-qa-desktop start --build
scripts/visual-qa-desktop --target viewer start --build    # equivalent
scripts/screenshot --no-build                       # uses default target
VQA_TARGET=viewer scripts/screenshot --no-build     # uses viewer target
tools/visual-qa-desktop start --build                       # compatibility alias
```

If a target disables layout/map/baseline support, use screenshot/OCR/status flows only for that target. Pane helpers (`map`, `resize-pane`, `tab-transfer`, `maximize`, `restore`) require live-map support.

Baseline fixture discovery is configurable. The default glob is `*.ron` for backward compatibility, but projects can set `VQA_BASELINE_FIXTURE_GLOB` to any pattern that matches their fixture files.

Video recording is Linux-first today. `record start|stop` works under Xvfb on Linux; macOS currently reports recording as unsupported instead of pretending to capture video.

## Environment Variables

Core tool configuration lives in `visual-qa.conf`. Optional workflow helper variables can live in `.env.local` when your project needs target selection, smoke-test routing, broader visual batteries, or baseline routing.

## Prerequisites

Linux: xdotool, maim, ffmpeg, ImageMagick, Xvfb, vulkan-swrast, tesseract
macOS: cliclick, ffmpeg, ImageMagick, tesseract

Run `scripts/visual-qa-desktop setup` to check.
