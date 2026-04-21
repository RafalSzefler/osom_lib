use core::{borrow::Borrow, hash::Hash, marker::PhantomData};

use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::power_of_two::PowerOfTwo32;

use crate::{
    bytell::{
        configuration::BytellConfig,
        hash_table::{BytellHashTable, block_layout::BlockLayoutHolder, control_byte::ControlByte, entry::Entry},
    },
    helpers::{KVP, ptr_to_mut, ptr_to_ref},
    traits::MutableHashTable,
};

struct BytellMutableIter<'a, TKey: 'a, TValue: 'a, TConfig> {
    data: *mut u8,
    last_element_index: u32,
    blocks_count: PowerOfTwo32,
    _marker: PhantomData<(&'a TKey, &'a mut TValue, TConfig)>,
}

impl<'a, TKey: 'a, TValue: 'a, TConfig> BytellMutableIter<'a, TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    pub const fn from_hash_table(
        table: &'a BytellHashTable<TKey, TValue, TConfig>,
    ) -> BytellMutableIter<'a, TKey, TValue, TConfig> {
        Self {
            data: table.data,
            last_element_index: 0,
            blocks_count: table.blocks_count,
            _marker: PhantomData,
        }
    }
}

impl<'a, TKey: 'a, TValue: 'a, TConfig> Iterator for BytellMutableIter<'a, TKey, TValue, TConfig> {
    type Item = (&'a TKey, &'a mut TValue);

    fn next(&mut self) -> Option<Self::Item> {
        let elements_count = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value();
        let capacity = self.blocks_count.value() * elements_count;
        let mut el_idx = self.last_element_index;

        loop {
            unsafe {
                if el_idx == capacity {
                    self.last_element_index = el_idx;
                    return None;
                }

                let entry = Entry::<TKey, TValue>::new(self.data, self.blocks_count, el_idx as usize);
                el_idx += 1;

                if !(*entry.control_byte()).contains_data() {
                    continue;
                }

                let kvp = ptr_to_mut!(entry.kvp());
                self.last_element_index = el_idx;
                return Some((&kvp.key, &mut kvp.value));
            }
        }
    }
}

impl<TKey, TValue, TConfig> MutableHashTable<TKey, TValue> for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    #[inline]
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
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        if self.blocks_count.value() == 0 {
            return None;
        }

        unsafe {
            let mut entry = self.get_entry_by_key(key);
            let control_byte = ptr_to_ref!(entry.control_byte());
            if !control_byte.is_direct_hit() {
                return None;
            }
            let mut entry_parent = None;

            loop {
                if ptr_to_ref!(entry.kvp()).key.borrow() == key {
                    break;
                }

                if let Some(next) = entry.next_link() {
                    entry_parent = Some(entry);
                    entry = next;
                    continue;
                }

                return None;
            }

            // Swap found entry with the tail
            {
                let mut tail_parent = None;
                let tail = {
                    let mut current_entry = entry.clone();
                    loop {
                        if let Some(next) = current_entry.next_link() {
                            tail_parent = Some(current_entry);
                            current_entry = next;
                        } else {
                            break current_entry;
                        }
                    }
                };

                if tail != entry {
                    let tail_kvp = ptr_to_mut!(tail.kvp());
                    let entry_kvp = ptr_to_mut!(entry.kvp());
                    core::mem::swap(tail_kvp, entry_kvp);
                    entry_parent = tail_parent;
                    entry = tail;
                }
            }

            if let Some(acutal_entry_parent) = entry_parent {
                ptr_to_mut!(acutal_entry_parent.control_byte()).set_distance_index(0);
            }

            let kvp = entry.kvp().read();
            *ptr_to_mut!(entry.control_byte()) = ControlByte::EMPTY;
            self.elements_count -= 1;
            Some((kvp.key, kvp.value))
        }
    }

    fn insert_or_update_with<FAdd, FUpdate>(&mut self, key: TKey, adder: FAdd, updater: FUpdate) -> &mut TValue
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        if self.blocks_count.value() == 0 {
            self.grow();
        }

        let mut counter = 0;

        #[allow(clippy::redundant_else)]
        'start: loop {
            counter += 1;
            assert!(counter < 30, "Tried to insert over 30 times. Giving up.");

            unsafe {
                let entry = self.get_entry_by_key(&key);
                let control_byte = *entry.control_byte();

                debug_check_or_release_hint!(
                    control_byte != ControlByte::RESERVED,
                    "Got reserved here? Should not happen."
                );

                if control_byte.is_direct_hit() {
                    let mut entry = entry;
                    loop {
                        let kvp = ptr_to_mut!(entry.kvp());
                        if kvp.key == key {
                            updater(&mut kvp.value);
                            return &mut kvp.value;
                        }

                        if let Some(next) = entry.next_link() {
                            entry = next;
                            continue;
                        }

                        break;
                    }

                    if self.should_grow() {
                        self.grow();
                        continue 'start;
                    }

                    let Some((free_entry, free_distance_index)) = self.search_for_free_entry(&entry) else {
                        self.grow();
                        continue 'start;
                    };

                    *free_entry.control_byte() = ControlByte::NEW_TAIL;
                    ptr_to_mut!(entry.control_byte()).set_distance_index(free_distance_index);
                    let new_kvp = KVP {
                        key: key,
                        value: adder(),
                    };
                    free_entry.kvp().write(new_kvp);
                    self.elements_count += 1;
                    return &mut ptr_to_mut!(free_entry.kvp()).value;
                } else {
                    if self.should_grow() {
                        self.grow();
                        continue 'start;
                    }

                    if control_byte != ControlByte::EMPTY {
                        // It is neither direct hit nor empty. Which means we hit storage entry.
                        // We need to move around the linked list this entry belongs to.

                        let mut current_parent = self.find_parent_for_storage_entry(&entry);
                        let mut current_entry = entry.clone();

                        let mut first_iteration = true;
                        loop {
                            let Some((free_entry, free_distance_index)) = self.search_for_free_entry(&current_parent)
                            else {
                                self.grow();
                                continue 'start;
                            };

                            *ptr_to_mut!(free_entry.control_byte()) = ControlByte::NEW_TAIL;
                            free_entry.kvp().write(current_entry.kvp().read());
                            ptr_to_mut!(current_parent.control_byte()).set_distance_index(free_distance_index);

                            let new_control = if first_iteration {
                                first_iteration = false;
                                ControlByte::RESERVED
                            } else {
                                ControlByte::EMPTY
                            };

                            let next_link = current_entry.next_link();
                            *current_entry.control_byte() = new_control;

                            let Some(next) = next_link else {
                                break;
                            };

                            current_parent = free_entry;
                            current_entry = next;
                        }
                    }

                    self.elements_count += 1;
                    *entry.control_byte() = ControlByte::NEW_DIRECT_HIT;
                    let kvp = KVP {
                        key: key,
                        value: adder(),
                    };
                    entry.kvp().write(kvp);
                    return &mut ptr_to_mut!(entry.kvp()).value;
                }
            }
        }
    }

    #[inline(always)]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a TKey, &'a mut TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        BytellMutableIter::from_hash_table(self)
    }
}
