---
title: Smoke Test After UI Changes
impact: HIGH
impactDescription: Runtime panics not caught by clippy
tags: testing, runtime, panics, wgpu, tokio
---

## Smoke Test After UI Changes

**Impact: HIGH (runtime panics not caught by clippy)**

Clippy catches compile errors but not runtime panics (missing Tokio runtime, wgpu init failures, font loading). Always run the app briefly after UI changes to catch these.
