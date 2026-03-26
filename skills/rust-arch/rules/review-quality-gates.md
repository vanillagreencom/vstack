---
title: Quality Gates
impact: MEDIUM
impactDescription: architectural violations slip through without gates
tags: review, quality-gates, checklist
---

## Quality Gates

**Impact: MEDIUM (architectural violations slip through without gates)**

Every architecture review must pass these gates before approval:

- [ ] Follows layered architecture (dependencies flow down)
- [ ] No circular dependencies
- [ ] Abstractions at module boundaries
- [ ] No hot-path anti-patterns
- [ ] Platform differences handled explicitly
- [ ] Pre-allocation strategy documented
- [ ] Error handling strategy clear

**Reject if any gate fails.**
