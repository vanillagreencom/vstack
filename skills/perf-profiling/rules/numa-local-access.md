---
title: NUMA Local Memory Access
impact: HIGH
impactDescription: Cross-socket memory access adds 40-100ns per access
tags: numa, memory, locality, socket, pinning
---

## NUMA Local Memory Access

**Impact: HIGH (cross-socket memory access adds 40-100ns per access)**

On multi-socket or multi-chiplet systems, memory access latency depends on which NUMA node owns the memory. Remote access is 2-5x slower. Target >95% local access ratio.

```bash
# Discover NUMA topology
numactl --hardware
lscpu | grep -i numa
cat /sys/devices/system/node/node*/cpulist

# Monitor local vs remote memory access
numastat -p $(pidof my_app)
# Target: >95% local (numa_hit / (numa_hit + numa_miss))

# Pin process to NUMA node 0
numactl --cpunodebind=0 --membind=0 ./my_app

# Pin to specific CPUs and memory node
numactl --physcpubind=0-3 --membind=0 ./my_app

# Check which NUMA node NIC is on (pin app to same node)
cat /sys/class/net/eth0/device/numa_node
```

Watch NUMA stats over time to detect drift: `watch -n 1 'numastat -p $(pidof my_app)'`
