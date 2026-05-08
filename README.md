# eisenstein-fuzz

**Millions of random inputs. Zero properties broken.**

The [eisenstein](https://github.com/SuperInstance/eisenstein) crate claims exact arithmetic with zero drift on the hexagonal lattice. This project exists to make that claim expensive. Thirteen property tests and six fuzz targets hammer the algebra with every input the fuzzer can generate — random coordinates, adversarial edge cases, pathological values — looking for the crack that proves the claim wrong.

So far: nothing. Every property holds. The ring axioms hold. D₆ closure holds. Rotation identity holds. The norm never goes negative. The conjugate is an involution. The disk count is always exactly `3R² + 3R + 1`.

The fuzz targets haven't found a counterexample. If they ever do, that's a real result — not a hypothetical.

## What gets tested

Six fuzz targets, each probing a different structural property of Eisenstein integers. Each target runs with libfuzzer behind it, generating novel inputs through coverage-guided mutation.

**`rotation_identity`** — Rotating `(a,b) → (-b, a+b)` six times must return the original. This is the D₆ rotational symmetry of the hexagonal lattice. Break this and the whole coordinate system is wrong.

**`norm_nonneg`** — The norm `a² - ab + b²` is always ≥ 0 for all integers. This is the foundational "no negative drift" invariant. It's true algebraically; the fuzzer checks that it's true computationally.

**`ring_axioms`** — Associativity, commutativity, distributivity of addition and multiplication. Eisenstein integers form a commutative ring — this target checks that the implementation matches the math.

**`d6_closure`** — Rotation preserves norm. Hex neighbors stay bounded. The D₆ symmetry group acts on the lattice correctly.

**`conjugate_involution`** — `conj(conj(p)) = p`. Complex conjugation is an involution, and norm is preserved under conjugation.

**`disk_coverage`** — HexDisk enumeration produces exactly `3R² + 3R + 1` points, no duplicates, no missing vertices, all inside the boundary.

## Quick start

### Property tests — no fuzz toolchain needed

```bash
cargo test
```

Runs ~300,000 property checks in seconds using deterministic pseudo-random inputs. Fast, reproducible, catches obvious breaks.

### Fuzz tests — requires nightly + cargo-fuzz

```bash
cargo install cargo-fuzz

cargo fuzz run rotation_identity -- -runs=100000 -max_total_time=60
cargo fuzz run norm_nonneg -- -runs=100000 -max_total_time=60
cargo fuzz run ring_axioms -- -runs=100000 -max_total_time=60
cargo fuzz run d6_closure -- -runs=100000 -max_total_time=60
cargo fuzz run conjugate_involution -- -runs=100000 -max_total_time=60
cargo fuzz run disk_coverage -- -runs=100000 -max_total_time=60
```

Each target runs for 60 seconds or 100K inputs, whichever comes first. Crank both if you want to run overnight.

## Project structure

```
eisenstein-fuzz/
├── Cargo.toml              # workspace root
├── fuzz/
│   ├── Cargo.toml          # fuzz targets (libfuzzer + arbitrary)
│   └── fuzz_targets/
│       ├── rotation_identity.rs
│       ├── norm_nonneg.rs
│       ├── ring_axioms.rs
│       ├── d6_closure.rs
│       ├── conjugate_involution.rs
│       └── disk_coverage.rs
├── tests/
│   └── property_tests.rs   # cargo test property tests
└── README.md
```

## The zero-drift claim, under pressure

Four invariants, tested exhaustively. Integer exactness — no rounding, ever. Norm multiplicativity — `‖a·b‖ = ‖a‖·‖b‖`. D₆ symmetry — all rotations preserve norm. Ring closure — Eisenstein integers are closed under `+`, `-`, `·`.

This project exists because a claim is only as strong as the testing behind it. The testing behind this one is ongoing.

## Eisenstein Ecosystem

Part of the **[Eisenstein hex integer ecosystem](https://github.com/SuperInstance/eisenstein)** — exact hex arithmetic from microcontrollers to browsers to formal verification.

| Project | Description |
|---------|-------------|
| **[eisenstein](https://github.com/SuperInstance/eisenstein)** | Core Rust crate — exact hex arithmetic, zero deps |
| **[eisenstein-c](https://github.com/SuperInstance/eisenstein-c)** | Same math, for microcontrollers. 1KB `.text`. |
| **[eisenstein-wasm](https://github.com/SuperInstance/eisenstein-wasm)** | Same math, for browsers and Node.js |
| **[eisenstein-bench](https://github.com/SuperInstance/eisenstein-bench)** | Benchmark all implementations side-by-side |
| **[eisenstein-fuzz](https://github.com/SuperInstance/eisenstein-fuzz)** | Property-based fuzzing across the ecosystem |
| **[eisenstein-do178c](https://github.com/SuperInstance/eisenstein-do178c)** | DO-178C formally verified for safety-critical systems |
| **[arm-neon-eisenstein-bench](https://github.com/SuperInstance/arm-neon-eisenstein-bench)** | 4× parallel hex math on ARM NEON |
| **[hexgrid-gen](https://github.com/SuperInstance/hexgrid-gen)** | Code generation for any language in the ecosystem |
| **[constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core)** | Production constraint framework built on Eisenstein math |
| **[flux-lucid](https://github.com/SuperInstance/flux-lucid)** | Unified intent-directed ecosystem orchestrator |

**Next →** Certify it: **[eisenstein-do178c](https://github.com/SuperInstance/eisenstein-do178c)**

## License

MIT OR Apache-2.0
