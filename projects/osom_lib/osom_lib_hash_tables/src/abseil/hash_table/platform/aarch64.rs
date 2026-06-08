#![allow(clippy::wildcard_imports)]

use core::arch::aarch64::*;
use core::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_layout::ABSEIL_BLOCK_SIZE,
    platform::{FullBlockScanResult, MatchingAndEmptyBlockScanResult, PlatformOps},
    set_bit_iterator::SetBitIterator,
};

pub struct Aarch64PlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for Aarch64PlatformOps {
    #[inline(always)]
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { neon_data(control_bytes) };
        SetBitIterator::new(indexes)
    }

    #[inline(always)]
    fn full_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> FullBlockScanResult {
        let (matching, empty, tombstones) = unsafe { neon_full_scan(control_bytes, partial_hash) };
        FullBlockScanResult {
            matching_indexes: SetBitIterator::new(matching),
            empty_buckets: SetBitIterator::new(empty),
            tombstones: SetBitIterator::new(tombstones),
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    fn empty_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { neon_empty(control_bytes) };
        SetBitIterator::new(indexes)
    }
}

macro_rules! build_bitmask {
    () => {{
        let result;
        #[allow(unused_unsafe)]
        unsafe {
            let bit_mask_lo = vcreate_u8(0x8040_2010_0804_0201_u64);
            result = vcombine_u8(bit_mask_lo, bit_mask_lo);
        }
        result
    }};
}

macro_rules! extract_bitmask {
    ( $mask: expr, $bit_mask: expr ) => {
        #[allow(unused_unsafe)]
        unsafe {
            let masked = vandq_u8($mask, $bit_mask);
            let lo = u16::from(vaddv_u8(vget_low_u8(masked)));
            let hi = u16::from(vaddv_u8(vget_high_u8(masked)));
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
        let bit_mask = build_bitmask!();
        extract_bitmask!(occupied, bit_mask)
    }
}

#[target_feature(enable = "neon")]
#[inline]
unsafe fn neon_empty(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u16 {
    unsafe {
        let ctrl = vld1q_u8(control_bytes.as_ptr());
        let is_empty = vceqq_u8(ctrl, vdupq_n_u8(0x80));
        let bit_mask = build_bitmask!();
        extract_bitmask!(is_empty, bit_mask)
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
        let bit_mask = build_bitmask!();
        (
            extract_bitmask!(matching_hash, bit_mask),
            extract_bitmask!(is_empty, bit_mask),
            extract_bitmask!(is_tombstone, bit_mask),
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
        let bit_mask = build_bitmask!();
        (
            extract_bitmask!(matching_hash, bit_mask),
            extract_bitmask!(is_empty, bit_mask),
        )
    }
}
