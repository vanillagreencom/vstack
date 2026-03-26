---
title: New Public Functions Require Tests
impact: HIGH
tags: testing, coverage, completeness
---

## New Public Functions Require Tests

**Impact: HIGH (untested public API surface)**

Unit test for happy path + at least one error case. Exception: trivial getters/setters, generated code. Don't reduce coverage without reason — removing tests requires explanation in commit message. Moving/consolidating tests is fine; deleting without replacement is not.
