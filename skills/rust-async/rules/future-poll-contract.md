---
title: "Future::poll Contract"
impact: CRITICAL
tags: future, poll, waker, pending, ready
---

## Future::poll Contract

**Impact: CRITICAL (busy-loop or hung task if violated)**

The `Future::poll` contract has three invariants:

1. **Return `Pending` only after registering a waker.** If you return `Poll::Pending` without calling `cx.waker().wake_by_ref()` or storing the waker for later notification, the executor has no way to know when to re-poll. The future is never woken and appears hung.

2. **Never return `Pending` without waker registration.** This causes a busy-loop: the executor keeps polling, gets `Pending`, and has no wake signal, so it either spins or gives up.

3. **Never poll after `Ready`.** Once a future returns `Poll::Ready(value)`, polling again is a logic error. The future may panic or return garbage. Use `FusedFuture` or `.fuse()` if you need safe repeated polling.

**Incorrect (Pending without waker registration):**

```rust
impl Future for MyFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(42)
        } else {
            // BUG: no waker registered — this future will never wake
            Poll::Pending
        }
    }
}
```

**Correct (waker stored for later notification):**

```rust
impl Future for MyFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(42)
        } else {
            // Store waker so the background task can call wake() when ready
            self.get_mut().waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```
