# Iced 0.14 Patterns

Patterns and rules for building Iced 0.14 applications with Elm-style state management.

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
   - `hr-` for Framework Constraints (Section 1; broad Iced behavior only)
   - `dev-` for Development Practices (Section 2)
   - `cache-` for Cache & Multi-Window (Section 3)
   - `elm-` for Elm Architecture (Section 4)
   - `interaction-` for Interaction (Section 5)
3. Fill in the frontmatter and content
4. Ensure you have clear incorrect/correct examples
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
- Rule files: `prefix-description.md` (e.g., `hr-widget-tree-consistency.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Broad Iced framework constraints; violations cause incorrect behavior or broken interaction
- `HIGH` - Significant correctness or performance impact
- `MEDIUM` - Good practices; violations cause coupling or maintenance burden
