use core::alloc::Layout;
use core::marker::PhantomData;

use osom_lib_primitives::power_of_two::PowerOfTwo32;

use crate::helpers::KVP;

pub const ABSEIL_BLOCK_SIZE: usize = 16;

#[repr(C)]
#[must_use]
pub struct AbseilLayout<TKey, TValue> {
    control_blocks_offset: usize,
    control_blocks_size: usize,
    key_value_pairs_offset: usize,
    key_value_pairs_size: usize,
    total_layout: Layout,
    _marker: PhantomData<KVP<TKey, TValue>>,
}

#[inline(always)]
const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

impl<TKey, TValue> AbseilLayout<TKey, TValue> {
    #[inline]
    pub const fn new(capacity: PowerOfTwo32) -> Self {
        const {
            assert!(ABSEIL_BLOCK_SIZE.is_power_of_two());
            assert!(ABSEIL_BLOCK_SIZE > 0);
        }
        let control_blocks_offset = 0;
        let control_blocks_size = capacity.as_usize() * size_of::<u8>() * ABSEIL_BLOCK_SIZE;
        let layout =
            unsafe { Layout::from_size_align_unchecked(control_blocks_size, max(ABSEIL_BLOCK_SIZE, align_of::<u8>())) };

        let key_value_pairs_size = capacity.as_usize() * size_of::<KVP<TKey, TValue>>() * ABSEIL_BLOCK_SIZE;
        let key_value_pairs_layout =
            unsafe { Layout::from_size_align_unchecked(key_value_pairs_size, align_of::<KVP<TKey, TValue>>()) };

        let Ok((total_layout, key_value_pairs_offset)) = layout.extend(key_value_pairs_layout) else {
            panic!("Failed to calculate AbseilLayout");
        };

        Self {
            control_blocks_offset,
            control_blocks_size,
            key_value_pairs_offset,
            key_value_pairs_size,
            total_layout,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn total_layout(&self) -> Layout {
        self.total_layout
    }

    #[inline(always)]
    pub const fn control_blocks_offset(&self) -> usize {
        self.control_blocks_offset
    }

    #[inline(always)]
    pub const fn control_blocks_size(&self) -> usize {
        self.control_blocks_size
    }

    #[inline(always)]
    pub const fn key_value_pairs_offset(&self) -> usize {
        self.key_value_pairs_offset
    }
}

const _: () = const {
    assert!(
        ABSEIL_BLOCK_SIZE == 16,
        "ABSEIL_BLOCK_SIZE constant should always be 16"
    );
};
