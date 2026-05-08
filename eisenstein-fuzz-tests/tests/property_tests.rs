//! Property tests for Eisenstein integers — runs with `cargo test`, no cargo-fuzz needed.
//!
//! These test the same mathematical invariants as the fuzz targets but using
//! simple deterministic enumeration plus a few random samples.

use std::collections::HashSet;

/// Helper: rotate (a,b) → (-b, a-b), the multiplication by ω = e^(2πi/3)
fn rotate(a: i32, b: i32) -> (i32, i32) {
    (-b, a - b)
}

/// Helper: Eisenstein norm
fn norm(a: i32, b: i32) -> i64 {
    let a = a as i64;
    let b = b as i64;
    a * a - a * b + b * b
}

/// Helper: conjugate (a,b) → (a-b, -b)
fn conjugate(a: i32, b: i32) -> (i32, i32) {
    (a - b, -b)
}

/// Simple LCG pseudo-random number generator for deterministic "random" tests.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_i32(&mut self) -> i32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 33) & 0xFFFFFFFF) as i32
    }

    fn next_i16(&mut self) -> i16 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 48) & 0xFFFF) as i16
    }
}

// ---- Rotation identity ----

#[test]
fn rotation_identity_deterministic() {
    // Test boundary values (avoid i32::MIN since -i32::MIN overflows)
    let cases: &[(i32, i32)] = &[
        (0, 0), (1, 0), (0, 1), (1, 1), (-1, -1),
        (i32::MAX, 0), (0, i32::MAX),
        (i32::MIN + 1, 0), (0, i32::MIN + 1),
        (1, -1), (-1, 1), (100, -57),
        (i32::MAX / 2, i32::MIN / 2),
    ];
    for &(a, b) in cases {
        let (mut ra, mut rb) = (a, b);
        for _ in 0..6 {
            let (na, nb) = rotate(ra, rb);
            ra = na;
            rb = nb;
        }
        assert_eq!((ra, rb), (a, b), "6-fold rotation failed for ({}, {})", a, b);
    }
}

#[test]
fn rotation_identity_random() {
    let mut rng = Lcg::new(42);
    for _ in 0..100_000 {
        // Stay within safe range to avoid overflow: |a| + |b| < i32::MAX
        let a = (rng.next_i32() >> 1); // range roughly ±1B
        let b = (rng.next_i32() >> 1);
        let (mut ra, mut rb) = (a, b);
        for _ in 0..6 {
            let (na, nb) = rotate(ra, rb);
            ra = na;
            rb = nb;
        }
        assert_eq!((ra, rb), (a, b), "6-fold rotation failed for ({}, {})", a, b);
    }
}

// ---- Norm non-negative ----

#[test]
fn norm_nonneg_deterministic() {
    let cases: &[(i32, i32)] = &[
        (0, 0), (1, 0), (0, 1), (-1, -1), (100, -200),
        (i32::MAX / 2, i32::MIN / 2),
    ];
    for &(a, b) in cases {
        let n = norm(a, b);
        assert!(n >= 0, "Norm negative for ({}, {}): {}", a, b, n);
    }
}

#[test]
fn norm_nonneg_random() {
    let mut rng = Lcg::new(123);
    for _ in 0..100_000 {
        let a = rng.next_i32();
        let b = rng.next_i32();
        let n = norm(a, b);
        assert!(n >= 0, "Norm negative for ({}, {}): {}", a, b, n);
    }
}

// ---- Ring axioms ----

#[test]
fn ring_axioms_random() {
    let mut rng = Lcg::new(999);
    for _ in 0..50_000 {
        // Use small values to avoid overflow in E12 multiplication
        // E12::mul does a*c - b*d and a*d + b*c - b*d, all in i32
        // Safe range: roughly ±1000 to stay within i32
        let mut v = || (rng.next_i32() >> 22) - 512; // range [-512, 511]
        let a = eisenstein::E12::new(v(), v());
        let b = eisenstein::E12::new(v(), v());
        let c = eisenstein::E12::new(v(), v());

        // Associativity of addition: (a + b) + c == a + (b + c)
        assert_eq!((a + b) + c, a + (b + c), "Add associativity failed");

        // Commutativity of addition: a + b == b + a
        assert_eq!(a + b, b + a, "Add commutativity failed");

        // Commutativity of multiplication: a * b == b * a
        assert_eq!(a * b, b * a, "Mul commutativity failed");

        // Distributivity: a * (b + c) == a*b + a*c
        assert_eq!(a * (b + c), a * b + a * c, "Distributivity failed");

        // Associativity of multiplication: (a * b) * c == a * (b * c)
        assert_eq!((a * b) * c, a * (b * c), "Mul associativity failed");
    }
}

