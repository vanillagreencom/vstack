---
title: Extend Existing Event Paths
impact: HIGH
impactDescription: Parallel subscriptions cause duplicate handling and ordering bugs
tags: subscription, window, lifecycle, events
---

## Extend Existing Event Paths

**Impact: HIGH (parallel subscriptions cause duplicate handling and ordering bugs)**

When changing window lifecycle handling, prefer extending the existing global event path over adding parallel subscriptions for the same event family.
