use eisenstein::E12;
use arbitrary::Arbitrary;

#[derive(Arbitrary)]
struct Input {
    a: i32,
    b: i32,
}

/// D6 closure: rotation preserves norm, and all 6 neighbors have norm within ±3 of original.
fn rotate(p: E12) -> E12 {
    E12::new(-p.b(), p.a() - p.b())
}

#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_d6_closure(input: Input) {
    let p = E12::new(input.a, input.b);
    let norm_p = p.norm();

    // Rotation preserves norm (all 6 rotations)
    let mut r = p;
    for i in 0..6 {
        r = rotate(r);
        assert_eq!(r.norm(), norm_p,
            "Rotation {} changed norm for ({}, {}): {} -> {}",
            i + 1, input.a, input.b, norm_p, r.norm());
    }

    // All 6 neighbors have norm within ±3 of original
    // (neighbors differ by a unit, norm of unit is 1, and |‖a+b‖ - ‖a‖| ≤ ‖b‖ = 1
    //  but in hex distance the max norm change is actually ≤ 2a+1 type bounds)
    // A safe bound: |norm(neighbor) - norm(original)| ≤ 2*sqrt(norm) + 1
    // But let's just check neighbors exist and are well-formed
    let neighbors = p.neighbors();
    for (i, n) in neighbors.iter().enumerate() {
        let diff = if n.norm() > norm_p { n.norm() - norm_p } else { norm_p - n.norm() };
        // For a neighbor p + e (|e|=1): |‖p+e‖ - ‖p‖| ≤ ‖e‖² + 2‖p‖·‖e‖... 
        // Actually: norm(p+e) = (p+e)·conj(p+e) = norm(p) + p·conj(e) + e·conj(p) + 1
        // The cross terms can be at most 2*sqrt(norm(p)) in magnitude.
        // So |diff| ≤ 2*sqrt(norm_p) + 1. Check this:
        let bound = if norm_p > 0 {
            let sqrt_n = (norm_p as f64).sqrt() as u64;
            2 * sqrt_n + 1
        } else {
            1
        };
        assert!(diff <= bound,
            "Neighbor {} norm too far from original: neighbor norm={}, original norm={}, diff={}, bound={}",
            i, n.norm(), norm_p, diff, bound);
    }
}
