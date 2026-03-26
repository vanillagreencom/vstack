# Architecture Patterns

Anti-patterns, scoring rubrics, error handling, and layered design patterns for architecture reviews.

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
   - `arch-` for Architectural Anti-Patterns (Section 1)
   - `perf-` for Performance Anti-Patterns (Section 2)
   - `lock-` for Lock-Free Anti-Patterns (Section 3)
   - `err-` for Error Handling & Data Integrity (Section 4)
   - `review-` for Review Process (Section 5)
   - `ui-` for UI Anti-Patterns (Section 6)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

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
- Rule files: `prefix-description.md` (e.g., `arch-god-object.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Structural or performance violations; must be rejected in reviews
- `HIGH` - Causes data corruption, silent failures, or concurrency bugs
- `MEDIUM` - Review process consistency or UI quality; track and address
