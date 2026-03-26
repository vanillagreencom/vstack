---
title: Safety Comments
impact: MEDIUM
tags: unsafe, safety, comments
---

## Safety Comments

**Impact: MEDIUM (unsafe blocks without justification are review blockers)**

Document every `unsafe` block with a `// SAFETY:` comment explaining why invariants hold. Items `pub` only for benchmark/integration test access get `#[doc(hidden)]` to suppress `missing_docs`.
