use crate::abseil::hash_table::{abseil_layout::ABSEIL_BLOCK_SIZE, set_bit_iterator::SetBitIterator};

pub trait PlatformOps {
    /// The function returns an iterator over indexes of those control bytes that match the passed
    /// partial_hash.
    fn iter_matching_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE], partial_hash: u8) -> SetBitIterator;

    /// The function returns an iterator over those index that point to valid, taken bucket.
    fn iter_data_indexes(control_bytes: &[u8; ABSEIL_BLOCK_SIZE]) -> SetBitIterator;
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
