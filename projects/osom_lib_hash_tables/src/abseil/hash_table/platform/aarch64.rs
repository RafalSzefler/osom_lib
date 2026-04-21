#![allow(clippy::wildcard_imports)]

use std::arch::aarch64::*;
use std::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_layout::ABSEIL_BLOCK_SIZE,
    platform::{FullBlockScanResult, MatchingAndEmptyBlockScanResult, PlatformOps},
    set_bit_iterator::SetBitIterator,
};

pub struct Aarch64PlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for Aarch64PlatformOps {
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { neon_data(control_bytes) };
        SetBitIterator::new(indexes)
    }

    fn full_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> FullBlockScanResult {
        let (matching, empty, tombstones) = unsafe { neon_full_scan(control_bytes, partial_hash) };
        FullBlockScanResult {
            matching_indexes: SetBitIterator::new(matching),
            empty_buckets: SetBitIterator::new(empty),
            tombstones: SetBitIterator::new(tombstones),
        }
    }

    fn matching_block_scan(
        control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        partial_hash: u8,
    ) -> MatchingAndEmptyBlockScanResult {
        let (matching, empty) = unsafe { neon_partial_scan(control_bytes, partial_hash) };
        MatchingAndEmptyBlockScanResult {
            matching_indexes: SetBitIterator::new(matching),
            empty_buckets: SetBitIterator::new(empty),
        }
    }
}

#[inline(always)]
fn build_bitmask() -> uint8x16_t {
    unsafe {
        let bit_mask_lo = vcreate_u8(0x8040_2010_0804_0201_u64);
        let bit_mask_hi = vcreate_u8(0x8040_2010_0804_0201_u64);
        vcombine_u8(bit_mask_lo, bit_mask_hi)
    }
}

macro_rules! extract_bitmask {
    ( $mask: expr ) => {
        #[allow(unused_unsafe)]
        unsafe {
            let bit_mask = build_bitmask();
            let masked = vandq_u8($mask, bit_mask);
            let lo = vaddv_u8(vget_low_u8(masked)) as u16;
            let hi = vaddv_u8(vget_high_u8(masked)) as u16;
            lo | (hi << 8)
        }
    };
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_data(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u16 {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let occupied = vcltq_u8(ctrl, vdupq_n_u8(0x80));
        extract_bitmask!(occupied)
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_full_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> (u16, u16, u16) {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let matching_hash = vceqq_u8(ctrl, vdupq_n_u8(partial_hash));
        let is_empty = vceqq_u8(ctrl, vdupq_n_u8(0x80));
        let is_tombstone = vceqq_u8(ctrl, vdupq_n_u8(0xff));
        (
            extract_bitmask!(matching_hash),
            extract_bitmask!(is_empty),
            extract_bitmask!(is_tombstone),
        )
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_partial_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> (u16, u16) {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let matching_hash = vceqq_u8(ctrl, vdupq_n_u8(partial_hash));
        let is_empty = vceqq_u8(ctrl, vdupq_n_u8(0x80));
        (extract_bitmask!(matching_hash), extract_bitmask!(is_empty))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

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
