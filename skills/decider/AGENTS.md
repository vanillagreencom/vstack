# Decider

Architectural decision document management — templates, creation workflows, search CLI, and supersession tracking. Provides the single source of truth for decision entry format and lifecycle.

## When to Apply

Reference these guidelines when:
- Creating a new decision entry after research completion
- Recording a significant path choice during implementation
- Searching for existing decisions governing an area of code
- Checking if a proposed change contradicts an active decision
- Superseding or partially superseding an existing decision
- Including decision context in PR bodies or delegation prompts
- Validating decision references in issue descriptions

## Skill Dependencies

This skill is self-contained. Other skills depend on it:

| Dependent Skill | Purpose |
|-----------------|---------|
| Orchestration | Decision creation in research-complete, search in review/submit workflows |
| Issue Lifecycle | Decision search in dev-implement/dev-fix/qa-review, creation in dev-implement |
| Project Management | Decision search in audit/roadmap workflows |

Project-level configuration:

| Variable | Purpose | Default |
|----------|---------|---------|
| `$DECISIONS_CMD` | Path to decisions CLI | `scripts/decisions` |
| `$DECISIONS_DIR` | Path to decision documents directory | — (required) |

---

## CLI Commands

| Command | Purpose | Output |
|---------|---------|--------|
| `$DECISIONS_CMD search --issue [ID]` | Find decisions linked to an issue | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD search "[KEYWORDS]"` | Ranked keyword search (AND, scored) | JSON `[{id, decision, path, score}]` |
| `$DECISIONS_CMD search "a\|b"` | Regex OR search | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD list` | List all active decisions | JSON `[{id, decision, path}]` |
| `$DECISIONS_CMD next-id` | Get next available DXXX | Single `DXXX` line |
| `$DECISIONS_CMD get [DXXX]` | Get decision details | JSON `{id, decision, status, date, path}` |

Options: `--limit N` (default: 5) for search results.

### Search Scoring

Multi-word queries use AND logic with relevance scoring:
- Decision/title match: 3 points per term
- ID match: 3 points per term
- Rationale match: 1 point per term

Queries containing `|`, `()`, or `\` switch to raw regex mode (no scoring).

---

## Decision Document Format

### Required Elements

| Element | Format | Constraint |
|---------|--------|------------|
| Title | `# DXXX: Title` | H1, uppercase D, zero-padded 3-digit number |
| Index back-link | `[← Decision Index](INDEX.md)` | Immediately after title |
| Date | `**Date**: YYYY-MM-DD` | ISO 8601 date |
| Status | `**Status**: [VALUE]` | See Status Values |
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

### File Naming

Pattern: `DXXX-kebab-case-descriptor.md`

Examples: `D001-session-caching.md`, `D010-test-organization.md`

### Status Values

| Value | Meaning |
|-------|---------|
| `Active` | Decision is in effect |
| `Superseded by DXXX` | Fully replaced by another decision |
| `Revisited` | Re-evaluated with outcome noted |
| `Active ([COMPONENTS] → DXXX)` | Partially superseded |

### Cross-Reference Conventions

| Reference Type | Format |
|----------------|--------|
| Decision-to-decision | `[DXXX](DXXX-descriptor.md)` |
| Decision-to-research | `[RESEARCH-ID](../research/RESEARCH-ID/findings.md)` |
| Code-to-decision | `// REVISIT(DXXX): [reason]` |
| Issue-to-decision | `**Decision [DXXX]**: [path/to/DXXX-descriptor.md]` |

### Formatting Rules

1. **Title**: `# [DECISION_ID]: [TITLE]` — H1, uppercase D + zero-padded number
2. **Back-link**: Always `[← Decision Index](INDEX.md)` after title
3. **Metadata**: `**Date**:`, `**Status**:`, `**Research**:` — bold key, colon, space, value
4. **H2 for sections** (`##`), H3 for subsections (`###`), H4 for sub-subsections (`####`)
5. **Tables**: Standard markdown pipe-and-dash format
6. **Code blocks**: Always specify language (e.g., `rust`, `sql`, `json`, `bash`)
7. **Cross-references**: Always markdown links `[DXXX](DXXX-descriptor.md)`

---

## INDEX.md Row Format

```markdown
| [DATE] | [DECISION_ID] | [RESEARCH_REF] | [DECISION_SUMMARY] | [RATIONALE_SUMMARY] | [REVISIT_WHEN] | [STATUS] | [Full]([DECISION_ID]-[DESCRIPTOR].md) |
```

Each field: 5-15 word summary. New rows appended chronologically before the `---` separator.

---

## Decision Entry Templates

### Minimal Entry (15-30 lines)

For focused, single-topic decisions with clear rationale.

```markdown
# [DECISION_ID]: [TITLE]

[← Decision Index](INDEX.md)

**Date**: [YYYY-MM-DD]
**Status**: Active
**Research**: [RESEARCH_REF or —]

**Context**: [1-2 sentences: what problem/need exists]

**Decision**: [1-2 sentences: what was chosen]

**Rationale**:
- [Key reason 1]
- [Key reason 2]

**Revisit When**: [Conditions that would trigger re-evaluation]

**Verification**: [How to verify — commands, benchmarks, tests]

**References**: [Related decision IDs, research IDs]
```

### Standard Entry (80-200 lines)

For decisions with alternatives considered, code patterns, and structured rationale.

