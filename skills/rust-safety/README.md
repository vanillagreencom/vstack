# Safety Audit Patterns

Checklists and rules for auditing unsafe Rust code.

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
   - `safety-` for SAFETY Comments (Section 1)
   - `unsafe-` for Unsafe Block Audit (Section 2)
   - `mem-` for Memory Safety (Section 3)
   - `ptr-` for Raw Pointer Audit (Section 4)
   - `lockfree-` for Lock-Free Structures (Section 5)
   - `epoch-` for Crossbeam Epoch (Section 6)
   - `sec-` for Security (Section 7)
   - `sev-` for Severity Classification (Section 8)
   - `san-` for Sanitizers (Section 9)
   - `fuzz-` for Fuzzing (Section 10)
   - `supply-` for Supply Chain (Section 11)
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
- Rule files: `prefix-description.md` (e.g., `safety-comment-standard.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Undefined behavior, memory corruption, or exploitable vulnerability; violations are unsound
- `HIGH` - Significant correctness or security impact; merge-blocking
- `MEDIUM` - Process and classification rules; violations cause inconsistency
