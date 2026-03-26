---
title: Verifier Error Resolution
impact: MEDIUM
impactDescription: verifier rejections block program loading entirely
tags: verifier, bpf, debugging, capabilities, vmlinux
---

## Verifier Error Resolution

**Impact: MEDIUM (verifier rejections block program loading entirely)**

Common verifier error to fix table:

| Error | Cause | Fix |
|-------|-------|-----|
| `invalid mem access 'scalar'` | Missing null check after map lookup | Add `if let Some(val) = map.get(&key)` guard |
| `back-edge detected` | Unbounded loop | Use bounded loop or `bpf_loop()` (kernel >=5.17) |
| `Type not found` | Stale vmlinux bindings | Regenerate with `aya-tool generate` |
| `Permission denied` | Missing capabilities | Need `CAP_BPF` or `CAP_SYS_ADMIN` |

Debug with: `RUST_LOG=debug cargo xtask run 2>&1 | grep verifier`

**Incorrect (unchecked map access):**

```rust
#[tracepoint]
pub fn my_prog(ctx: TracePointContext) -> u32 {
    let pid = ctx.pid();
    // Verifier rejects: map lookup can return null
    let val = unsafe { STATE.get(&pid) };
    let count = unsafe { *val }; // "invalid mem access 'scalar'"
    0
}
```

**Correct (null check after every map lookup):**

```rust
#[tracepoint]
pub fn my_prog(ctx: TracePointContext) -> u32 {
    let pid = ctx.pid();
    // Verifier accepts: null check before dereference
    if let Some(val) = unsafe { STATE.get(&pid) } {
        let count = *val;
        // use count
    }
    0
}
```
