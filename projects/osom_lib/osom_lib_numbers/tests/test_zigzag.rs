use rstest::rstest;

use osom_lib_numbers::zigzag;

#[rstest]
#[case(0, 0)]
#[case(1, 2)]
#[case(-1, 1)]
#[case(2, 4)]
#[case(-2, 3)]
#[case(3, 6)]
#[case(-3, 5)]
#[case(4, 8)]
#[case(-4, 7)]
#[case(5, 10)]
#[case(-5, 9)]
#[case(i32::MIN, u32::MAX)]
#[case(i32::MAX, u32::MAX-1)]
#[case(i32::MIN+1, u32::MAX-2)]
fn test_zigzag32(#[case] value: i32, #[case] expected: u32) {
    assert_eq!(zigzag::zigzag_encode32(value), expected);
    assert_eq!(zigzag::zigzag_decode32(expected), value);
}

#[rstest]
#[case(0, 0)]
#[case(1, 2)]
#[case(-1, 1)]
#[case(2, 4)]
#[case(-2, 3)]
#[case(3, 6)]
#[case(-3, 5)]
#[case(4, 8)]
#[case(-4, 7)]
#[case(5, 10)]
#[case(-5, 9)]
#[case(i64::MIN, u64::MAX)]
#[case(i64::MAX, u64::MAX-1)]
#[case(i64::MIN+1, u64::MAX-2)]
fn test_zigzag64(#[case] value: i64, #[case] expected: u64) {
    assert_eq!(zigzag::zigzag_encode64(value), expected);
    assert_eq!(zigzag::zigzag_decode64(expected), value);
}

#[rstest]
#[case(0, 0)]
#[case(1, 2)]
#[case(-1, 1)]
#[case(2, 4)]
#[case(-2, 3)]
#[case(3, 6)]
#[case(-3, 5)]
#[case(4, 8)]
#[case(-4, 7)]
#[case(5, 10)]
#[case(-5, 9)]
#[case(i128::MIN, u128::MAX)]
#[case(i128::MAX, u128::MAX-1)]
#[case(i128::MIN+1, u128::MAX-2)]
fn test_zigzag128(#[case] value: i128, #[case] expected: u128) {
    assert_eq!(zigzag::zigzag_encode128(value), expected);
    assert_eq!(zigzag::zigzag_decode128(expected), value);
}
