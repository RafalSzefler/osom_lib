use osom_lib_reprc::traits::ReprC;

use crate::{abseil::hash_table::abseil_layout::ABSEIL_BLOCK_SIZE, helpers::KVP};

pub(super) const CONTROL_BYTE_EMPTY: u8 = 0x80;
pub(super) const CONTROL_BYTE_TOMBSTONE: u8 = 0xff;

#[repr(C)]
#[must_use]
pub struct AbseilBlock<TKey, TValue> {
    control_block: *mut [u8; ABSEIL_BLOCK_SIZE],
    key_values: *mut [KVP<TKey, TValue>; ABSEIL_BLOCK_SIZE],
}

unsafe impl<TKey, TValue> ReprC for AbseilBlock<TKey, TValue>
where
    TKey: ReprC,
    TValue: ReprC,
{
    const CHECK: () = {
        let () = <*mut [u8; ABSEIL_BLOCK_SIZE] as ReprC>::CHECK;
        let () = <*mut [KVP<TKey, TValue>; ABSEIL_BLOCK_SIZE] as ReprC>::CHECK;
    };
}

impl<TKey, TValue> AbseilBlock<TKey, TValue> {
    #[inline(always)]
    pub const fn new(
        control_block: *mut [u8; ABSEIL_BLOCK_SIZE],
        key_values: *mut [KVP<TKey, TValue>; ABSEIL_BLOCK_SIZE],
    ) -> Self {
        Self {
            control_block,
            key_values,
        }
    }

    #[inline(always)]
    pub const fn control_block_ptr(&self) -> *mut [u8; ABSEIL_BLOCK_SIZE] {
        self.control_block
    }

    #[inline(always)]
    pub const fn key_values_ptr(&self) -> *mut [KVP<TKey, TValue>; ABSEIL_BLOCK_SIZE] {
        self.key_values
    }

    #[inline(always)]
    pub const fn key_value_pair_at_index(&self, index: usize) -> *mut KVP<TKey, TValue> {
        unsafe { self.key_values_ptr().cast::<KVP<TKey, TValue>>().add(index) }
    }
}
