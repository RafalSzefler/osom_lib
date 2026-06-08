#![cfg(all(test, feature = "std"))]

use rstest::rstest;

use super::*;

const EMPTY: u8 = 0x80;
const TOMBSTONE: u8 = 0xff;

#[rstest]
#[case(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], &[])]
#[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
#[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
fn test_platform_impl_iter_data_indexes(
    #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
    #[case] expected_indexes: &[usize],
) {
    let result: Vec<usize> = PlatformImpl::iter_data_indexes(control_bytes).collect();
    assert_eq!(result, expected_indexes);
}

#[rstest]
#[case(
    &[1, 2, EMPTY, TOMBSTONE, 2, 9, EMPTY, TOMBSTONE, 2, 6, 2, EMPTY, TOMBSTONE, 7, 8, EMPTY],
    2,
    &[1, 4, 8, 10],
    &[2, 6, 11, 15],
    &[3, 7, 12]
)]
#[case(
    &[EMPTY, TOMBSTONE, EMPTY, TOMBSTONE, 5, 6, 7, 8, 9, 10, EMPTY, TOMBSTONE, 11, 12, 13, 14],
    42,
    &[],
    &[0, 2, 10],
    &[1, 3, 11]
)]
fn test_platform_impl_full_block_scan(
    #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
    #[case] partial_hash: u8,
    #[case] expected_matching_indexes: &[usize],
    #[case] expected_empty_indexes: &[usize],
    #[case] expected_tombstone_indexes: &[usize],
) {
    let result = PlatformImpl::full_block_scan(control_bytes, partial_hash);
    let matching: Vec<usize> = result.matching_indexes.collect();
    let empty: Vec<usize> = result.empty_buckets.collect();
    let tombstones: Vec<usize> = result.tombstones.collect();

    assert_eq!(matching, expected_matching_indexes);
    assert_eq!(empty, expected_empty_indexes);
    assert_eq!(tombstones, expected_tombstone_indexes);
}

#[rstest]
#[case(
    &[1, 5, EMPTY, TOMBSTONE, 5, 8, EMPTY, TOMBSTONE, 5, 1, 2, EMPTY, TOMBSTONE, 3, 4, EMPTY],
    5,
    &[1, 4, 8],
    &[2, 6, 11, 15]
)]
#[case(
    &[TOMBSTONE, TOMBSTONE, EMPTY, EMPTY, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    99,
    &[],
    &[2, 3]
)]
fn test_platform_impl_matching_block_scan(
    #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
    #[case] partial_hash: u8,
    #[case] expected_matching_indexes: &[usize],
    #[case] expected_empty_indexes: &[usize],
) {
    let result = PlatformImpl::matching_block_scan(control_bytes, partial_hash);
    let matching: Vec<usize> = result.matching_indexes.collect();
    let empty: Vec<usize> = result.empty_buckets.collect();

    assert_eq!(matching, expected_matching_indexes);
    assert_eq!(empty, expected_empty_indexes);
}

#[rstest]
#[case(
    &[EMPTY, 1, 2, TOMBSTONE, EMPTY, 5, 6, 7, EMPTY, 9, TOMBSTONE, 11, 12, EMPTY, 14, 15],
    &[0, 4, 8, 13]
)]
#[case(
    &[TOMBSTONE, TOMBSTONE, TOMBSTONE, TOMBSTONE, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    &[]
)]
fn test_platform_impl_empty_scan(
    #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
    #[case] expected_empty_indexes: &[usize],
) {
    let empty_indexes: Vec<usize> = PlatformImpl::empty_scan(control_bytes).collect();
    assert_eq!(empty_indexes, expected_empty_indexes);
}
