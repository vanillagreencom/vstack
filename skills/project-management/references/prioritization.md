# Prioritization Guide

## Factor Definitions

### Critical Path (x3)

| Score | Criteria |
|-------|----------|
| 3 | Blocks release |
| 2 | Blocks feature |
| 1 | Nice to have |
| 0 | Independent |

### Dependencies (x2)

| Score | Criteria |
|-------|----------|
| 3 | 3+ issues blocked |
| 2 | 1-2 issues blocked |
| 1 | Soft dependency |
| 0 | None |

**Tip**: Query issue relations to count blocked issues:
```bash
$ISSUE_CLI cache issues list-relations [ISSUE_ID]
```

### Risk (x2)

| Score | Criteria |
|-------|----------|
| 3 | Unknown technology |
| 2 | Complex integration |
| 1 | Straightforward |
| 0 | Trivial |

### Value (x1)

| Score | Criteria |
|-------|----------|
| 3 | Core functionality |
| 2 | Enhancement |
| 1 | Polish |
| 0 | Optional |

### Estimate (x0.5, subtracted)

| Score | Criteria |
|-------|----------|
| 3 | >1 week (5 pts) |
| 2 | 2-5 days (4 pts) |
| 1 | 1-2 days (2-3 pts) |
| 0 | <1 day (1 pt) |

## Example

**API Gateway Service**:
- Critical Path: 3 (blocks all downstream service communication)
- Dependencies: 3 (blocks auth, billing, notifications)
- Risk: 3 (first-time service mesh integration)
- Value: 3 (core infrastructure)
- Estimate: 2 (~3-4 days, 4 pts)

Score: (3x3) + (3x2) + (3x2) + (3x1) - (2x0.5) = **23** → P1

## Trade-offs

- **High risk + high value**: Spike first (1-2 days exploration)
- **Large issue blocking many**: Break into smaller deliverables
- **Multiple P1 items**: True dependencies first, then risk, then value
