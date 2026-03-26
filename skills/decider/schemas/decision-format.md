# Decision Document Format Schema

Defines the canonical structure and constraints for decision documents and the decision index.

## Decision Document (`DXXX-descriptor.md`)

### Required Elements

| Element | Format | Constraint |
|---------|--------|------------|
| Title | `# DXXX: Title` | H1, uppercase D, zero-padded 3-digit number |
| Index back-link | `[← Decision Index](INDEX.md)` | Immediately after title |
| Date | `**Date**: YYYY-MM-DD` | ISO 8601 date |
| Status | `**Status**: [VALUE]` | See Status Values below |
| Research | `**Research**: [REF]` or `**Research**: —` | Link or dash |
| Decision statement | Bold-label or H2 section | Must explicitly state what was chosen |
| Rationale | Bullets, table, or section | Must explain why |
| Revisit When | Bullets or inline | Conditions for re-evaluation |

### Optional Elements

| Element | When to Include |
|---------|-----------------|
| `**Applies to**:` | Decision scoped to specific context |
| `**Refines**:` | Decision refines/extends prior decisions |
| `**API Contract**:` | Decision defines or changes an API contract |
| `## Summary` | Standard/comprehensive entries |
| `## Context` | When problem context isn't obvious |
| `## Pattern` | When decision introduces code patterns |
| `## Verification` | When decision is testable |
| `## Alternatives Considered` | When alternatives were evaluated |
| `## Impact` | When decision affects other decisions |
| `## Appendices` | For detailed schemas, code examples |

### File Naming

Pattern: `DXXX-kebab-case-descriptor.md`

- `DXXX`: Zero-padded sequential ID (D001, D010, D034)
- `descriptor`: Kebab-case summary of the decision topic (2-5 words)
- Examples: `D001-session-caching.md`, `D010-test-organization.md`

## INDEX.md Structure

### Table Schema

```markdown
| Date | ID | Research | Decision | Rationale | Revisit When | Status | Link |
|------|-----|----------|----------|-----------|--------------|--------|------|
```

### Required Sections

1. **Title**: `# Architectural Decision Log`
2. **Description**: 1-2 sentence purpose statement
3. **Decision Log**: Table with all entries
4. **Format Reference**: Guidelines for what to log / not log

### Format Reference Content

```markdown
## Format Reference

### What to Log
- Technology selections with alternatives considered
- Performance trade-offs (chose X over Y for reason Z)
- Significant path choices where conditions might change
- Research-informed decisions (reference research ID in rationale)

### What NOT to Log
- Variable names, small refactors, bug fixes
- Obvious choices with no realistic alternatives
- Standard pattern applications

### Status Values
- **Active**: Current decision in effect
- **Superseded by DXXX**: Replaced by newer decision
- **Revisited**: Re-evaluated, with outcome noted

### Code Comments
Use `// REVISIT(DXXX):` in code to mark implementation points tied to decisions.
```

## Status Values

| Value | Meaning | When |
|-------|---------|------|
| `Active` | Decision is in effect | Default for new decisions |
| `Superseded by DXXX` | Fully replaced by another decision | New decision covers entire scope |
| `Revisited` | Re-evaluated with outcome noted | Conditions changed, decision re-assessed |
| `Active ([COMPONENTS] → DXXX)` | Partially superseded | New decision replaces specific components only |

## Cross-Reference Conventions

| Reference Type | Format |
|----------------|--------|
| Decision-to-decision | `[DXXX](DXXX-descriptor.md)` |
| Decision-to-research | `[RESEARCH-ID](../research/RESEARCH-ID/findings.md)` |
| Decision-to-code | `` `path/to/file.rs` `` or `[file](../../path/to/file.rs)` |
| Code-to-decision | `// REVISIT(DXXX): [reason]` |
| Issue-to-decision | `**Decision [DXXX]**: [path/to/DXXX-descriptor.md]` |

## Content Guidelines

### What Makes a Good Decision Entry

- **Declares** what was chosen, not just what was considered
- **Explains** why with concrete rationale (benchmarks, industry precedent, constraints)
- **Anticipates** change with specific revisit conditions
- **References** research for detailed analysis — decision keeps tight summary
- **Links** to related decisions, code, and issues

### Sizing

| Scope | Template | Typical Lines |
|-------|----------|---------------|
| Single technology choice | Minimal | 15-30 |
| Pattern with alternatives | Standard | 80-200 |
| Architecture-level | Comprehensive | 200-600 |
