use eisenstein::E12;
use arbitrary::Arbitrary;

#[derive(Arbitrary)]
struct Input {
    a: i32,
    b: i32,
}

/// Conjugate is an involution: conj(conj(p)) == p.
#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_conjugate_involution(input: Input) {
    let p = E12::new(input.a, input.b);
    let conj1 = p.conjugate();
    let conj2 = conj1.conjugate();
    assert_eq!(conj2, p,
        "Conjugate involution failed for ({}, {}): got ({}, {})",
        input.a, input.b, conj2.a(), conj2.b());

    // Norm is preserved under conjugation
    assert_eq!(p.norm(), conj1.norm(),
        "Conjugate changed norm for ({}, {}): {} -> {}",
        input.a, input.b, p.norm(), conj1.norm());
}
