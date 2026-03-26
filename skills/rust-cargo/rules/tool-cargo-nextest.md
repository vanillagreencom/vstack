---
title: cargo-nextest Parallel Testing
impact: HIGH
impactDescription: slow test suites blocking CI feedback loops
tags: nextest, testing, ci, parallel
---

## cargo-nextest Parallel Testing

**Impact: HIGH (slow test suites blocking CI feedback loops)**

`cargo nextest run` provides parallel test execution, 2-3x faster than `cargo test` for large test suites. Configure `.config/nextest.toml` for test-threads, retries for flaky tests, JUnit output for CI reporting, per-test timeouts, and test grouping. Nextest runs each test as a separate process, providing better isolation and failure output.

**Incorrect (default cargo test with no configuration):**

```yaml
# CI — serial test execution, no retries, no timeout
steps:
  - run: cargo test --workspace
  # Single-threaded by default for integration tests
  # No JUnit output for CI dashboards
  # Flaky test fails the whole run
```

**Correct (nextest with full configuration):**

```toml
# .config/nextest.toml
[store]
dir = "target/nextest"

[profile.default]
test-threads = "num-cpus"
status-level = "pass"
failure-output = "immediate-final"
slow-timeout = { period = "60s", terminate-after = 2 }

[profile.default.junit]
path = "target/nextest/default/junit.xml"

[profile.ci]
test-threads = 4
retries = 2
fail-fast = false

[profile.ci.junit]
path = "target/nextest/ci/junit.xml"
```

```yaml
# CI step
steps:
  - run: cargo install cargo-nextest --locked
  - run: cargo nextest run --workspace --profile ci
```
