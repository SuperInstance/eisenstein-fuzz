use eisenstein::E12;
use arbitrary::Arbitrary;

#[derive(Arbitrary)]
struct Input {
    a1: i16,
    b1: i16,
    a2: i16,
    b2: i16,
    a3: i16,
    b3: i16,
}

/// Ring axioms: associativity of addition, commutativity, distributivity.
/// Using i16 to avoid overflow in multiplication.
#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_ring_axioms(input: Input) {
    let a = E12::new(input.a1 as i32, input.b1 as i32);
    let b = E12::new(input.a2 as i32, input.b2 as i32);
    let c = E12::new(input.a3 as i32, input.b3 as i32);

    // Associativity of addition: (a + b) + c == a + (b + c)
    assert_eq!((a + b) + c, a + (b + c),
        "Addition associativity failed");

    // Commutativity of addition: a + b == b + a
    assert_eq!(a + b, b + a, "Addition commutativity failed");

    // Commutativity of multiplication: a * b == b * a
    assert_eq!(a * b, b * a, "Multiplication commutativity failed");

    // Distributivity: a * (b + c) == a*b + a*c
    assert_eq!(a * (b + c), a * b + a * c,
        "Distributivity failed for ({},{})*(({},{})+({},{}))",
        input.a1, input.b1, input.a2, input.b2, input.a3, input.b3);

    // Associativity of multiplication: (a * b) * c == a * (b * c)
    assert_eq!((a * b) * c, a * (b * c),
        "Multiplication associativity failed");
}
