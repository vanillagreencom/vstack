# Linux Profiling for Low-Latency Systems

Profiling patterns for sub-millisecond latency systems on Linux.

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
   - `hw-` for Hardware Event Accuracy (Section 1)
   - `cpu-` for CPU Profiling (Section 2)
   - `cache-` for Cache & TLB (Section 3)
   - `numa-` for NUMA Locality (Section 4)
   - `jitter-` for System Jitter (Section 5)
   - `mem-` for Memory Profiling (Section 6)
3. Fill in the frontmatter and content
4. Include practical commands that can be run immediately
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

\```bash
# Bad example here
\```

**Correct (description of what's right):**

\```bash
# Good example here
\```
```

## File Naming Convention

- Files starting with `_` are special (excluded from build)
- Rule files: `prefix-description.md` (e.g., `hw-amd-generic-events.md`)
- Section is inferred from filename prefix
- Rules are sorted alphabetically by title within each section

## Impact Levels

- `CRITICAL` - Hardware/platform constraints; violations produce silently wrong profiling data
- `HIGH` - Significant impact on profiling accuracy or system performance
- `MEDIUM` - Good practices; violations cause missed optimization opportunities or slower diagnosis
