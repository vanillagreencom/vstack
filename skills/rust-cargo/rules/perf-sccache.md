---
title: Shared Compilation Cache
impact: HIGH
impactDescription: redundant recompilation across builds and CI runs
tags: sccache, cache, ci, distributed
---

## Shared Compilation Cache

**Impact: HIGH (redundant recompilation across builds and CI runs)**

Shared compilation cache via `RUSTC_WRAPPER=sccache`. Works locally or distributed (S3, GCS, Redis). For GitHub Actions CI, use `Swatinem/rust-cache@v2` to cache `target/` between runs. Disable caching for final release builds where cache may interfere with LTO.

**Incorrect (no caching — full rebuild every time):**

```yaml
# GitHub Actions — no caching
steps:
  - uses: actions/checkout@v4
  - run: cargo build --workspace
  # Every CI run recompiles all dependencies from scratch
  # 5-15 minutes wasted per run
```

**Correct (sccache locally + CI caching):**

```bash
# Local development
cargo install sccache --locked
export RUSTC_WRAPPER=sccache

# Verify it's working
sccache --show-stats

# Optional: distributed cache
export SCCACHE_BUCKET=my-sccache-bucket
export SCCACHE_REGION=us-east-1
```

```yaml
# GitHub Actions — with rust-cache
steps:
  - uses: actions/checkout@v4

  - uses: dtolnay/rust-toolchain@stable

  - uses: Swatinem/rust-cache@v2
    with:
      cache-on-failure: true
      shared-key: "ci-${{ hashFiles('**/Cargo.lock') }}"

  - name: Install sccache
    uses: mozilla-actions/sccache-action@v0.0.4

  - run: cargo build --workspace
    env:
      RUSTC_WRAPPER: sccache

  # Disable cache for final release (LTO needs clean build)
  - run: cargo build --release
    if: github.ref == 'refs/heads/main'
    env:
      RUSTC_WRAPPER: ""
      CARGO_INCREMENTAL: "0"
```
