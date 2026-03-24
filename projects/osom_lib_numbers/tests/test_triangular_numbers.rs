use osom_lib_numbers::IterTriangular;

use rstest::rstest;

const FIRST_100_TRIANGULAR_NUMBERS: &[u32] = &[
    0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351,
    378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176,
    1225, 1275, 1326, 1378, 1431, 1485, 1540, 1596, 1653, 1711, 1770, 1830, 1891, 1953, 2016, 2080, 2145, 2211, 2278,
    2346, 2415, 2485, 2556, 2628, 2701, 2775, 2850, 2926, 3003, 3081, 3160, 3240, 3321, 3403, 3486, 3570, 3655, 3741,
    3828, 3916, 4005, 4095, 4186, 4278, 4371, 4465, 4560, 4656, 4753, 4851, 4950,
];

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(10)]
#[case(15)]
#[case(50)]
#[case(51)]
#[case(98)]
#[case(99)]
#[case(100)]
fn test_triangular_numbers_empty(#[case] count: u32) {
    let iter = IterTriangular::new(count);
    assert_eq!(
        iter.collect::<Vec<_>>(),
        &FIRST_100_TRIANGULAR_NUMBERS[..count as usize]
    );
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(1 << 1)]
#[case(1 << 2)]
#[case(1 << 4)]
#[case(1 << 10)]
#[case(1 << 11)]
#[case(1 << 12)]
#[case(1 << 19)]
#[case(1 << 20)]
#[case(1 << 23)]
fn test_triangular_numbers_generate_permutation_over_powers_of_two(#[case] count: u32) {
    assert!(count == 0 || count.is_power_of_two());
    let mask = count.wrapping_sub(1);
    let mut iter = IterTriangular::new(count);
    let mut seen = vec![false; count as usize];
    while let Some(number) = iter.next() {
        let current = (number & mask) as usize;
        assert!(!seen[current]);
        seen[current] = true;
    }
}
