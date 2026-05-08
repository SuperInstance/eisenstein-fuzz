use eisenstein::E12;
use arbitrary::Arbitrary;

#[derive(Arbitrary)]
struct Input {
    a: i32,
    b: i32,
}

/// Eisenstein norm a² - ab + b² is always non-negative (over the integers).
/// This is the foundational "no negative drift" invariant.
#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_norm_nonneg(input: Input) {
    let a = input.a as i64;
    let b = input.b as i64;
    let norm = a * a - a * b + b * b;
    assert!(norm >= 0, "Norm negative for ({}, {}): {}", input.a, input.b, norm);
}
