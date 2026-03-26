# Perf Threading

Topology-aware thread pinning, core isolation, SPSC patterns, and page fault prevention for Rust low-latency systems.

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
   - `topo-` for CPU Topology (Section 1)
   - `isolate-` for Core Isolation (Section 2)
   - `spsc-` for SPSC Patterns (Section 3)
   - `fault-` for Page Fault Prevention (Section 4)
3. Fill in the frontmatter and content
4. Add the rule to the Quick Reference in `SKILL.md`
5. Add the expanded rule to the appropriate section in `AGENTS.md`

## Impact Levels

- `CRITICAL` - Wrong thread placement adds 2-3x latency; affects every message
- `HIGH` - Causes microsecond-scale jitter, incorrect measurements, or page fault spikes
