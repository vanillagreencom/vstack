---
title: Cranelift Dev Backend
impact: HIGH
impactDescription: unnecessary codegen time in debug builds slows iteration
tags: cranelift, codegen, dev, nightly
---

## Cranelift Dev Backend

**Impact: HIGH (unnecessary codegen time in debug builds slows iteration)**

Cranelift codegen backend provides 20-40% faster development compilation at the cost of slower runtime performance. Use for `cargo check`-equivalent workflow where you need to run tests but don't care about execution speed. Configure via environment variable or cargo config. Nightly-only. Never use for release builds or benchmarks.

**Incorrect (using cranelift for release or benchmarks):**

```toml
# .cargo/config.toml
[profile.release]
codegen-backend = "cranelift"  # WRONG: cranelift generates slower code
# Benchmarks will be meaningless, release binary will underperform
```

**Correct (cranelift only for dev profile):**

```bash
# Install cranelift component (nightly required)
rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Option 1: Environment variable (per-session)
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly test

# Option 2: Cargo config (persistent)
```

```toml
# .cargo/config.toml — dev only
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"

# Release always uses default LLVM backend
[profile.release]
# codegen-backend not set — uses LLVM
```
