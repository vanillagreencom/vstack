---
title: Test Modules Mirror Production
impact: HIGH
tags: testing, organization, extraction, modules
---

## Test Modules Mirror Production

**Impact: HIGH (stale catch-all test files after extraction)**

When production responsibilities split into focused files, move the matching focused tests and shared fixtures into sibling `#[path]` test modules in the same change. Do not leave the old catch-all test file as the stale owner.
