# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. Hardware Event Accuracy (hw)

**Impact:** CRITICAL
**Description:** Hardware performance counter mappings vary across CPU vendors. Using generic events on the wrong architecture produces silently incorrect data, leading to wrong optimization decisions.

## 2. CPU Profiling (cpu)

**Impact:** HIGH
**Description:** Patterns for identifying CPU hot paths using flamegraphs, eBPF, and off-CPU analysis. Choosing the right profiler and configuration avoids measurement overhead artifacts.

## 3. Cache & TLB (cache)

**Impact:** HIGH
**Description:** Cache miss and TLB analysis patterns. Poor data locality is the most common cause of consistent high latency in memory-intensive systems.

## 4. NUMA Locality (numa)

**Impact:** HIGH
**Description:** NUMA topology discovery and memory pinning. Cross-socket memory access adds 40-100ns per access, catastrophic for latency-sensitive paths.

## 5. System Jitter (jitter)

**Impact:** HIGH
**Description:** Scheduler latency, IRQ affinity, core isolation, and real-time scheduling verification. System-level interference causes tail latency spikes invisible to application-level profiling.

## 6. Memory Profiling (mem)

**Impact:** MEDIUM
**Description:** Heap profiling, leak detection, and allocation-free verification. Unexpected allocations in hot paths cause latency spikes and GC-like pauses.
