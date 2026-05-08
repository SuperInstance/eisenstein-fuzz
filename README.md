# eisenstein-fuzz

Property-based fuzzing for [Eisenstein integers](https://en.wikipedia.org/wiki/Eisenstein_integer) — proving the zero-drift claim through exhaustive automated testing.

## What is this?

The [`eisenstein`](https://github.com/SuperInstance/eisenstein) crate provides exact integer arithmetic for Eisenstein integers on the hexagonal lattice. This fuzzing project hammers it with millions of random inputs to verify that the mathematical properties hold universally — no floating point, no approximations, no drift.

## Properties tested

| Target | Property | Why it matters |
|--------|----------|----------------|
| `rotation_identity` | Rotating (a,b) → (-b, a+b) six times returns the original | D6 rotational symmetry of the hexagonal lattice |
| `norm_nonneg` | a² - ab + b² ≥ 0 for all integers | Foundational "no negative drift" invariant |
| `ring_axioms` | Associativity, commutativity, distributivity | Eisenstein integers form a valid commutative ring |
| `d6_closure` | Rotation preserves norm; neighbors are bounded | D6 symmetry group acts on the lattice |
| `conjugate_involution` | conj(conj(p)) = p; norm preserved | Complex conjugation is an involution |
| `disk_coverage` | HexDisk count = 3R² + 3R + 1, no duplicates, all inside | Enumeration correctness for hex grids |

## Quick start

### Property tests (no fuzz setup needed)

```bash
cargo test
```

This runs ~300K property checks in seconds using deterministic pseudo-random inputs.

### Fuzz tests (requires cargo-fuzz + nightly)

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run individual targets
cargo fuzz run rotation_identity -- -runs=100000 -max_total_time=60
cargo fuzz run norm_nonneg -- -runs=100000 -max_total_time=60
cargo fuzz run ring_axioms -- -runs=100000 -max_total_time=60
cargo fuzz run d6_closure -- -runs=100000 -max_total_time=60
cargo fuzz run conjugate_involution -- -runs=100000 -max_total_time=60
cargo fuzz run disk_coverage -- -runs=100000 -max_total_time=60
```

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

## The zero-drift claim

Eisenstein integers provide exact arithmetic on a hexagonal lattice with zero cumulative drift. This is critical for safety-critical systems where floating-point accumulation errors are unacceptable. Every operation preserves:

1. **Integer exactness** — no rounding, ever
2. **Norm multiplicativity** — ‖a·b‖ = ‖a‖·‖b‖
3. **D6 symmetry** — all rotations preserve norm
4. **Ring closure** — the Eisenstein integers are closed under +, -, ·

This fuzzing project exists to prove these claims hold for all inputs, not just the ones we thought to test.

## License

MIT OR Apache-2.0
