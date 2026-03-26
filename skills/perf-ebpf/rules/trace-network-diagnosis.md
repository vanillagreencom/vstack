---
title: Network Diagnosis
impact: HIGH
impactDescription: missed retransmits or buffer issues cause unexplained latency
tags: bpftrace, tcp, retransmit, xdp, network, multicast
---

## Network Diagnosis

**Impact: HIGH (missed retransmits or buffer issues cause unexplained latency)**

For exchange connectivity: `bpftrace -e 'kprobe:tcp_retransmit_skb { @[kstack] = count(); }'` (retransmit sources), `bpftrace -e 'kprobe:tcp_rcv_established { @bytes = hist(arg2); }'` (receive buffer sizes). XDP in Aya for dropping irrelevant multicast at line rate before kernel stack processing.

**Incorrect (tcpdump for retransmit analysis):**

```bash
# tcpdump copies every packet to userspace — high overhead on busy links
# Must post-process pcap to find retransmits
tcpdump -i eth0 -w capture.pcap
# Then: tshark -r capture.pcap -Y "tcp.analysis.retransmission"
```

**Correct (bpftrace for targeted network diagnostics):**

```bash
# Retransmit sources with kernel stack trace — zero packet copying
bpftrace -e 'kprobe:tcp_retransmit_skb { @[kstack] = count(); }'

# Receive buffer size distribution
bpftrace -e 'kprobe:tcp_rcv_established { @bytes = hist(arg2); }'
```

```rust
// XDP program: drop irrelevant multicast at line rate
// Runs before sk_buff allocation — no kernel stack overhead
#[xdp]
pub fn multicast_filter(ctx: XdpContext) -> u32 {
    match try_filter(&ctx) {
        Ok(action) => action,
        Err(_) => xdp_action::XDP_PASS,
    }
}

fn try_filter(ctx: &XdpContext) -> Result<u32, ()> {
    let eth = unsafe { ptr_at::<EthHdr>(ctx, 0)? };
    // Drop multicast groups we don't subscribe to
    if is_irrelevant_multicast(unsafe { &*eth }) {
        return Ok(xdp_action::XDP_DROP); // Dropped before kernel stack
    }
    Ok(xdp_action::XDP_PASS)
}
```
