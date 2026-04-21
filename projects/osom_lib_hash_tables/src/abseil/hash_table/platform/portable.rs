use std::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_block::{CONTROL_BYTE_EMPTY, CONTROL_BYTE_TOMBSTONE},
    abseil_layout::ABSEIL_BLOCK_SIZE,
    platform::{PlatformOps, ScanBlockResult},
    set_bit_iterator::SetBitIterator,
};

pub struct PortablePlatformOps {
    _priv: PhantomData<()>,
}

impl PlatformOps for PortablePlatformOps {
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let mut indexes = 0u16;
        for (idx, value) in control_bytes.iter().enumerate() {
            if *value & 0x80 == 0 {
                indexes |= 1 << idx;
            }
        }
        SetBitIterator::new(indexes)
    }

    fn scan_block(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> ScanBlockResult {
        let mut matching_indexes = 0u16;
        let mut empty_indexes = 0u16;
        let mut tombstone_indexes = 0u16;

        for (idx, value) in control_bytes.iter().enumerate() {
            let value = *value;
            let bit = 1 << idx;
            if value < 0x80 {
                if value == partial_hash {
                    matching_indexes |= bit;
                }
            } else {
                match value {
                    CONTROL_BYTE_EMPTY => {
                        empty_indexes |= bit;
                    }
                    CONTROL_BYTE_TOMBSTONE => {
                        tombstone_indexes |= bit;
                    }
                    _ => unreachable!("There are only two possible values for control byte above or equal to 0x80."),
                }
            }
        }

        ScanBlockResult {
            matching_indexes: SetBitIterator::new(matching_indexes),
            empty_buckets: SetBitIterator::new(empty_indexes),
            tombstones: SetBitIterator::new(tombstone_indexes),
        }
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
    fn test_portable_iter_data_indexes(
        #[case] control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        #[case] expected_indexes: &[usize],
    ) {
        let result: Vec<usize> = PortablePlatformOps::iter_data_indexes(control_bytes).collect();
        assert_eq!(result, expected_indexes);
    }
}
