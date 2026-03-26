---
title: Metric Kind Must Match
impact: HIGH
impactDescription: Invalid comparison between wall_time and instruction_count
tags: recording, metric, comparison
---

## Metric Kind Must Match

**Impact: HIGH (invalid comparison between wall_time and instruction_count)**

Baseline and latest results must use the same `metric_kind`. Cross-kind comparison (e.g., Criterion wall_time vs iai-callgrind instruction_count) is rejected. For `latency_distribution`, compare within the same topology label.
