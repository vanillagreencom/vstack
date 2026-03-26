---
title: Unit Test Path Pattern
impact: HIGH
tags: testing, structure, path, pub_crate
---

## Unit Test Path Pattern

**Impact: HIGH (tests can't access pub(crate) items without this)**

Sibling file pattern for `pub(crate)` access:
- `module.rs` + `module_tests.rs` in same directory
- Source declares: `#[cfg(test)] #[path = "module_tests.rs"] mod tests;`
- Test imports: `use super::*;`

When a test file exceeds 1,000 lines, split into focused modules with descriptive names. Split modules may use explicit `use super::Type` or `use crate::` imports.
