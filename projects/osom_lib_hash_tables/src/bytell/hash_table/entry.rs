#![allow(clippy::cast_possible_truncation)]
use core::marker::PhantomData;

use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::{kvp::KVP, power_of_two::PowerOfTwo32};
use osom_lib_reprc::macros::reprc;

use crate::bytell::{
    constants::JUMP_DISTANCES,
    hash_table::{block_layout::BlockLayoutHolder, control_byte::ControlByte},
};

#[reprc]
#[must_use]
pub struct Entry<TKey, TValue> {
    beginning_of_data: *mut u8,
    blocks_count: PowerOfTwo32,
    element_index: u32,
    _marker: PhantomData<KVP<TKey, TValue>>,
}

impl<TKey, TValue> PartialEq for Entry<TKey, TValue> {
    fn eq(&self, other: &Self) -> bool {
        self.element_index == other.element_index
            && self.beginning_of_data == other.beginning_of_data
            && self.blocks_count == other.blocks_count
    }
}

impl<TKey, TValue> Eq for Entry<TKey, TValue> {}

impl<TKey, TValue> Entry<TKey, TValue> {
    #[inline(always)]
    #[allow(clippy::used_underscore_binding)]
    pub const fn new(beginning_of_data: *mut u8, blocks_count: PowerOfTwo32, element_index: usize) -> Self {
        debug_check_or_release_hint!(!beginning_of_data.is_null(), "beginning_of_data is null");
        debug_check_or_release_hint!(blocks_count.as_usize() > 0, "block_count is zero");
        debug_check_or_release_hint!(
            blocks_count.as_usize().is_power_of_two(),
            "block_count is not a power of two"
        );

        let block_capacity = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value() as usize;
        let range = blocks_count.as_usize() * block_capacity;

        debug_check_or_release_hint!(element_index < range, "element_index outside block_count range");

        let _ = block_capacity;
        let _ = range;

        let element_index = element_index as u32;
        Self {
            beginning_of_data,
            blocks_count,
            element_index,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn control_byte(&self) -> *mut ControlByte {
        let block_ptr = self.block_ptr();
        let in_block_index = self.in_block_index();
        debug_check_or_release_hint!(!block_ptr.is_null(), "block_ptr is null");
        unsafe { block_ptr.add(in_block_index) }.cast()
    }

    #[inline(always)]
    pub const fn kvp(&self) -> *mut KVP<TKey, TValue> {
        let block_ptr = self.block_ptr();
        let in_block_index = self.in_block_index();
        let data_offset = BlockLayoutHolder::<TKey, TValue>::LAYOUT.data_offset();
        let nth_kvp = size_of::<KVP<TKey, TValue>>() * in_block_index;
        unsafe { block_ptr.add(data_offset + nth_kvp) }.cast()
    }

    #[inline(always)]
    pub const unsafe fn clone(&self) -> Self {
        Self {
            beginning_of_data: self.beginning_of_data,
            blocks_count: self.blocks_count,
            element_index: self.element_index,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn element_index(&self) -> u32 {
        self.element_index
    }

    #[inline(always)]
    const fn block_ptr(&self) -> *mut u8 {
        let block_capacity = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value();
        let block_index = (self.element_index / block_capacity) as usize;
        debug_check_or_release_hint!(block_index < self.blocks_count.as_usize(), "block_index out of range");
        let block_size = BlockLayoutHolder::<TKey, TValue>::LAYOUT.layout().size();
        let offset = block_index * block_size;
        unsafe { self.beginning_of_data.add(offset) }
    }

    #[inline(always)]
    const fn in_block_index(&self) -> usize {
        let block_capacity = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value();
        debug_check_or_release_hint!(block_capacity > 0, "block_capacity is zero");
        (self.element_index & (block_capacity - 1)) as usize
    }

    #[inline(always)]
    const fn offset(&self, offset: usize) -> Self {
        let new_index = self.element_index.wrapping_add(offset as u32);
        let range = self.blocks_count.value() * BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value();
        debug_check_or_release_hint!(range.is_power_of_two(), "range is not a power of two");
        debug_check_or_release_hint!(range > 0, "range is zero");

        let new_index = new_index & (range - 1);

        Self {
            beginning_of_data: self.beginning_of_data,
            blocks_count: self.blocks_count,
            element_index: new_index,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn next_link(&self) -> Option<Self> {
        let index = unsafe { *self.control_byte() }.distance_index();
        if index == 0 {
            return None;
        }
        debug_check_or_release_hint!(
            index < JUMP_DISTANCES.len(),
            "Tried to access beyond JUMP_DISTANCES array."
        );
        let offset = JUMP_DISTANCES[index];
        Some(self.offset(offset))
    }
}
