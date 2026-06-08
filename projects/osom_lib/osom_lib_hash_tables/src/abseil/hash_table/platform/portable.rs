use core::marker::PhantomData;

use crate::abseil::hash_table::{
    abseil_block::{CONTROL_BYTE_EMPTY, CONTROL_BYTE_TOMBSTONE},
    abseil_layout::ABSEIL_BLOCK_SIZE,
    platform::{FullBlockScanResult, MatchingAndEmptyBlockScanResult, PlatformOps},
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

    fn full_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> FullBlockScanResult {
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

        FullBlockScanResult {
            matching_indexes: SetBitIterator::new(matching_indexes),
            empty_buckets: SetBitIterator::new(empty_indexes),
            tombstones: SetBitIterator::new(tombstone_indexes),
        }
    }

    fn matching_block_scan(
        control_bytes: &[u8; ABSEIL_BLOCK_SIZE],
        partial_hash: u8,
    ) -> MatchingAndEmptyBlockScanResult {
        let mut matching_indexes = 0u16;
        let mut empty_indexes = 0u16;

        for (idx, value) in control_bytes.iter().enumerate() {
            let value = *value;
            let bit = 1 << idx;
            if value < 0x80 {
                if value == partial_hash {
                    matching_indexes |= bit;
                }
            } else if value == CONTROL_BYTE_EMPTY {
                empty_indexes |= bit;
            }
        }

        MatchingAndEmptyBlockScanResult {
            matching_indexes: SetBitIterator::new(matching_indexes),
            empty_buckets: SetBitIterator::new(empty_indexes),
        }
    }

    fn empty_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator {
        let mut indexes = 0u16;
        for (idx, value) in control_bytes.iter().enumerate() {
            if *value == CONTROL_BYTE_EMPTY {
                indexes |= 1 << idx;
            }
        }
        SetBitIterator::new(indexes)
    }
}
