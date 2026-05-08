use eisenstein::E12;
use eisenstein::HexDisk;
use arbitrary::Arbitrary;
use std::collections::HashSet;

#[derive(Arbitrary)]
struct Input {
    radius: u8, // 1..=100
}

/// HexDisk coverage: correct count, no duplicates, all points within bounds.
#[cfg_attr(all(test, fuzz), libfuzzer_sys::fuzz_test)]
fn fuzz_disk_coverage(input: Input) {
    let r = (input.radius as u32).max(1).min(100);
    let disk = HexDisk::radius(r);
    let expected_count = 3u64 * (r as u64) * (r as u64) + 3 * (r as u64) + 1;

    let points: Vec<E12> = disk.iter().collect();

    // Correct count
    assert_eq!(points.len() as u64, expected_count,
        "HexDisk radius {} count mismatch: got {}, expected {}",
        r, points.len(), expected_count);

    // No duplicates
    let unique: HashSet<E12> = points.iter().copied().collect();
    assert_eq!(unique.len(), points.len(),
        "HexDisk radius {} has duplicates", r);

    // All points are inside the disk
    for p in &points {
        assert!(disk.contains(p),
            "Point ({}, {}) outside disk of radius {}", p.a(), p.b(), r);
    }
}
