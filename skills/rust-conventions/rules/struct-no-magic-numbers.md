---
title: No Magic Numbers
impact: HIGH
tags: constants, literals, modularity
---

## No Magic Numbers

**Impact: HIGH (silent divergence when values change)**

Numeric literals used more than once or with non-obvious meaning must be named constants (`const` at module top). Includes: thresholds, percentages, pixel sizes, timing values. Exception: 0, 1, 2 in obvious contexts.
