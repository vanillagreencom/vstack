---
title: Cargo Config for Cross-Compilation
impact: HIGH
tags: cargo, config, toml, target, rustflags
---

## Cargo Config for Cross-Compilation

**Impact: HIGH (misconfigured cargo config causes silent build failures or wrong defaults)**

Use `.cargo/config.toml` in the project root for per-target settings. Keep platform-specific configuration in the project, not in user-global `~/.cargo/config.toml`, so all contributors and CI share the same setup.

**Incorrect (scattering config across CLI flags and global config):**

```toml
# ~/.cargo/config.toml (global — not shared with team or CI)
[build]
target = "aarch64-unknown-linux-gnu"

# Then relying on environment variables in scripts:
# RUSTFLAGS="-C target-feature=+crt-static" cargo build
# This is fragile and not reproducible
```

**Correct (project-local .cargo/config.toml with all cross settings):**

```toml
# .cargo/config.toml (committed to repo)

# Default build target (optional — set only if project is single-target)
# [build]
# target = "aarch64-unknown-linux-gnu"

# Per-target linker and runner configuration
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
runner = "qemu-aarch64 -L /usr/aarch64-linux-gnu"

[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

# Environment variables for cross-compilation
[env]
# OPENSSL_STATIC = "1"
# OPENSSL_DIR = "/usr/local/musl"
```
