---
title: Kernel Core Isolation Parameters
impact: HIGH
impactDescription: Scheduler noise injects microsecond jitter on shared cores
tags: isolcpus, nohz_full, rcu_nocbs, taskset, chrt, isolation
---

## Kernel Core Isolation Parameters

**Impact: HIGH (scheduler noise injects microsecond jitter on shared cores)**

`isolcpus=4,5,6,7` removes cores from the general scheduler — only explicitly pinned threads run there. Combine with `nohz_full=4,5,6,7` (disable timer ticks) and `rcu_nocbs=4,5,6,7` (offload RCU callbacks). For maximum isolation: `taskset -c 4 ./my_app` + `chrt -f 99` for FIFO real-time priority.

Verify isolation:
- `cat /sys/devices/system/cpu/isolated` — confirms isolated core list
- `ps -eo pid,comm,psr | awk '$3 == 4'` — confirms no unwanted processes on core 4

**Incorrect (running latency-critical thread on non-isolated core):**

```bash
# No kernel isolation — scheduler places housekeeping threads on same cores
./my_app  # Thread may share core with kworker, rcu, ksoftirqd
```

**Correct (full isolation stack):**

```bash
# Kernel boot params (grub/systemd-boot):
# isolcpus=4,5,6,7 nohz_full=4,5,6,7 rcu_nocbs=4,5,6,7

# Verify isolation
cat /sys/devices/system/cpu/isolated
# Output: 4-7

# Launch with core pinning + real-time priority
taskset -c 4 chrt -f 99 ./my_app

# Verify no unwanted processes on isolated core
ps -eo pid,comm,psr | awk '$3 == 4'
# Should show only your my_app process
```
