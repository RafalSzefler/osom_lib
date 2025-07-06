use osom_lib_rand::number::Number;

pub fn test_fill_bytes<T: Number, G: FnMut(&mut [u8])>(mut generator: G) {
    const BINS: usize = u8::MAX as usize + 1;
    const BYTES_SIZE: usize = 100000;
    const EXPECTED_PER_BIN: f64 = BYTES_SIZE as f64 / BINS as f64;

    let mut bytes = [0u8; BYTES_SIZE];
    generator(&mut bytes);

    let mut bins = [0u32; BINS];

    // Generate random numbers and count them into bins
    for i in 0..BYTES_SIZE {
        let value = bytes[i];
        bins[value as usize] += 1;
    }

    for count in bins {
        assert!(count > (EXPECTED_PER_BIN * 0.8) as u32);
        assert!(count < (EXPECTED_PER_BIN * 1.2) as u32);
    }
}

pub fn test_statistical_properties<T: Number, G: FnMut() -> T>(mut generator: G) {
    test_averages(&mut generator);
    test_chi_square(&mut generator);
}

fn test_averages<T: Number, G: FnMut() -> T>(generator: &mut G) {
    const ITERATIONS: usize = 100000;
    let mut sum = 0u128;
    let mut expected_sum = 0u128;

    for _ in 0..ITERATIONS {
        // We convert to u32 to avoid overflows.
        let value = (generator().as_u128() as u32) as u128;
        sum += value;
        expected_sum += (u32::MAX as u128) / 2;
    }

    // It is extremely unlikely to generate exactly the same sum.
    assert_ne!(sum, expected_sum);

    let quotient = sum as f64 / expected_sum as f64;
    assert!(quotient > 0.95);
    assert!(quotient < 1.05);
}

fn test_chi_square<T: Number, G: FnMut() -> T>(generator: &mut G) {
    const BINS: usize = 100;
    const ITERATIONS: usize = 1000000;
    const EXPECTED_PER_BIN: f64 = ITERATIONS as f64 / BINS as f64;
    const CHI_SQUARE_THRESHOLD: f64 = 135.0; // ~91.2% confidence level for 99 degrees of freedom

    let mut bins = [0u64; BINS];

    // Generate random numbers and count them into bins
    for _ in 0..ITERATIONS {
        let value = generator().as_u128();
        let bin_index = (value % BINS as u128) as usize;
        bins[bin_index] += 1;
    }

    // Calculate chi-square statistic
    let mut chi_square = 0.0;
    for observed in &bins {
        let observed_f = *observed as f64;
        let diff = observed_f - EXPECTED_PER_BIN;
        chi_square += (diff * diff) / EXPECTED_PER_BIN;
    }

    // For a good random generator, chi-square should be below the threshold
    // (indicating uniform distribution across bins)
    assert!(
        chi_square < CHI_SQUARE_THRESHOLD,
        "Chi-square test failed: {} >= {} (expected uniform distribution)",
        chi_square,
        CHI_SQUARE_THRESHOLD
    );
}
