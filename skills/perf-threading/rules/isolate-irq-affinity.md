---
title: IRQ Affinity Steering
impact: HIGH
impactDescription: Hardware interrupts on trading cores inject 5-50us jitter
tags: irq, affinity, nic, interrupts, smp_affinity
---

## IRQ Affinity Steering

**Impact: HIGH (hardware interrupts on trading cores inject 5-50us jitter)**

Move all IRQs away from isolated cores. Exception: the NIC IRQ for the trading feed should be on the same NUMA node as the trading thread but NOT on the same core.

**Incorrect (IRQs still routed to isolated cores):**

```bash
# Isolated cores 4-7 but IRQs not redirected
# Hardware interrupts still fire on core 4, injecting jitter
cat /proc/interrupts | grep eth0
#            CPU0  CPU1  CPU2  CPU3  CPU4  ...
# eth0:       102    55    37    28   312   <- IRQs hitting core 4
```

**Correct (steer all IRQs away, place NIC IRQ on same NUMA node):**

```bash
# Move ALL IRQs to housekeeping cores (0-3)
for i in /proc/irq/*/smp_affinity_list; do echo "0-3" > "$i"; done

# Set NIC to single queue for deterministic IRQ placement
ethtool -L eth0 combined 1

# Place NIC IRQ on same NUMA node as trading thread, different core
# Trading thread on core 4 (NUMA 0), NIC IRQ on core 2 (NUMA 0)
echo "2" > /proc/irq/<nic_irq>/smp_affinity_list

# Verify
cat /proc/interrupts | awk '{print $NF}' | sort -u
```
