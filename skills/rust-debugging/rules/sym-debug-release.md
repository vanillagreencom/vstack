---
title: Debug Info in Release Builds
impact: MEDIUM
tags: debug-info, release, profile, split-debuginfo
---

## Debug Info in Release Builds

**Impact: MEDIUM (release-only bugs undiagnosable without symbols)**

Always build release with at least `debug = 1` (line tables) for profiling. Use `debug = 2` for full debugging of release-only issues. Debug info does not affect runtime performance — only binary size and build time.

**Incorrect (release profile with no debug info):**

```toml
# Cargo.toml
[profile.release]
# debug = false (default) — no symbols at all
# Profilers show only hex addresses
# Release-only bugs can't be diagnosed
```

**Correct (release profile with appropriate debug info):**

```toml
# Cargo.toml

# For profiling (recommended default):
[profile.release]
debug = 1                      # Line tables only — small overhead
split-debuginfo = "packed"     # Linux default — single .dwp file

# For debugging release-only issues:
[profile.release]
debug = 2                      # Full debug info — variable inspection
split-debuginfo = "unpacked"   # macOS — faster incremental linking

# debug = 1: ~10-20% larger binary, line-level profiling
# debug = 2: ~2-5x larger binary, full variable inspection
# Runtime performance: identical — debug info is not loaded at runtime
```
