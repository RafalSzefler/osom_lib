use osom_lib_reprc::macros::reprc;

use crate::abseil::hash_table::{abseil_layout::ABSEIL_BLOCK_SIZE, set_bit_iterator::SetBitIterator};

#[reprc]
pub struct FullBlockScanResult {
    pub matching_indexes: SetBitIterator,
    pub empty_buckets: SetBitIterator,
    pub tombstones: SetBitIterator,
}

#[reprc]
pub struct MatchingAndEmptyBlockScanResult {
    pub matching_indexes: SetBitIterator,
    pub empty_buckets: SetBitIterator,
}

pub trait PlatformOps {
    /// The function returns an iterator over those index that point to valid, taken bucket.
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator;

    /// Scans the block for matching partial_hash, empty buckets and tombstones at the same time.
    fn full_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> FullBlockScanResult;

    /// Scans the block for matching partial_hash or empty buckets.
    fn matching_block_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8)
    -> MatchingAndEmptyBlockScanResult;

    /// Scans the block for empty buckets only.
    fn empty_scan(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator;
}

cfg_select! {
    // (any(target_arch = "x86", target_arch = "x86_64")) => {
    //     compile_error!("Abseil hash table is not supported on x86 or x86_64 targets.");
    // },
    target_arch = "aarch64" => {
        mod aarch64;
        pub type PlatformImpl = aarch64::Aarch64PlatformOps;
    },
    _ => {
        mod portable;
        pub type PlatformImpl = portable::PortablePlatformOps;
    },
}
