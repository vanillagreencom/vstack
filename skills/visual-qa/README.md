# Visual QA

Automated visual testing in isolated virtual displays with OCR-based element targeting and baseline regression detection.

## Structure

- `scripts/visual-qa` - Main interactive testing tool (2250+ lines)
- `scripts/screenshot` - Standalone single-screenshot capture
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Configuration

Create `visual-qa.conf` in your project root:

```bash
VQA_APP_BINARY="target/debug/my-app"
VQA_BUILD_CMD="cargo build -p my-app --features dev"
VQA_WINDOW_TITLE="My Application"
VQA_SCREENSHOT_ENV_KEY="MY_APP_SCREENSHOT_DIR"
VQA_LAYOUT_ENV_KEY="MY_APP_LAYOUT_FIXTURE"
VQA_MAP_ENV_KEY="MY_APP_MAP_PATH"
VQA_DEFAULT_LAYOUT_ENV="MY_APP_DEFAULT_LAYOUT=1"
```

Without config, defaults assume a standard Cargo project layout.

## Prerequisites

Linux: xdotool, maim, ffmpeg, ImageMagick, Xvfb, vulkan-swrast, tesseract
macOS: cliclick, ffmpeg, ImageMagick, tesseract

Run `scripts/visual-qa setup` to check.
