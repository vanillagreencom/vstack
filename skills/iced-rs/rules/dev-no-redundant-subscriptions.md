---
title: No Redundant Event Subscriptions
impact: HIGH
impactDescription: Duplicate event handling, wasted computation, subtle bugs
tags: subscription, events, window
---

## No Redundant Event Subscriptions

**Impact: HIGH (duplicate event handling, wasted computation, subtle bugs)**

Before adding a new `window::*` or event subscription, check whether the same event family already flows through an existing listener. Extend the existing path unless a separate subscription is required and benchmarked.
