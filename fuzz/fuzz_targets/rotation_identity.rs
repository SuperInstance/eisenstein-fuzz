use eisenstein::E12;
use arbitrary::Arbitrary;

#[derive(Arbitrary)]
struct Input {
    a: i32,
    b: i32,
}

/// Rotating an Eisenstein integer 6 times via (a,b) → (-b, a+b) returns the original.
/// This is the D6 rotational symmetry of the hexagonal lattice.
fn rotate(p: E12) -> E12 {
    E12::new(-p.b(), p.a() - p.b())
}

#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_rotation_identity(input: Input) {
    let original = E12::new(input.a, input.b);
    let mut p = original;
    for _ in 0..6 {
        p = rotate(p);
    }
    assert_eq!(p, original, "6-fold rotation identity failed for ({}, {})", input.a, input.b);
}