```markdown
# [DECISION_ID]: [TITLE]

[← Decision Index](INDEX.md)

**Date**: [YYYY-MM-DD]
**Status**: Active
**Research**: [RESEARCH_REF or —]

## Summary
[1-2 paragraph executive summary]

## Context
[Detailed explanation of the problem]

## Decision
[Explicit statement of what was chosen]

## Pattern
[Code examples, directory structures — if applicable]

## Rationale
| Criterion | Chosen Approach | Alternative |
|-----------|-----------------|-------------|
| [criterion] | [advantage] | [disadvantage] |

## Decision Criteria
### Use [chosen approach] when:
- [condition 1]

### Use [alternative] when:
- [condition 1]

## Verification
[Commands, tests, benchmarks]

## Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| [alt 1] | [reason] |

## Revisit When
- [Condition 1]
```

### Comprehensive Entry (200-600 lines)

For large architectural decisions spanning multiple concerns.

```markdown
# [DECISION_ID]: [TITLE]

[← Decision Index](INDEX.md)

**Date**: [YYYY-MM-DD]
**Status**: Active
**Research**: [RESEARCH_REF or —]
**Applies to**: [Context string]
**Refines**: [DXXX, DYYY]

## Summary
[Comprehensive overview with cross-references]

## Requirements
| Requirement | Scope | Notes | Owner |
|-------------|-------|-------|-------|

## Rationale
### [Section 1]
### [Section 2]

## Design
### [Component 1]
### [Component 2]

## Impact
| Decision | Change | Rationale |
|----------|--------|-----------|

## Resolved Decisions
[Settled questions]

## Revisit When
- [Condition 1]
```

Choose the smallest template that covers the decision's scope.

---

## Create Decision Workflow

> **Dependencies**: `$DECISIONS_CMD` (optional for next-id), project decision documents directory

### 1. Assign Decision ID

```bash
$DECISIONS_CMD next-id
```

If `$DECISIONS_CMD` not configured: Read INDEX.md, find last DXXX row, increment by 1.

Generate a 2-5 word kebab-case descriptor from the decision summary.

### 2. Select Template

| Scope | Template |
|-------|----------|
| Single technology choice, clear winner | Minimal |
| Multiple alternatives, patterns to document | Standard |
| Architecture-level, multi-concern | Comprehensive |

### 3. Write Decision File

Create `[project decision documents]/[DECISION_ID]-[DESCRIPTOR].md` per selected template.

Required fields:
- `**Date**:` — today's date
- `**Status**:` — `Active`
- `**Research**:` — link or `—`
- Decision statement, rationale, revisit conditions

Keep tight — reference research for details.

### 4. Add INDEX.md Row

Add row at end of table per INDEX row format.

### 5. Update Partially Superseded Decisions

If new decision partially affects existing decisions:
- Read referenced decision file
- Update status to `Active ([COMPONENTS] → [DECISION_ID])` in both file and INDEX.md

### 6. Add Code References

For implementation points: `// REVISIT([DECISION_ID]): [reason]`

### 7. Return

```
Decision: [DECISION_ID] - [TITLE]
Path: [project decision documents]/[DECISION_ID]-[DESCRIPTOR].md
```

---

## Update Decision Workflow

### Update Types

| Type | Status Change |
|------|---------------|
| `supersede` | `Superseded by [NEW_DECISION_ID]` |
| `partial_supersede` | `Active ([COMPONENTS] → [NEW_DECISION_ID])` |
| `revisit` | `Revisited` + append outcome |

### Steps

1. Read decision file
2. Update status line in decision file
3. Update status column in INDEX.md
4. For supersession: update `// REVISIT(DXXX)` comments in code to reference new decision

---

## Search Decisions Workflow

### Before implementing (feasibility check)

```bash
$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"
```

Find governing decisions. If matches found, **read the full decision file** — index summaries are insufficient.

### Before applying review fixes

```bash
$DECISIONS_CMD search "[RELEVANT_KEYWORDS]"
```

If review item contradicts an active decision, skip with reference (e.g., "Skipped — contradicts D010").

### During PR context gathering

```bash
$DECISIONS_CMD search --issue [ISSUE_ID]
```

Collect decision IDs and summaries for delegation prompts. Include in PR body:
```markdown
- **[DECISION_ID]**: [ONE_LINE_SUMMARY] — `[DECISION_FILE_PATH]`
```

### Contradict-check

Suggestions contradicting active decisions are invalid unless the decision itself is flawed (flag as blocker with justification).

---

## Decision Lifecycle

```
Research Complete → Create Decision
                        ↓
                 INDEX.md + DXXX-descriptor.md
                        ↓
            ┌───────────┴───────────┐
            ↓                       ↓
    Search/Reference         Update/Supersede
    (review, audit,          (new research,
     implementation)          revisit conditions met)
```

## Content Guidelines

- **Declares** what was chosen, not just what was considered
- **Explains** why with concrete rationale
- **Anticipates** change with specific revisit conditions
- **References** research for details — decision keeps tight summary
- **Links** to related decisions, code, and issues

### What to Log

- Technology selections with alternatives considered
- Performance trade-offs (chose X over Y for reason Z)
- Significant path choices where conditions might change
- Research-informed decisions

### What NOT to Log

- Variable names, small refactors, bug fixes
- Obvious choices with no realistic alternatives
- Standard pattern applications
