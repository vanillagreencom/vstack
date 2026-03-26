# Zero-Allocation Rust Patterns

Patterns for eliminating heap allocations in performance-critical Rust hot paths.

## Structure

- `rules/` - Individual rule files (one per rule)
  - `_sections.md` - Section metadata (titles, impacts, descriptions)
  - `_template.md` - Template for creating new rules
  - `prefix-description.md` - Individual rule files
- **`SKILL.md`** - Quick-reference index for skill-aware harnesses
- **`AGENTS.md`** - Full compiled document for all harnesses

## Creating a New Rule

1. Copy `rules/_template.md` to `rules/prefix-description.md`
2. Choose the appropriate area prefix:
   - `alloc-` for Allocation Elimination (Section 1)
   - `ds-` for Data Structures (Section 2)
   - `verify-` for Verification (Section 3)
   - `pit-` for Pitfalls (Section 4)
3. Fill in the frontmatter and content
4. Include incorrect/correct code examples where the rule would be ambiguous without them
5. Add the rule to the Quick Reference in `SKILL.md`
6. Add the expanded rule to the appropriate section in `AGENTS.md`

## Rule File Structure

```markdown
---
title: Rule Title Here
impact: MEDIUM
impactDescription: Optional description
tags: tag1, tag2
---

## Rule Title Here

**Impact: MEDIUM (optional impact description)**

Brief explanation of the rule and why it matters.

**Incorrect (description of what's wrong):**

\```rust
// Bad code example
\```

**Correct (description of what's right):**

\```rust
// Good code example
\```
```

## File Naming Convention

- Files starting with `_` are special (excluded from build)
- Rule files: `prefix-description.md` (e.g., `alloc-object-pools.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Core allocation elimination patterns; violations cause latency spikes in hot paths
- `HIGH` - Data structure choices and verification practices; wrong choices cause cache misses or undetected allocations
- `MEDIUM` - Common Rust idioms that silently allocate; awareness prevents accidental regressions
