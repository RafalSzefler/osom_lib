use std::arch::aarch64::*;
use std::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_layout::ABSEIL_BLOCK_SIZE, platform::PlatformOps, set_bit_iterator::SetBitIterator,
};

pub struct Aarch64PlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for Aarch64PlatformOps {
    #[inline(always)]
    fn iter_matching_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> SetBitIterator {
        // Safety: aarch64 always has NEON; control_bytes has exactly ABSEIL_BLOCK_SIZE bytes
        let indexes = unsafe { neon_matching(control_bytes, partial_hash) };
        SetBitIterator::new(indexes.reverse_bits())
    }

    #[inline(always)]
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        // Safety: aarch64 always has NEON; control_bytes has exactly ABSEIL_BLOCK_SIZE bytes
        let indexes = unsafe { neon_data(control_bytes) };
        SetBitIterator::new(indexes.reverse_bits())
    }
}

/// Extracts a 16-bit bitmask from a uint8x16_t comparison result (0xFF/0x00 per lane).
/// Bit i of the result is set iff lane i of `mask` is 0xFF.
///
/// # Safety
/// Caller must ensure NEON is available (always true on aarch64).
#[target_feature(enable = "neon")]
#[inline]
unsafe fn extract_bitmask(mask: uint8x16_t) -> u32 {
    // Each byte position gets a unique power-of-2 bit selector within its 8-byte group
    const BIT_SELECTOR: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

    unsafe {
        let sel = vld1q_u8(BIT_SELECTOR.as_ptr());

        // AND mask (0xFF or 0x00) with bit selectors: each lane now holds 0 or its bit value
        let bits = vandq_u8(mask, sel);

        // Sum each 8-byte group via three rounds of pairwise addition
        let low = vget_low_u8(bits);
        let high = vget_high_u8(bits);
        // Round 1: [p0+p1, p2+p3, p4+p5, p6+p7, p8+p9, p10+p11, p12+p13, p14+p15]
        let sum = vpadd_u8(low, high);
        // Round 2: [p0..p3, p4..p7, p8..p11, p12..p15, ...]
        let sum = vpadd_u8(sum, sum);
        // Round 3: [p0..p7, p8..p15, ...]
        let sum = vpadd_u8(sum, sum);

        let lo = vget_lane_u8::<0>(sum) as u32;
        let hi = vget_lane_u8::<1>(sum) as u32;
        lo | (hi << 8)
    }
}

/// # Safety
/// Caller must ensure NEON is available (always true on aarch64).
#[target_feature(enable = "neon")]
unsafe fn neon_matching(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> u32 {
    unsafe {
        let ptr = control_bytes.as_ptr();
        let v0 = vld1q_u8(ptr);
        let v1 = vld1q_u8(ptr.add(16));
        let needle = vdupq_n_u8(partial_hash);
        let mask0 = extract_bitmask(vceqq_u8(v0, needle));
        let mask1 = extract_bitmask(vceqq_u8(v1, needle));
        mask0 | (mask1 << 16)
    }
}

/// # Safety
/// Caller must ensure NEON is available (always true on aarch64).
#[target_feature(enable = "neon")]
unsafe fn neon_data(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> u32 {
    unsafe {
        let ptr = control_bytes.as_ptr();
        let v0 = vld1q_u8(ptr);
        let v1 = vld1q_u8(ptr.add(16));
        // Valid bytes have bit 7 (MSB) clear; AND with 0x80 and compare to 0
        let high_bit = vdupq_n_u8(0x80);
        let zero = vdupq_n_u8(0);
        let mask0 = extract_bitmask(vceqq_u8(vandq_u8(v0, high_bit), zero));
        let mask1 = extract_bitmask(vceqq_u8(vandq_u8(v1, high_bit), zero));
        mask0 | (mask1 << 16)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1, &[])]
    #[case(&[0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 15, &[2])]
    #[case(&[0, 0, 15, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 156], 15, &[2, 25])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 13, 0, 156], 156, &[31])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 13, 0, 156], 0, &[0, 1, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 30])]
    fn test_aarch64_iter_matching_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] partial_hash: u8,
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = Aarch64PlatformOps::iter_matching_indexes(control_bytes, partial_hash).collect();
        assert_eq!(result, expected_indexes);
    }

    #[rstest]
    #[case(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], &[])]
    #[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 13, 0, 156], &[0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30])]
    fn test_aarch64_iter_data_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = Aarch64PlatformOps::iter_data_indexes(control_bytes).collect();
        assert_eq!(result, expected_indexes);
    }
}
