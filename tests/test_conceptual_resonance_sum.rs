//\! Comprehensive tests for the "Conceptual Resonance Sum" logic.
//\!
//\! Testing library/framework: Rust's built-in test harness (libtest) using #[test] and assert_* macros.
//\! No external test dependencies are introduced.

fn compute_term(count: u64, prime: u32) -> u64 {
    // Pure function: raises count to the given prime exponent.
    count.pow(prime)
}

fn conceptual_resonance_sum_from_pairs(pairs: &[(u64, u32)]) -> u64 {
    pairs.iter().map(|&(c, p)| compute_term(c, p)).sum()
}

// Constants derived from the provided diff/file contents.
const COUNTS_AND_PRIMES: [(u64, u32); 7] = [
    (5, 3),   // COUNT_P3 ^ 3
    (11, 5),  // COUNT_P5 ^ 5
    (19, 7),  // COUNT_P7 ^ 7
    (5, 11),  // COUNT_P11 ^ 11
    (10, 13), // COUNT_P13 ^ 13
    (2, 17),  // COUNT_P17 ^ 17
    (3, 19),  // COUNT_P19 ^ 19
];

// Expected individual term values (u64-safe).
const EXPECTED_TERMS: [u64; 7] = [
    125,              // 5^3
    161_051,          // 11^5
    893_871_739,      // 19^7
    48_828_125,       // 5^11
    10_000_000_000_000, // 10^13
    131_072,          // 2^17
    1_162_261_467,    // 3^19
];

// Expected final sum of all terms.
const EXPECTED_SUM: u64 = 10_002_105_253_579;

#[test]
fn terms_match_expected_values() {
    for ((count, prime), expected) in COUNTS_AND_PRIMES.iter().zip(EXPECTED_TERMS.iter()) {
        let actual = compute_term(*count, *prime);
        assert_eq\!(actual, *expected, "Term {}^{} mismatch", count, prime);
    }
}

#[test]
fn sum_matches_expected_value() {
    let sum = conceptual_resonance_sum_from_pairs(&COUNTS_AND_PRIMES);
    assert_eq\!(sum, EXPECTED_SUM, "Conceptual Resonance Sum mismatch");
}

#[test]
fn sum_is_order_independent() {
    // Shuffled order of the same (count, prime) pairs
    const SHUFFLED: [(u64, u32); 7] = [
        (10, 13),
        (3, 19),
        (5, 11),
        (19, 7),
        (2, 17),
        (5, 3),
        (11, 5),
    ];
    let sum_original = conceptual_resonance_sum_from_pairs(&COUNTS_AND_PRIMES);
    let sum_shuffled = conceptual_resonance_sum_from_pairs(&SHUFFLED);
    assert_eq\!(sum_original, sum_shuffled, "Sum should be independent of term order");
    assert_eq\!(sum_original, EXPECTED_SUM, "Sum should equal the expected constant");
}

#[test]
fn zero_counts_yield_zero_sum() {
    let zeros: [(u64, u32); 7] = [(0, 3), (0, 5), (0, 7), (0, 11), (0, 13), (0, 17), (0, 19)];
    let sum = conceptual_resonance_sum_from_pairs(&zeros);
    assert_eq\!(sum, 0, "All-zero counts should yield a zero sum");
}

#[test]
fn ones_counts_yield_count_of_terms() {
    let ones: [(u64, u32); 7] = [(1, 3), (1, 5), (1, 7), (1, 11), (1, 13), (1, 17), (1, 19)];
    let sum = conceptual_resonance_sum_from_pairs(&ones);
    assert_eq\!(sum, ones.len() as u64, "1^p == 1; sum should equal number of terms");
}

#[test]
fn modifying_one_count_changes_sum_by_expected_delta() {
    // Change only the last pair (3, 19) -> (4, 19)
    let mut modified = COUNTS_AND_PRIMES;
    modified[6] = (4, 19);

    let base = conceptual_resonance_sum_from_pairs(&COUNTS_AND_PRIMES);
    let changed = conceptual_resonance_sum_from_pairs(&modified);

    let delta_expected = compute_term(4, 19) - compute_term(3, 19);
    assert_eq\!(changed - base, delta_expected, "Delta should equal 4^19 - 3^19");
}

#[test]
fn compute_term_is_monotonic_in_count_for_positive_exponent_small_domain() {
    let p: u32 = 3;
    for c in 0u64..20 {
        let a = compute_term(c, p);
        let b = compute_term(c + 1, p);
        assert\!(b > a, "Monotonicity violated at c = {} for exponent {}", c, p);
    }
}