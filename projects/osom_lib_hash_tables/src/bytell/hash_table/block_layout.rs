//! Unfortunately the size of block is not known at compile time. Not until
//! `generic_const_exprs` feature is stabilized anyway.
//!
//! Therefore we simply work with `*mut u8` pointers and with [`BlockLayout`]
//! struct that tells us how a blocked is layed out in memory.
#![allow(clippy::cast_possible_truncation)]

use core::{alloc::Layout, marker::PhantomData};

use osom_lib_primitives::power_of_two::PowerOfTwo32;
use osom_lib_reprc::macros::reprc;

use crate::helpers::KVP;

#[reprc]
pub struct BlockLayout {
    /// The number of elements a block can hold.
    block_capacity: PowerOfTwo32,

    /// The binary size of the block.
    size: usize,

    /// The alignment of the block.
    alignment: PowerOfTwo32,
}

impl BlockLayout {
    pub const fn new<TKey, TValue>() -> Self {
        let block_capacity = const {
            let mut result = 16usize;
            let key_align = align_of::<TKey>();
            if key_align > result {
                result = key_align;
            }
            let value_align = align_of::<TValue>();
            if value_align > result {
                result = value_align;
            }
            assert!(result.is_power_of_two());
            assert!(result < (1 << 16) as usize);
            unsafe { PowerOfTwo32::new_unchecked(result as u32) }
        };

        let metadata_size = block_capacity.as_usize() * size_of::<u8>();
        let metadata_align = align_of::<u8>();
        let metadata_layout = unsafe { Layout::from_size_align_unchecked(metadata_size, metadata_align) };

        let data_size = block_capacity.as_usize() * size_of::<KVP<TKey, TValue>>();
        let data_align = align_of::<KVP<TKey, TValue>>();
        let data_layout = unsafe { Layout::from_size_align_unchecked(data_size, data_align) };

        let Ok((total_layout, offset)) = metadata_layout.extend(data_layout) else {
            panic!("Failed to calculate block layout");
        };

        assert!(offset == block_capacity.as_usize());
        let align = total_layout.align();
        assert!(align.is_power_of_two());
        assert!(align < (1 << 20) as usize);
        let align = align as u32;

        Self {
            block_capacity,
            size: total_layout.size(),
            alignment: unsafe { PowerOfTwo32::new_unchecked(align) },
        }
    }

    /// Returns the number of elements a block can hold.
    #[inline(always)]
    pub const fn block_capacity(&self) -> PowerOfTwo32 {
        self.block_capacity
    }

    /// Returns the binary layout of the block.
    #[inline(always)]
    pub const fn layout(&self) -> Layout {
        unsafe { Layout::from_size_align_unchecked(self.size, self.alignment.as_usize()) }
    }

    /// Returns byte offset in the binary layout where the actualy (key, value)
    /// data starts.
    #[inline(always)]
    pub const fn data_offset(&self) -> usize {
        self.block_capacity.as_usize()
    }
}

#[repr(transparent)]
pub struct BlockLayoutHolder<TKey, TValue> {
    _marker: PhantomData<(TKey, TValue)>,
}

impl<TKey, TValue> BlockLayoutHolder<TKey, TValue> {
    pub const LAYOUT: BlockLayout = const {
        let layout = BlockLayout::new::<TKey, TValue>();
        assert!(layout.block_capacity().as_usize() > 0);
        assert!(layout.block_capacity().as_usize().is_power_of_two());
        layout
    };
}
