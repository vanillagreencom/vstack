---
title: cargo-deny Policy Enforcement
impact: HIGH
impactDescription: vulnerable, banned, or unlicensed dependencies slip into production
tags: cargo-deny, license, advisory, security, ci
---

## cargo-deny Policy Enforcement

**Impact: HIGH (vulnerable, banned, or unlicensed dependencies slip into production)**

`cargo-deny` enforces license, advisory, ban, and source policies. Configure `deny.toml` with four sections: `[advisories]` for RUSTSEC database checks, `[licenses]` with an explicit allowlist, `[bans]` for duplicate or forbidden crates, and `[sources]` for registry restrictions. Run as a blocking CI check. Example: ban `openssl` globally, allow via a wrapper exception.

**Incorrect (no policy enforcement — anything goes):**

```yaml
# CI pipeline with no dependency auditing
steps:
  - run: cargo build
  - run: cargo test
  # No license check, no advisory scan, no ban enforcement
```

**Correct (deny.toml with full policy):**

```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "allow"
deny = [
    { name = "openssl-sys", wrappers = ["openssl"] },
    { name = "cmake" },
]

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

```yaml
# CI step — blocking check
- run: cargo install cargo-deny --locked
- run: cargo deny check
```
