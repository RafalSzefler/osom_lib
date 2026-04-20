use core::hash::Hash;

use osom_lib_primitives::length::Length;

use crate::abseil::hash_table::abseil_block::{CONTROL_BYTE_EMPTY, CONTROL_BYTE_TOMBSTONE};
use crate::abseil::hash_table::abseil_unsafe_iter::AbseilUnsafeMutIter;
use crate::abseil::hash_table::platform::{PlatformImpl, PlatformOps as _};
use crate::abseil::utils::probe_block_indexes;
use crate::helpers::{KVP, ptr_to_mut, ptr_to_ref};
use crate::traits::MutableHashTable;

use crate::abseil::{configuration::AbseilConfig, hash_table::AbseilHashTable};

impl<TKey, TValue, TConfig> MutableHashTable<TKey, TValue> for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
        let value_ptr = &raw const value;
        let adder = || unsafe { value_ptr.read() };

        let mut result: Option<TValue> = None;
        let updater = |current: &mut TValue| {
            let value = unsafe { value_ptr.read() };
            let old_value = core::mem::replace(current, value);
            result = Some(old_value);
        };

        let _ = self.insert_or_update_with(key, adder, updater);
        core::mem::forget(value);
        result
    }

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(TKey, TValue)>
    where
        TKey: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let blocks_count = self.blocks_count();
        let (h1, h2) = self.config.calculate_partial_hashes(key);

        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index);
            let control_bytes = ptr_to_mut!(block.control_block_ptr());
            for matching_index in PlatformImpl::iter_matching_indexes(control_bytes, h2) {
                let kvp_ptr = block.key_value_pair_at_index(matching_index);
                let kvp = ptr_to_ref!(kvp_ptr);
                if kvp.key.borrow() == key {
                    unsafe { core::hint::assert_unchecked(matching_index < control_bytes.len()) };
                    control_bytes[matching_index] = CONTROL_BYTE_TOMBSTONE;
                    let kvp = unsafe { kvp_ptr.read() };
                    unsafe {
                        self.elements_count = Length::new_unchecked(self.elements_count.as_u32().unchecked_sub(1));
                    }
                    return Some((kvp.key, kvp.value));
                }
            }
        }

        None
    }

    #[inline(never)]
    fn insert_or_update_with<FAdd, FUpdate>(&mut self, key: TKey, adder: FAdd, updater: FUpdate) -> &mut TValue
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        if self.remaining_capacity == Length::ZERO {
            self.grow_for_size(self.elements_count + 1);
        }

        let (h1, h2) = self.config.calculate_partial_hashes(&key);
        let blocks_count = self.blocks_count();

        // Single pass: search for an existing key while simultaneously tracking
        // the first tombstone and watching for an empty slot (which terminates
        // the probe chain and proves the key is absent).
        let mut first_tombstone: Option<(usize, usize)> = None; // (group_index, slot_index)

        // TODO: can the three iter_matching_indexes be combined into one? That's what Claude thinks,
        // and apparantly std::HashMap does.
        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index);
            let control_bytes = ptr_to_mut!(block.control_block_ptr());

            for matching_index in PlatformImpl::iter_matching_indexes(control_bytes, h2) {
                let kvp_ptr = block.key_value_pair_at_index(matching_index);
                let kvp = ptr_to_mut!(kvp_ptr);
                if kvp.key == key {
                    updater(&mut kvp.value);
                    return &mut kvp.value;
                }
            }

            if first_tombstone.is_none()
                && let Some(ts_idx) = PlatformImpl::iter_matching_indexes(control_bytes, CONTROL_BYTE_TOMBSTONE).next()
            {
                first_tombstone = Some((group_index, ts_idx));
            }

            if let Some(empty_idx) = PlatformImpl::iter_matching_indexes(control_bytes, CONTROL_BYTE_EMPTY).next() {
                // Empty slot proves the key is absent. Prefer tombstone (reuse deleted slot)
                // over the empty slot when available.
                let (target_group, target_slot, used_empty) = match first_tombstone {
                    Some((tg, ts)) => (tg, ts, false),
                    None => (group_index, empty_idx, true),
                };

                let target_block = self.get_block_by_index(target_group);
                let target_ctrl = ptr_to_mut!(target_block.control_block_ptr());
                let target_kvp_ptr = target_block.key_value_pair_at_index(target_slot);

                unsafe {
                    *target_ctrl.get_unchecked_mut(target_slot) = h2;
                    target_kvp_ptr.write(KVP { key, value: adder() });
                };

                unsafe {
                    self.elements_count = Length::new_unchecked(self.elements_count.as_u32().unchecked_add(1));
                    if used_empty {
                        self.remaining_capacity =
                            Length::new_unchecked(self.remaining_capacity.as_u32().unchecked_sub(1));
                    }
                }

                // Safety: the pointer is valid for the lifetime of self.
                return unsafe { &mut (*target_kvp_ptr).value };
            }
        }

        // With correct remaining_capacity management there is always at least one
        // empty slot in the table, so this path is unreachable.
        unreachable!("no empty slot found despite remaining_capacity > 0")
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a TKey, &'a mut TValue)> + 'a
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        AbseilUnsafeMutIter::from_hash_table(self).map(|kvp| {
            let kvp_ref = ptr_to_mut!(kvp);
            (&kvp_ref.key, &mut kvp_ref.value)
        })
    }
}
