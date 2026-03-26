---
title: Message and State Stay in Root
impact: MEDIUM
impactDescription: Coupling and import cycles across modules
tags: elm, state, message, organization
---

## Message and State Stay in Root

**Impact: MEDIUM (coupling and import cycles across modules)**

Message enum and State struct stay in the root module. Extracted modules receive `&State` or `&mut State` references. Never split these across files. Root keeps: State, Message, boot/new/update/subscription/view dispatch, thin multi-subsystem accessors.
