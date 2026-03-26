# Latency Measurement

Patterns for accurate latency measurement, percentile tracking, and regression detection in sub-millisecond systems.

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
   - `mf-` for Measurement Fundamentals (Section 1)
   - `bench-` for Benchmarking (Section 2)
   - `mon-` for Runtime Monitoring (Section 3)
   - `prof-` for Profiling (Section 4)
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
- Rule files: `prefix-description.md` (e.g., `mf-percentiles-not-averages.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Core measurement principles; violations produce misleading data
- `HIGH` - Significant correctness or performance impact on benchmarks and monitoring
- `MEDIUM` - Good practices; violations waste optimization effort
