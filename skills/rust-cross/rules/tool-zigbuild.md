---
title: Cargo Zigbuild
impact: HIGH
tags: zigbuild, zig, glibc, cross-compiler
---

## Cargo Zigbuild

**Impact: HIGH (glibc version mismatch causes runtime crashes on target systems)**

`cargo-zigbuild` uses Zig as a drop-in C/C++ cross-compiler. No Docker required. Key advantage: precise glibc version targeting with `--target aarch64-unknown-linux-gnu.2.17`. Better than `cross` for CI without Docker, precise glibc control, and mixed C/Rust projects.

**Incorrect (building without glibc version pinning):**

```bash
# Default build links against host glibc (e.g., 2.35)
cargo build --target aarch64-unknown-linux-gnu --release
# Binary fails on RHEL 7 / CentOS 7 (glibc 2.17):
# ./myapp: /lib64/libc.so.6: version `GLIBC_2.28' not found
```

**Correct (using zigbuild with pinned glibc version):**

```bash
# Install zigbuild and Zig
cargo install cargo-zigbuild
pip install ziglang
# Or: snap install zig --classic

# Build with precise glibc version targeting
cargo zigbuild --target aarch64-unknown-linux-gnu.2.17 --release
# Binary works on any Linux with glibc >= 2.17

# Also works for x86_64
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17 --release

# No Docker, no container overhead, fast builds
```
