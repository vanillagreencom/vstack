---
title: Separate Debug Symbols
impact: MEDIUM
tags: symbols, objcopy, strip, rustfilt, addr2line
---

## Separate Debug Symbols

**Impact: MEDIUM (large binaries in production or no symbols for post-mortem)**

Extract debug symbols into a separate file for crash analysis while shipping stripped binaries. Use `rustfilt` for symbol demangling when analyzing addresses.

**Incorrect (shipping full debug binary or stripping everything):**

```bash
# Option A: Ship 500MB debug binary to production
cp target/release/app /deploy/app

# Option B: Strip everything — no post-mortem possible
strip target/release/app
# Core dump is now useless — no symbols to resolve
```

**Correct (extract symbols, strip binary, link for analysis):**

```bash
# 1. Extract debug symbols to separate file
objcopy --only-keep-debug target/release/app app.debug

# 2. Strip the binary for deployment
strip --strip-debug target/release/app

# 3. Add debug link so GDB auto-finds symbols
objcopy --add-gnu-debuglink=app.debug target/release/app

# 4. Ship stripped binary, archive .debug file
# Deploy: target/release/app (small)
# Archive: app.debug (for post-mortem)

# 5. Demangle addresses from crash logs
addr2line -e app.debug 0x12345 | rustfilt
# Output: myapp::server::handle_request at src/server.rs:42
```
