---
title: SPSC Latency Measurement
impact: HIGH
impactDescription: Criterion can't measure two-thread latency — wrong tool gives wrong numbers
tags: spsc, latency, benchmark, hdrhistogram, criterion, measurement
---

## SPSC Latency Measurement

**Impact: HIGH (Criterion can't measure two-thread latency — wrong tool gives wrong numbers)**

Criterion is single-threaded and cannot measure SPSC channel latency. Use a custom harness: spawn producer + consumer, pin to specific cores, producer writes timestamps, consumer records latency via `HdrHistogram`, run for fixed duration, report P50/P99/P99.9. Measure both intra-CCD and cross-CCD to quantify topology impact. Rate-limit producer to realistic message rate (e.g., 1M msg/s) — flood mode hides queueing effects.

**Incorrect (using Criterion for SPSC latency):**

```rust
// Criterion runs single-threaded — this measures push() alone, not end-to-end
fn bench_spsc(c: &mut Criterion) {
    let (mut prod, mut cons) = rtrb::RingBuffer::new(1024);
    c.bench_function("spsc", |b| {
        b.iter(|| {
            prod.push(42).unwrap();
            cons.pop().unwrap(); // Same thread — no contention, no cache transfer
        });
    });
}
```

**Correct (custom two-thread harness with histogram):**

```rust
use hdrhistogram::Histogram;
use std::time::{Duration, Instant};

fn measure_spsc_latency(producer_core: usize, consumer_core: usize) {
    let (mut prod, mut cons) = rtrb::RingBuffer::<Instant>::new(1024);
    let duration = Duration::from_secs(5);
    let msg_interval = Duration::from_micros(1); // 1M msg/s

    let consumer = std::thread::spawn(move || {
        pin_to_core(consumer_core);
        let mut hist = Histogram::<u64>::new(3).unwrap();
        while let Ok(sent_at) = cons.pop() {
            let latency_ns = sent_at.elapsed().as_nanos() as u64;
            hist.record(latency_ns).ok();
        }
        hist
    });

    pin_to_core(producer_core);
    let start = Instant::now();
    while start.elapsed() < duration {
        let _ = prod.push(Instant::now());
        std::thread::sleep(msg_interval); // Rate-limit to realistic rate
    }
    drop(prod); // Signal consumer to exit

    let hist = consumer.join().unwrap();
    println!("P50={}, P99={}, P99.9={}",
        hist.value_at_quantile(0.50),
        hist.value_at_quantile(0.99),
        hist.value_at_quantile(0.999));
}
```
