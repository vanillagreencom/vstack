# CPU Cache Optimization

CPU cache optimization patterns for Rust hot paths — data layout, false sharing, prefetching, and measurement.

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
   - `layout-` for Data Layout (Section 1)
   - `sharing-` for False Sharing (Section 2)
   - `mem-` for Prefetching & Pages (Section 3)
   - `meas-` for Measurement (Section 4)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `CRITICAL` - Dominates hot-path latency or causes 10-100x throughput degradation
- `HIGH` - Measurable performance impact; causes memory-bound stalls or missed optimization opportunities
