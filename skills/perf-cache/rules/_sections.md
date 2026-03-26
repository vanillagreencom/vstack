# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Data Layout (layout)

**Impact:** CRITICAL
**Description:** Struct layout, field ordering, hot/cold splitting, and AoS vs SoA decisions. Poor layout wastes cache lines on every access and dominates hot-path latency.

## 2. False Sharing (sharing)

**Impact:** CRITICAL
**Description:** Detection and prevention of false sharing between threads. A single false-sharing site can degrade multi-threaded throughput by 10-100x.

## 3. Prefetching & Pages (mem)

**Impact:** HIGH
**Description:** Hardware prefetch patterns, huge pages, and memory locking. Controls TLB miss rate, page fault latency, and prefetcher effectiveness on non-sequential access.

## 4. Measurement (meas)

**Impact:** HIGH
**Description:** Cache performance measurement tools, thresholds, and simulation. Without measurement, cache optimization is guesswork.
