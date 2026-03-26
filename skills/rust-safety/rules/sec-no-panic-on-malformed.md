---
title: No Panics on Malformed Data
impact: HIGH
impactDescription: Panics on external input enable denial of service
tags: security, panic, unwrap, external-input
---

## No Panics on Malformed Data

**Impact: HIGH (panics on external input enable denial of service)**

Code handling external input must return `Result`, never `unwrap()` or `expect()` on data that could be malformed. A panic triggered by crafted input is a denial-of-service vulnerability. Use `unwrap()` only on invariants proven by prior validation in the same function.
