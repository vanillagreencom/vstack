---
title: Giant File
impact: CRITICAL
impactDescription: blocks tooling and navigation
tags: architecture, file-size, modularity
---

## Giant File

**Impact: CRITICAL (blocks tooling and navigation)**

Files exceeding ~1,500 lines of implementation (or ~2,000 with tests) become difficult to navigate and can exceed tool context limits. They also tend to accumulate unrelated responsibilities.

**Indicator:** File line count growing beyond limits; multiple unrelated impl blocks.

**Fix:** Split by responsibility. Each module gets one clear responsibility and a minimal public API.
