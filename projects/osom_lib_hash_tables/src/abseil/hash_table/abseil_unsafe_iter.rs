use core::hash::Hash;

use osom_lib_primitives::power_of_two::PowerOfTwo32;

use crate::{
    abseil::configuration::AbseilConfig,
    helpers::{KVP, ptr_to_ref},
};

use super::platform::{PlatformImpl, PlatformOps};
use super::{AbseilHashTable, set_bit_iterator::SetBitIterator};

pub struct AbseilUnsafeIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    table: *const AbseilHashTable<TKey, TValue, TConfig>,
    current_group_idx: u32,
    current_group_iter: SetBitIterator,
}

pub struct AbseilUnsafeMutIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    table: *mut AbseilHashTable<TKey, TValue, TConfig>,
    current_group_idx: u32,
    current_group_iter: SetBitIterator,
}

macro_rules! from_hash_table {
    ( $table: expr ) => {{
        let table = { $table };
        let table_ref = ptr_to_ref!(table);
        if table_ref.blocks_count() == PowerOfTwo32::ZERO {
            return Self {
                table,
                current_group_idx: 0,
                current_group_iter: SetBitIterator::new(0),
            };
        }

        let first_block = table_ref.get_block_by_index(0);
        let control_bytes = ptr_to_ref!(first_block.control_block_ptr());
        Self {
            table,
            current_group_idx: 0,
            current_group_iter: PlatformImpl::iter_data_indexes(control_bytes),
        }
    }};
}

macro_rules! next_data_index {
    ( $self: expr ) => {{
        let s = { $self };
        let table_ref = ptr_to_ref!(s.table);
        let blocks_count = table_ref.blocks_count().value();

        loop {
            if let Some(index) = s.current_group_iter.next() {
                return Some(index);
            }
            s.current_group_idx += 1;
            if s.current_group_idx >= blocks_count {
                break;
            }
            let block = table_ref.get_block_by_index(s.current_group_idx as usize);
            let control_bytes = ptr_to_ref!(block.control_block_ptr());
            s.current_group_iter = PlatformImpl::iter_data_indexes(control_bytes);
        }

        None
    }};
}

impl<TKey, TValue, TConfig> AbseilUnsafeIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    #[inline(always)]
    pub fn from_hash_table(table: *const AbseilHashTable<TKey, TValue, TConfig>) -> Self {
        from_hash_table!(table)
    }

    fn next_data_index(&mut self) -> Option<usize> {
        next_data_index!(self)
    }
}

impl<TKey, TValue, TConfig> AbseilUnsafeMutIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    #[inline(always)]
    pub fn from_hash_table(table: *mut AbseilHashTable<TKey, TValue, TConfig>) -> Self {
        from_hash_table!(table)
    }

    fn next_data_index(&mut self) -> Option<usize> {
        next_data_index!(self)
    }
}

impl<TKey, TValue, TConfig> Iterator for AbseilUnsafeIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    type Item = *const KVP<TKey, TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let table_ref = ptr_to_ref!(self.table);
        let idx = self.next_data_index()?;
        let block = table_ref.get_block_by_index(self.current_group_idx as usize);
        Some(block.key_value_pair_at_index(idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let table = ptr_to_ref!(self.table);
        (0, Some(table.elements_count.as_usize()))
    }
}

impl<TKey, TValue, TConfig> Iterator for AbseilUnsafeMutIter<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    type Item = *mut KVP<TKey, TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let table_ref = ptr_to_ref!(self.table);
        let idx = self.next_data_index()?;
        let block = table_ref.get_block_by_index(self.current_group_idx as usize);
        Some(block.key_value_pair_at_index(idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let table = ptr_to_ref!(self.table);
        (0, Some(table.elements_count.as_usize()))
    }
}
