---
title: VS Code CodeLLDB Configuration
impact: HIGH
tags: vscode, codelldb, ide, launch-json
---

## VS Code CodeLLDB Configuration

**Impact: HIGH (no IDE debugging without correct launch.json)**

The CodeLLDB extension provides Rust debugging in VS Code. Configure `launch.json` with cargo integration for automatic builds, source mapping for standard library stepping, and test binary support.

**Incorrect (missing cargo integration and source maps):**

```json
{
    "type": "lldb",
    "request": "launch",
    "program": "target/debug/myapp"
    // No automatic build — may debug stale binary
    // No source map — can't step into std library
}
```

**Correct (full CodeLLDB configuration):**

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug",
    "cargo": {
        "args": ["build"]
    },
    "sourceMap": {
        "/rustc/<hash>": "${env:HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"
    }
}

// Debug tests — build but don't run, then attach:
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug Tests",
    "cargo": {
        "args": ["test", "--no-run"]
    }
    // Set "program" to the test binary path from cargo test --no-run output
}
```
