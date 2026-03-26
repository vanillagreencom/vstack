---
title: Fast Linker Configuration
impact: HIGH
impactDescription: linking dominates build time for large binaries
tags: linker, mold, lld, performance, config
---

## Fast Linker Configuration

**Impact: HIGH (linking dominates build time for large binaries)**

Linker speed comparison: mold (5-10x over GNU ld) > lld (2x) > gold (1.5x) > GNU ld. Configure in `.cargo/config.toml`. Biggest win for large binaries where linking can take 10-30 seconds with GNU ld. mold is the recommended default for Linux development.

**Incorrect (default GNU ld linker):**

```toml
# .cargo/config.toml — no linker configured
# Uses system default (GNU ld) — slowest option
# 15-30 second link times for large binaries
```

**Correct (mold linker on Linux):**

```toml
# .cargo/config.toml

# Linux — mold (fastest)
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

# macOS — use lld via Xcode (mold not available)
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Windows — lld (bundled with Rust)
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

```bash
# Install mold on Linux
# Ubuntu/Debian
sudo apt install mold
# Arch
sudo pacman -S mold
# Fedora
sudo dnf install mold
```
