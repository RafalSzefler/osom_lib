use std::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_layout::ABSEIL_BLOCK_SIZE, platform::PlatformOps, set_bit_iterator::SetBitIterator,
};

pub struct PortablePlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for PortablePlatformOps {
    fn iter_matching_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> SetBitIterator {
        let mut indexes: u32 = 0;
        for (idx, value) in control_bytes.iter().enumerate() {
            if *value == partial_hash {
                indexes |= 1 << idx;
            }
        }
        SetBitIterator::new(indexes.reverse_bits())
    }

    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let mut indexes: u32 = 0;
        for (idx, value) in control_bytes.iter().enumerate() {
            if *value & 0x80 == 0 {
                indexes |= 1 << idx;
            }
        }
        SetBitIterator::new(indexes.reverse_bits())
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
    fn test_portable_iter_matching_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] partial_hash: u8,
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = PortablePlatformOps::iter_matching_indexes(control_bytes, partial_hash).collect();
        assert_eq!(result, expected_indexes);
    }

    #[rstest]
    #[case(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], &[])]
    #[case(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31])]
    #[case(&[0, 0, 15, 0, 255, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 13, 0, 156], &[0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30])]
    fn test_portable_iter_data_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = PortablePlatformOps::iter_data_indexes(control_bytes).collect();
        assert_eq!(result, expected_indexes);
    }
}
