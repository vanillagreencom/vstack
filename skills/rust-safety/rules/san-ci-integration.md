---
title: Sanitizer CI Integration
impact: HIGH
impactDescription: Sanitizers must run in CI to catch memory and concurrency bugs before merge
tags: sanitizer, ci, github-actions, testing, pipeline
---

## Sanitizer CI Integration

**Impact: HIGH (sanitizers must run in CI to catch memory and concurrency bugs before merge)**

Sanitizers should be integrated into CI as a matrix of checks. Miri and ASan are blocking (pre-merge). TSan is advisory (non-blocking) due to false positives in lock-free code.

**CI policy:**

| Sanitizer | Gate | Rationale |
|---|---|---|
| Miri | Blocking pre-merge | Catches UB with zero false positives |
| ASan | Blocking pre-merge | Catches memory errors with near-zero false positives |
| TSan | Non-blocking (advisory) | False positives on atomics; use loom for lock-free verification |

Run sanitizers only on crates with unsafe code to save CI time. Use a workspace filter or crate list.

**GitHub Actions workflow with matrix strategy:**

```yaml
name: Sanitizers
on: [pull_request]

jobs:
  sanitizers:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        sanitizer: [miri, asan, tsan]
        include:
          - sanitizer: miri
            command: cargo +nightly miri test
            env_flags: "-Zmiri-strict-provenance"
            blocking: true
          - sanitizer: asan
            command: >-
              cargo +nightly test -Zbuild-std
              --target x86_64-unknown-linux-gnu
            env_flags: "-Zsanitizer=address"
            blocking: true
          - sanitizer: tsan
            command: >-
              cargo +nightly test -Zbuild-std
              --target x86_64-unknown-linux-gnu
            env_flags: "-Zsanitizer=thread"
            blocking: false
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: miri, rust-src
      - name: Run ${{ matrix.sanitizer }}
        run: ${{ matrix.command }}
        env:
          RUSTFLAGS: ${{ matrix.env_flags }}
          MIRIFLAGS: ${{ matrix.sanitizer == 'miri' && matrix.env_flags || '' }}
        continue-on-error: ${{ !matrix.blocking }}
```

**Incorrect (no sanitizer CI — bugs reach production):**

```yaml
# Only running cargo test — no sanitizer coverage
- run: cargo test
```

**Correct (sanitizer matrix with blocking policy):**

```yaml
# Miri and ASan block merge, TSan is advisory
# See full workflow above
- name: Run ${{ matrix.sanitizer }}
  run: ${{ matrix.command }}
  continue-on-error: ${{ !matrix.blocking }}
```