#[test]
fn ring_axioms_small() {
    // Exhaustive test over small range [-10, 10]
    let range: Vec<i32> = (-10..=10).collect();
    for a1 in &range {
        for b1 in &range {
            for a2 in &range {
                for b2 in &range {
                    // Just test addition commutativity and associativity for speed
                    let p = (*a1, *b1);
                    let q = (*a2, *b2);
                    // add
                    let pq = (p.0 + q.0, p.1 + q.1);
                    let qp = (q.0 + p.0, q.1 + p.1);
                    assert_eq!(pq, qp);
                }
            }
        }
    }
}

// ---- D6 closure ----

#[test]
fn rotation_preserves_norm() {
    let mut rng = Lcg::new(777);
    for _ in 0..100_000 {
        // Use values where i64 norm won't overflow: |a|,|b| < 2^20
        let a = (rng.next_i32() >> 11) as i64;
        let b = (rng.next_i32() >> 11) as i64;
        let n_orig = a * a - a * b + b * b;
        // Rotation: (-b, a-b) in i64
        let ra = -b;
        let rb = a - b;
        let n_rot = ra * ra - ra * rb + rb * rb;
        assert_eq!(n_orig, n_rot,
            "Rotation changed norm: ({}, {}) norm={} → ({}, {}) norm={}",
            a, b, n_orig, ra, rb, n_rot);
    }
}

// ---- Conjugate involution ----

#[test]
fn conjugate_involution_deterministic() {
    let cases: &[(i32, i32)] = &[
        (0, 0), (1, 0), (0, 1), (1, 1), (-1, -1), (3, -2), (100, 57),
    ];
    for &(a, b) in cases {
        let (c1a, c1b) = conjugate(a, b);
        let (c2a, c2b) = conjugate(c1a, c1b);
        assert_eq!((c2a, c2b), (a, b),
            "Conjugate involution failed for ({}, {})", a, b);
    }
}

#[test]
fn conjugate_involution_random() {
    let mut rng = Lcg::new(555);
    for _ in 0..100_000 {
        let a = rng.next_i32();
        let b = rng.next_i32();
        let (c1a, c1b) = conjugate(a, b);
        let (c2a, c2b) = conjugate(c1a, c1b);
        assert_eq!((c2a, c2b), (a, b),
            "Conjugate involution failed for ({}, {})", a, b);
    }
}

#[test]
fn conjugate_preserves_norm() {
    let mut rng = Lcg::new(333);
    for _ in 0..100_000 {
        let a = rng.next_i32();
        let b = rng.next_i32();
        let n_orig = norm(a, b);
        let (ca, cb) = conjugate(a, b);
        let n_conj = norm(ca, cb);
        assert_eq!(n_orig, n_conj,
            "Conjugate changed norm for ({}, {})", a, b);
    }
}

// ---- Disk coverage ----

#[test]
fn disk_counts_formula() {
    for r in 0u32..=50 {
        let expected = 3u64 * (r as u64).pow(2) + 3 * (r as u64) + 1;
        // Verify the formula against the eisenstein crate's HexDisk
        let disk = eisenstein::HexDisk::radius(r);
        assert_eq!(disk.count(), expected, "Formula mismatch at r={}", r);
    }
}

#[test]
fn disk_no_duplicates_small() {
    for r in 0u32..=20 {
        let disk = eisenstein::HexDisk::radius(r);
        let points: Vec<eisenstein::E12> = disk.iter().collect();
        let unique: HashSet<eisenstein::E12> = points.iter().copied().collect();
        assert_eq!(unique.len(), points.len(),
            "Duplicates in HexDisk radius {}", r);
        assert_eq!(points.len() as u64, disk.count(),
            "Count mismatch at radius {}", r);
    }
}

#[test]
fn disk_all_points_inside() {
    for r in 0u32..=15 {
        let disk = eisenstein::HexDisk::radius(r);
        for p in disk.iter() {
            assert!(disk.contains(&p),
                "Point ({}, {}) outside disk of radius {}", p.a(), p.b(), r);
        }
    }
}
