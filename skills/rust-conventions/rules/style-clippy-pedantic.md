---
title: Clippy Pedantic
impact: MEDIUM
tags: clippy, pedantic, lints, casting
---

## Clippy Pedantic

**Impact: MEDIUM (CI failures from unaddressed lints)**

For projects running `-W clippy::pedantic -D warnings`:

- `#[inline]` not `#[inline(always)]` — let compiler decide
- Prefer `&T` over owned `T` for non-consumed params (`needless_pass_by_value`)
- `#[repr(...)]` before `#[derive(...)]`
- Run `cargo fmt` before commit — always
- Don't manually align inline comments — `cargo fmt` normalizes them

### Casting Lints

Pedantic flags `as` casts. Add `#[allow(clippy::...)]` with a justification comment:

| Lint | When |
|------|------|
| `cast_possible_truncation` | `u128 as u64`, `usize as u16` — bounded values |
| `cast_sign_loss` | `i32 as u32` on known-positive/negated values |
| `cast_possible_wrap` | `usize as i32` for returns bounded by design |
