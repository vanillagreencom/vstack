---
title: Module Extraction Pattern
impact: MEDIUM
impactDescription: Bloated root module, hard-to-test code
tags: elm, modules, extraction, organization
---

## Module Extraction Pattern

**Impact: MEDIUM (bloated root module, hard-to-test code)**

Extract when: feature-gated and self-contained, OR cohesive responsibility group, OR >30 lines on a well-defined State subset. Module pattern: `impl State` block with doc comment, `crate::` imports, `pub(crate)` methods. Feature gates move with the function — if all functions share a gate, apply it to the `mod` declaration.
