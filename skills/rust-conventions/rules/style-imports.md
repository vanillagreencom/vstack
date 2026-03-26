---
title: Import Conventions
impact: MEDIUM
tags: imports, use, modules, style
---

## Import Conventions

**Impact: MEDIUM (inconsistent imports cause merge conflicts and confusion)**

- **Module-level imports** — `use` statements at top of file. Feature-gated: `#[cfg(feature = "X")] use crate::module::Thing;`. Function-level only when: single use AND would cause name clash.
- **Grouping order**: std → external crates → `crate::` → `super::`/`self::`. Blank line between groups.
- **Prefer**: modules, types, macros. Use qualified paths for functions: `module::function()`.
- **Avoid glob imports** except: preludes, `use super::*` in test modules.
- **Avoid enum variant imports** except: `Some`, `None`, `Ok`, `Err`.
