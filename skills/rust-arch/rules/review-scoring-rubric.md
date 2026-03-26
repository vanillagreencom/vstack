---
title: Review Scoring Rubric
impact: MEDIUM
impactDescription: inconsistent review standards without a framework
tags: review, scoring, quality
---

## Review Scoring Rubric

**Impact: MEDIUM (inconsistent review standards without a framework)**

Architecture reviews score across five dimensions:

| Dimension | Weight | Pass | Focus |
|-----------|--------|------|-------|
| **Design Efficiency** | 2x | >=90 | No anti-patterns in hot path |
| Modularity | 1x | >=80 | Clean boundaries, single responsibility |
| Maintainability | 1x | >=80 | Easy to modify, understand |
| Testability | 1x | >=80 | Easy to unit test, mock |
| Scalability | 1x | >=80 | Can handle growth |

**Formula:** (Design Efficiency x 2 + Modularity + Maintainability + Testability + Scalability) / 6

**Pass criteria:** Overall >=80 AND Design Efficiency >=90.

### Scoring Guide

**Design Efficiency (0-100):**
- 100: Zero anti-patterns, optimal data flow
- 90: Minor inefficiencies, no hot-path issues
- 70: Some anti-patterns, not in critical path
- 50: Anti-patterns affect performance
- <50: Critical anti-patterns in hot path

**Modularity (0-100):**
- 100: Perfect separation, clear interfaces
- 80: Good boundaries, minor coupling
- 60: Some tight coupling
- <60: Circular dependencies or god objects
