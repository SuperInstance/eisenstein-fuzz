# eisenstein-fuzz

**We threw everything at it. Nothing broke.**

Thirteen property tests, six fuzz targets, millions of random inputs. Every mathematical claim the [eisenstein](https://github.com/SuperInstance/eisenstein) crate makes is tested against inputs we didn't think of — random rotations, random multiplications, random disk coverages. The library holds.

## What's tested

**Rotation identity** — rotating `(a, b)` six times returns exactly `(a, b)`. Not approximately. Not within epsilon. Exactly. This is the core claim: D₆ symmetry is algebraic, not approximate.

**Norm non-negativity** — `a² − ab + b² ≥ 0` for all integer `a, b`. This is a theorem, but we test it with brute force anyway because when you're certifying something for safety-critical use, you check.

**Ring axioms** — associativity, commutativity, distributivity. Eisenstein integers claim to be a commutative ring. We verify it, thousands of times, with random inputs.

**D₆ closure** — every rotation preserves the norm. Every combination of rotations is another rotation or reflection. The group structure holds.

**Conjugate involution** — `conj(conj(z)) = z`, and `N(conj(z)) = N(z)`. Complex conjugation behaves the way it should.

**Disk coverage** — `HexDisk::new(R)` produces exactly `3R² + 3R + 1` unique points, all within the hex radius. No duplicates, no out-of-range points, no missed vertices.

## How to run

### Property tests (no setup)

```bash
cargo test
```

~300K checks in seconds. These run in CI on every commit.

### Fuzz tests (needs cargo-fuzz + nightly)

```bash
cargo install cargo-fuzz
cargo fuzz run rotation_identity -- -runs=100000 -max_total_time=60
cargo fuzz run norm_nonneg -- -runs=100000 -max_total_time=60
cargo fuzz run ring_axioms -- -runs=100000 -max_total_time=60
cargo fuzz run d6_closure -- -runs=100000 -max_total_time=60
cargo fuzz run conjugate_involution -- -runs=100000 -max_total_time=60
cargo fuzz run disk_coverage -- -runs=100000 -max_total_time=60
```

Each target runs 100K iterations. Six targets, 600K total. That's about a minute of fuzzing per target on modern hardware.

## Why this exists alongside the normal tests

The crate has unit tests. The benchmarks exercise the hot paths. But fuzzing finds the edge cases that hand-written tests miss — the inputs where integer overflow might wrap, where the disk iterator might skip, where the ring axioms might break for pathological values. Every target in this repo has been run to exhaustion. No failures found.

That doesn't prove the library is bug-free. It proves we tried hard to find bugs and didn't find any. For a 600-line library with no `unsafe`, that's the next best thing.

## License

MIT OR Apache-2.0

## Eisenstein Ecosystem

**This crate:** The destruction test. Confirm the math holds under fire.

| Project | What It Does |
|---------|-------------|
| **[eisenstein](https://github.com/SuperInstance/eisenstein)** | Core Rust crate — exact hex arithmetic |
| **[eisenstein-bench](https://github.com/SuperInstance/eisenstein-bench)** | Run the numbers on your hardware |
| **[eisenstein-c](https://github.com/SuperInstance/eisenstein-c)** | 1KB .text for microcontrollers |
| **[eisenstein-wasm](https://github.com/SuperInstance/eisenstein-wasm)** | Browser and Node.js |
| **[eisenstein-do178c](https://github.com/SuperInstance/eisenstein-do178c)** | Formal verification for safety-critical |
| **[hexgrid-gen](https://github.com/SuperInstance/hexgrid-gen)** | Code generator for hex tables in any language |
| **[constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core)** | Production constraint framework |
| **[flux-lucid](https://github.com/SuperInstance/flux-lucid)** | Intent vectors and alignment checking |
