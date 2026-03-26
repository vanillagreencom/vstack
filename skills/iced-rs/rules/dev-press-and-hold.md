---
title: Press-and-Hold Input
impact: HIGH
impactDescription: Hold actions fire on release instead of press
tags: button, mouse_area, on_press, input
---

## Press-and-Hold Input

**Impact: HIGH (hold actions fire on release instead of press)**

`button(...).on_press(...)` fires on mouse-up (release). For true mouse-down behavior (repeat scroll, press-and-hold actions), use `mouse_area(...).on_press(...)`.
