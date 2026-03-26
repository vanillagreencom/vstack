# Lock-Free Safety Patterns

Verification patterns and correctness rules for lock-free data structures, atomic orderings, and epoch-based memory reclamation in Rust.

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
   - `sound-` for Soundness (Section 1)
   - `verify-` for Verification (Section 2)
   - `ord-` for Ordering (Section 3)
   - `epoch-` for Epoch Reclamation (Section 4)
   - `test-` for Testing (Section 5)
3. Fill in the frontmatter and content
4. Ensure you have clear incorrect/correct examples where the rule would be ambiguous without them
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
- Rule files: `prefix-description.md` (e.g., `sound-unsafecell-spsc.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Undefined behavior or false safety verification; violations cause silent data corruption or undetected concurrency bugs
- `HIGH` - Incorrect behavior on specific architectures or use-after-free; may not manifest on developer hardware (x86)
- `MEDIUM` - Wasted resources or insufficient test coverage; correctness not immediately at risk
