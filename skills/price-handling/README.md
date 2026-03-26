# Price Handling Patterns

f64 price handling rules for trading systems — epsilon comparison, tick-size rounding, feed normalization, and display formatting.

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
   - `core-` for Core Rules (Section 1)
   - `boundary-` for Boundaries (Section 2)
   - `type-` for Type Design (Section 3)
   - `feed-` for Feed Ingestion (Section 4)
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
- Rule files: `prefix-description.md` (e.g., `core-epsilon-comparison.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Non-negotiable constraints; violations cause incorrect prices, failed orders, or corrupted P&L
- `HIGH` - Significant correctness impact; wrong rounding boundary or formatting breaks downstream consumers
- `MEDIUM` - Good patterns; violations cause coupling, scattered logic, or accidental misuse
