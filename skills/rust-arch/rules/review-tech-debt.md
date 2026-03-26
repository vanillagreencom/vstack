---
title: Technical Debt Classification
impact: MEDIUM
impactDescription: unclassified debt gets lost or misprioritized
tags: review, tech-debt, classification
---

## Technical Debt Classification

**Impact: MEDIUM (unclassified debt gets lost or misprioritized)**

Classify discovered technical debt by impact and urgency:

| Priority | Impact | Timeline | Example |
|----------|--------|----------|---------|
| P1 Urgent | Blocks performance budget | Fix immediately | Mutex in tick processing |
| P2 High | Architectural violation | Fix this cycle | Circular dependency |
| P3 Normal | Code smell, tech debt | Plan for backlog | Missing abstraction |
| P4 Low | Minor improvement | Track only | Naming, docs |

### Tracking Format

```
TD-XXX: [Short description]
- Location: module/file description
- Impact: [Performance|Maintainability|Safety]
- Priority: P1|P2|P3|P4
- Estimate: 1-5 points
- Notes: [Additional context]
```
