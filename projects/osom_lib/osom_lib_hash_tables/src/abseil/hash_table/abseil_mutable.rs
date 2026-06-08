use core::borrow::Borrow;
use core::hash::Hash;

use osom_lib_primitives::{kvp::KVP, length::Length};

use crate::abseil::hash_table::abseil_block::CONTROL_BYTE_TOMBSTONE;
use crate::abseil::hash_table::abseil_unsafe_iter::AbseilUnsafeMutIter;
use crate::abseil::hash_table::platform::{PlatformImpl, PlatformOps as _};
use crate::abseil::utils::probe_block_indexes;
use crate::errors::HashTableError;
use crate::helpers::{ptr_to_mut, ptr_to_ref};
use crate::traits::MutableHashTable;

use crate::abseil::{configuration::AbseilConfig, hash_table::AbseilHashTable};

impl<TKey, TValue, TConfig> MutableHashTable<TKey, TValue> for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.get_key_value_mut(key).map(|kvp| kvp.value)
    }

    fn get_key_value_mut<Q>(&mut self, key: &Q) -> Option<KVP<&TKey, &mut TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let result = self.get_key_value_raw(key)?;
        unsafe {
            let (key_ptr, value_ptr) = KVP::unpack_ptr(result);
            Some(KVP {
                key: key_ptr.as_ref_unchecked(),
                value: value_ptr.as_mut_unchecked(),
            })
        }
    }

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let blocks_count = self.blocks_count();
        let (h1, h2) = self.config.calculate_partial_hashes(key);

        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index);
            let control_bytes = ptr_to_mut!(block.control_block_ptr());
            let mut scan_data = PlatformImpl::matching_block_scan(control_bytes, h2);
            for matching_index in scan_data.matching_indexes {
                let kvp_ptr = block.key_value_pair_at_index(matching_index);
                let kvp = ptr_to_ref!(kvp_ptr);
                if kvp.key.borrow() == key {
                    unsafe { core::hint::assert_unchecked(matching_index < control_bytes.len()) };
                    control_bytes[matching_index] = CONTROL_BYTE_TOMBSTONE;
                    let kvp = unsafe { kvp_ptr.read() };
                    unsafe {
                        self.elements_count = Length::new_unchecked(self.elements_count.as_u32().unchecked_sub(1));
                    }
                    return Some(kvp);
                }
            }

            if scan_data.empty_buckets.next().is_some() {
                return None;
            }
        }

        None
    }

    fn try_insert_or_update_with<FAdd, FUpdate>(
        &mut self,
        key: TKey,
        adder: FAdd,
        updater: FUpdate,
    ) -> Result<&mut TValue, HashTableError>
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        if self.remaining_capacity == Length::ZERO {
            self.grow_for_size(self.elements_count + 1)?;
        }

        let (h1, h2) = self.config.calculate_partial_hashes(&key);
        let blocks_count = self.blocks_count();

        // Single pass: search for an existing key while simultaneously tracking
        // the first tombstone and watching for an empty slot (which terminates
        // the probe chain and proves the key is absent).
        let mut first_tombstone: Option<(usize, usize)> = None; // (group_index, slot_index)

        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index);
            let control_bytes = ptr_to_mut!(block.control_block_ptr());
            let mut scan_result = PlatformImpl::full_block_scan(control_bytes, h2);

            for matching_index in scan_result.matching_indexes {
                let kvp_ptr = block.key_value_pair_at_index(matching_index);
                let kvp = ptr_to_mut!(kvp_ptr);
                if kvp.key == key {
                    updater(&mut kvp.value);
                    return Ok(&mut kvp.value);
                }
            }

            if first_tombstone.is_none()
                && let Some(tombstone_idx) = scan_result.tombstones.next()
            {
                first_tombstone = Some((group_index, tombstone_idx));
            }

            if let Some(empty_idx) = scan_result.empty_buckets.next() {
                // Empty slot proves the key is absent. Prefer tombstone (reuse deleted slot)
                // over the empty slot when available.
                let (target_group, target_slot) = if let Some((tg, ts)) = first_tombstone {
                    (tg, ts)
                } else {
                    self.remaining_capacity =
                        unsafe { Length::new_unchecked(self.remaining_capacity.as_u32().unchecked_sub(1)) };
                    (group_index, empty_idx)
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
                }

                let kvp = ptr_to_mut!(target_kvp_ptr);

                // Safety: the pointer is valid for the lifetime of self.
                return Ok(&mut kvp.value);
            }
        }

        // With correct remaining_capacity management there is always at least one
        // empty slot in the table, so this path is unreachable.
        unreachable!("no empty slot found despite remaining_capacity > 0")
    }

    #[inline(always)]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = KVP<&'a TKey, &'a mut TValue>> + 'a
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        AbseilUnsafeMutIter::from_hash_table(self).map(|kvp| ptr_to_mut!(kvp).as_mut_kvp())
    }
}
