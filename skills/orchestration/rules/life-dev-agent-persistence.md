---
title: Dev Agent Persistence
impact: HIGH
impactDescription: Context loss between fix cycles degrades fix quality
tags: lifecycle, persistence, dev-agent
---

## Dev Agent Persistence

**Impact: HIGH (context loss between fix cycles degrades fix quality)**

Dev agents persist for the entire session — never shut down except at finalization. After completing initial work, they may be re-delegated for review fix items, QA fix items, comment fixes, or CI failure fixes. Each re-delegation: create new tasks → send message with delegation. The agent wakes, finds new PENDING tasks by prefix.
