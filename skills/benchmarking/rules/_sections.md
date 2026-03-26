# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Regression Classification (reg)

**Impact:** CRITICAL
**Description:** How to classify and report performance regressions. Misclassification causes missed blockers or unnecessary churn.

## 2. Recording (rec)

**Impact:** HIGH
**Description:** Rules for recording benchmark results correctly. Violations cause missing data or invalid comparisons.

## 3. Schema (schema)

**Impact:** MEDIUM
**Description:** Schema v3 format requirements and metric kind semantics. Violations cause cross-tool comparison failures.
