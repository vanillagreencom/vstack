---
title: Agent Sequencing by Data Dependency
impact: MEDIUM
impactDescription: Parallel agents produce conflicting changes when data dependencies exist
tags: coordination, sequencing, parallel
---

## Agent Sequencing by Data Dependency

**Impact: MEDIUM (parallel agents produce conflicting changes when data dependencies exist)**

When multiple agents work on related issues, determine blocking relations from data dependencies, not just agent type. Only set blocking relations if agent A creates types, APIs, or modules that agent B consumes. No data flow = no blocking, regardless of agent ordering.

Apply agent sequencing:
1. Infer agent from label or component path
2. Identify candidate pairs from sequential requirements
3. Confirm with Creates ↔ Consumes analysis
4. Set blocking relations on parent issues, not children, when bundled
