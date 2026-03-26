---
title: OpenSSL Strategies for Cross Builds
impact: HIGH
tags: openssl, rustls, vendored, tls, static
---

## OpenSSL Strategies for Cross Builds

**Impact: HIGH (OpenSSL is the most common cross-compilation blocker)**

OpenSSL is a C library that requires target-specific headers and libraries. Four strategies exist, ordered by preference. For new projects, always prefer rustls. For existing projects with deep openssl dependency, use vendored.

**Incorrect (hoping OpenSSL just works in cross builds):**

```bash
# Cross-compiling a project that depends on openssl crate:
cargo build --target aarch64-unknown-linux-gnu --release
# error: failed to run custom build command for `openssl-sys`
# Could not find directory of OpenSSL installation
```

**Correct (choosing an appropriate OpenSSL strategy):**

```toml
# Strategy 1 (BEST): Switch to rustls — no C dependency at all
# Cargo.toml
[dependencies]
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
# Most HTTP/TLS crates support rustls as an alternative backend

# Strategy 2: Vendored OpenSSL — builds from source, slow but portable
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
# This compiles OpenSSL from source for the target — adds ~60s to build
# Works with any target, no pre-built libraries needed

# Strategy 3: Pre-build in Cross.toml Dockerfile
# Cross.toml
# [target.aarch64-unknown-linux-gnu]
# dockerfile = "cross/Dockerfile.aarch64"
# Where Dockerfile installs target-specific libssl-dev

# Strategy 4: OPENSSL_STATIC=1 with zigbuild
# OPENSSL_STATIC=1 OPENSSL_DIR=/path/to/openssl \
#   cargo zigbuild --target aarch64-unknown-linux-gnu --release
```
