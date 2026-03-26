---
title: LSP vs Grep
impact: MEDIUM
tags: navigation, lsp, grep, refactoring
---

## LSP vs Grep

**Impact: MEDIUM (wasted time reading entire files)**

- **Semantic queries → LSP** — findReferences, goToDefinition, incomingCalls for understanding code structure and impact
- **Text patterns → Grep** — string literals, log messages, config keys
- **Before refactoring** — Use LSP findReferences to understand full impact
- **Type uncertainty** — Use LSP hover instead of reading entire files
- **LSP returns 0 results?** — Don't trust it; fall back to Grep. Position mapping is unreliable.
- **Stale diagnostics after commits** — Verify with `cargo check` before acting on LSP warnings.
