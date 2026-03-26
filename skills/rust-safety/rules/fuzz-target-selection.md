---
title: Fuzz Target Selection
impact: HIGH
impactDescription: Fuzzing the wrong targets wastes CI time while leaving attack surface untested
tags: fuzzing, testing, security, coverage, prioritization
---

## Fuzz Target Selection

**Impact: HIGH (fuzzing the wrong targets wastes CI time while leaving attack surface untested)**

Fuzz testing is most effective on code that processes untrusted or complex input. Prioritize targets by attack surface and complexity.

**Fuzz these (high value):**
- Parsers (file formats, network protocols, configuration)
- Deserializers (serde implementations, custom binary formats)
- FFI boundaries (data crossing language boundaries)
- Unsafe code (pointer arithmetic, slice construction from raw parts)
- Protocol handlers (message framing, state machines)
- Codec implementations (compression, encryption, encoding)

**Do not fuzz (low value):**
- Pure business logic with bounded, well-typed inputs
- UI code and rendering
- Trivially correct code (simple getters, field access)
- Code with no unsafe and no external input

**Prioritization rule:** Anything processing untrusted input is a fuzz target. If an attacker controls the bytes, fuzz it.

**Coverage-guided fuzzing:** libFuzzer tracks code coverage to explore new execution paths. It mutates inputs that trigger new branches, progressively reaching deeper code paths. This makes it far more effective than random input generation.

**Seed corpus for faster results:**

```bash
# Add known edge cases as seed inputs
mkdir -p corpus/my_target
echo -n "" > corpus/my_target/empty
echo -n "valid_input" > corpus/my_target/basic
cp test_fixtures/edge_case.bin corpus/my_target/
```

Seed corpus inputs give the fuzzer a starting point with known coverage, dramatically reducing time to find new paths compared to starting from empty input.

**Incorrect (fuzzing trivial safe code):**

```rust
// Wasted effort — this code cannot crash or have memory errors
fuzz_target!(|data: &[u8]| {
    let x: u32 = data.len() as u32;
    let _ = x.saturating_add(1);
});
```

**Correct (fuzzing a parser that processes untrusted input):**

```rust
fuzz_target!(|data: &[u8]| {
    // Parser processes untrusted network input — high fuzz value
    let _ = my_crate::protocol::parse_message(data);
});
```
