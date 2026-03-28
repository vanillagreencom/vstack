# Recommendation Bias

Guidelines for categorizing review findings as fix (apply in PR) vs issue (track separately). Applies to any codebase — bias toward reliability when uncertain.

## Verification Prerequisite

Before classifying any comment as noise, stale, or not actionable — read the actual file(s) and location(s) it references. Verify against current code. A comment is only stale if the code proves it so; never infer staleness from comment text, age, or assumptions about what prior fixes addressed.

**No file read = no dismissal.** The cost of reading a file is low; the cost of wrongly dismissing a valid comment is a bug shipped to production.

## Decision Flow

For each potential suggestion, evaluate in order:

**1. Actionable?** Must have specific deliverable, observable impact, and bounded scope.
- Vague ("Add logging to X", "Add tests for X", "Document X", "Consider X") → **omit**
- Informational notes (not actionable) → **omit**
- Exception: automated regression detection (e.g., benchmark exit code 1) is never informational — classify per project's regression rules

**2. Related?** Does it relate to the issue being fixed or the code being changed?
- The test is semantic (about the problem or changes), not mechanical (is the file in the diff)
- Out-of-diff files documenting the mechanism being fixed → related
- Improvements to nearby code not about the problem being solved → unrelated
- Documentation and reference updates for changed APIs/patterns → always **`fix`** (never `issue`). Updated inline, same PR.
- Unrelated → **`issue`** regardless of size

**3. Size?** Can it be applied directly in this PR?
- Small, apply directly → **`fix`**
- Needs delegation, tracking, history, or new files → **`issue`**

### When Uncertain

- About category: prefer `fix` for related suggestions
- About relevance: prefer `issue`
- Can't be a fix and not specific enough for an issue → omit

## Category Signals

All assume the suggestion passed steps 1-2 (actionable and related).

| Signal | Category |
|--------|----------|
| Small, quick to apply | `fix` |
| Doc/reference updates for changed code | `fix` — always, regardless of size |
| Needs tracking, delegation, or history | `issue` |
| Architectural change, cross-component | `issue` |
| Test coverage (add to existing test) | `fix` |
| Test coverage (new file/suite/scenarios) | `issue` |
| Performance fix in touched code | `fix` |
| Performance work needing benchmarks | `issue` |
| Error handling gaps | `issue` — silent failures have real cost |
| Security vulnerabilities | `fix` if quick, else `issue` — never skip |
| Data validation gaps | `fix` if quick, else `issue` — cascading failures |

## Issue Scope

- **In-scope**: Part of this PR's work, big enough for sub-issue tracking. Child of parent issue.
- **Out-of-scope**: Worth doing, not part of this PR. Tracked separately.

## Priority

1-4 scale. Review agents typically assign 2-4.

| Pri | Meaning | Use When |
|-----|---------|----------|
| P1 | Urgent | Blocks critical path |
| P2 | High | Important, architecture |
| P3 | Normal | Standard work |
| P4 | Low | Nice-to-have, cleanup |

## Key Principle

**"Low priority" ≠ omit.** Track if actionable. Vague items are noise, not visibility.
