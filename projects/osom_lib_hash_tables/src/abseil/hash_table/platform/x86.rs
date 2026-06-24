#![allow(clippy::wildcard_imports, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_block::CONTROL_BYTE_EMPTY, abseil_layout::ABSEIL_BLOCK_SIZE, platform::{FullBlockScanResult, MatchingAndEmptyBlockScanResult, PlatformOps}, set_bit_iterator::SetBitIterator
};

/// `0x80` for `_mm_set1_epi8` (empty control byte in Abseil).
const CONTROL_EMPTY_I8: i8 = CONTROL_BYTE_EMPTY as i8;

pub struct X86PlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for X86PlatformOps {
    #[inline(always)]
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { sse2_data(control_bytes) };
        SetBitIterator::new(indexes)
    }

    #[inline(always)]
    fn full_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> FullBlockScanResult {
        let (matching, empty, tombstones) = unsafe { sse2_full_scan(control_bytes, partial_hash) };
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
        let (matching, empty) = unsafe { sse2_partial_scan(control_bytes, partial_hash) };
        MatchingAndEmptyBlockScanResult {
            matching_indexes: SetBitIterator::new(matching),
            empty_buckets: SetBitIterator::new(empty),
        }
    }

    #[inline(always)]
    fn empty_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let indexes = unsafe { sse2_empty(control_bytes) };
        SetBitIterator::new(indexes)
    }
}

/// Converts SSE byte equality / comparison masks to the same 16-bit layout as
/// `SetBitIterator` (bit *i* set ⇔ byte *i* matched).
#[inline]
unsafe fn movemask_to_u16(v: __m128i) -> u16 {
    (unsafe { _mm_movemask_epi8(v) }) as u16
}

#[target_feature(enable = "sse2")]
#[inline]
unsafe fn sse2_data(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u16 {
    unsafe {
        let ctrl = _mm_loadu_si128(control_bytes.as_ptr().cast());
        let high = _mm_and_si128(ctrl, _mm_set1_epi8(0x80u8 as i8));
        let occupied = _mm_cmpeq_epi8(high, _mm_setzero_si128());
        movemask_to_u16(occupied)
    }
}

#[target_feature(enable = "sse2")]
#[inline]
unsafe fn sse2_empty(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u16 {
    unsafe {
        let ctrl = _mm_loadu_si128(control_bytes.as_ptr().cast());
        let is_empty = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(CONTROL_EMPTY_I8));
        movemask_to_u16(is_empty)
    }
}

#[target_feature(enable = "sse2")]
#[inline]
unsafe fn sse2_full_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> (u16, u16, u16) {
    unsafe {
        let ctrl = _mm_loadu_si128(control_bytes.as_ptr().cast());
        let matching_hash = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(partial_hash as i8));
        let is_empty = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(CONTROL_EMPTY_I8));
        let is_tombstone = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(-1i8));
        (
            movemask_to_u16(matching_hash),
            movemask_to_u16(is_empty),
            movemask_to_u16(is_tombstone),
        )
    }
}

#[target_feature(enable = "sse2")]
#[inline]
unsafe fn sse2_partial_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> (u16, u16) {
    unsafe {
        let ctrl = _mm_loadu_si128(control_bytes.as_ptr().cast());
        let matching_hash = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(partial_hash as i8));
        let is_empty = _mm_cmpeq_epi8(ctrl, _mm_set1_epi8(CONTROL_EMPTY_I8));
        (movemask_to_u16(matching_hash), movemask_to_u16(is_empty))
    }
}
