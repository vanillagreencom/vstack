---
title: IRQ Affinity Away from Critical Cores
impact: HIGH
impactDescription: Interrupt handling on latency-critical cores causes unpredictable jitter spikes
tags: irq, affinity, jitter, core-isolation, interrupts
---

## IRQ Affinity Away from Critical Cores

**Impact: HIGH (interrupt handling on latency-critical cores causes unpredictable jitter spikes)**

Hardware interrupts preempt running threads. If IRQs fire on cores running latency-sensitive work, they cause unpredictable jitter. Move all IRQs to non-critical cores.

```bash
# Check IRQ distribution across CPUs
cat /proc/interrupts | head -20

# Check specific IRQ affinity masks
for irq in /proc/irq/*/smp_affinity; do
    echo "$irq: $(cat $irq)"
done

# Move IRQs away from critical cores (e.g., cores 0-3)
# Set affinity to cores 4+ (mask depends on core count)
echo "fff0" | sudo tee /proc/irq/*/smp_affinity
```

Combine with core isolation (`isolcpus` kernel parameter) for maximum effect.
