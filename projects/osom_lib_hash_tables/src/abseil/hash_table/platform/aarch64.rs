#![allow(clippy::wildcard_imports)]

use std::arch::aarch64::*;
use std::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_layout::ABSEIL_BLOCK_SIZE,
    platform::{PlatformOps, ScanBlockResult},
    set_bit_iterator::SetBitIterator,
};

pub struct Aarch64PlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for Aarch64PlatformOps {
    fn iter_matching_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> SetBitIterator {
        let indexes = unsafe { neon_matching(control_bytes, partial_hash) };
        SetBitIterator::new(indexes)
    }

    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { neon_data(control_bytes) };
        SetBitIterator::new(indexes)
    }

    fn scan_block(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> ScanBlockResult {
        let (matching, empty, tombstones) = unsafe { neon_scan(control_bytes, partial_hash) };
        ScanBlockResult {
            matching_indexes: SetBitIterator::new(matching),
            empty_buckets: SetBitIterator::new(empty),
            tombstones: SetBitIterator::new(tombstones),
        }
    }
}

/// Extracts a 16-bit bitmask from a uint8x16_t comparison result (0xFF/0x00 per lane).
/// Bit i of the result is set iff lane i of `mask` is 0xFF.
#[target_feature(enable = "neon")]
#[inline]
unsafe fn extract_bitmask(mask: uint8x16_t) -> u16 {
    // AND each lane with its bit-position power-of-two: 0xFF→power, 0x00→0.
    // Summing the lower/upper halves then yields the bitmask for lanes 0-7 / 8-15.
    unsafe {
        const BIT_MASK: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let bit_mask = vld1q_u8(BIT_MASK.as_ptr());
        let masked = vandq_u8(mask, bit_mask);
        let lo = vaddv_u8(vget_low_u8(masked)) as u16;
        let hi = vaddv_u8(vget_high_u8(masked)) as u16;
        lo | (hi << 8)
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_matching(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> u16 {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let matching_hash = vceqq_u8(ctrl, vdupq_n_u8(partial_hash));
        extract_bitmask(matching_hash)
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_data(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u16 {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let occupied = vcltq_u8(ctrl, vdupq_n_u8(0x80));
        extract_bitmask(occupied)
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> (u16, u16, u16) {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let matching_hash = vceqq_u8(ctrl, vdupq_n_u8(partial_hash));
        let is_empty = vceqq_u8(ctrl, vdupq_n_u8(0x80));
        let is_tombstone = vceqq_u8(ctrl, vdupq_n_u8(0xff));
        (
            extract_bitmask(matching_hash),
            extract_bitmask(is_empty),
            extract_bitmask(is_tombstone),
        )
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1, &[])]
    #[case(&[0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 15, &[2])]
    #[case(&[0, 0, 15, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 15], 15, &[2, 15])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 156], 156, &[15])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0], 0, &[0, 1, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15])]
    fn test_aarch64_iter_matching_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] partial_hash: u8,
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = Aarch64PlatformOps::iter_matching_indexes(control_bytes, partial_hash).collect();
        assert_eq!(result, expected_indexes);
    }

    #[rstest]
    #[case(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], &[])]
    #[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
    fn test_aarch64_iter_data_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = Aarch64PlatformOps::iter_data_indexes(control_bytes).collect();
        assert_eq!(result, expected_indexes);
    }
}
