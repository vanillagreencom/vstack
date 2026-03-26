# Sections

This file defines all sections, their ordering, impact levels, and descriptions.
The section ID (in parentheses) is the filename prefix used to group rules.

---

## 1. CPU Topology (topo)

**Impact:** CRITICAL
**Description:** CPU topology discovery and thread pinning — CCD/L3 sharing groups, hyperthreading siblings, NUMA awareness. Incorrect placement adds 2-3x latency on cross-CCD communication.

## 2. Core Isolation (isolate)

**Impact:** HIGH
**Description:** Kernel-level core isolation, timer tick suppression, IRQ affinity steering. Without isolation, kernel housekeeping and interrupts inject microsecond-scale jitter into latency-critical threads.

## 3. SPSC Patterns (spsc)

**Impact:** HIGH
**Description:** Single-producer single-consumer channel selection, topology-aware placement, and latency measurement. Wrong channel or cross-CCD placement adds 100-200ns per message on the hot path.

## 4. Page Fault Prevention (fault)

**Impact:** HIGH
**Description:** Memory locking, stack pre-faulting, and steady-state page fault elimination. A single minor page fault costs 1-5us — unacceptable on latency-critical paths.
