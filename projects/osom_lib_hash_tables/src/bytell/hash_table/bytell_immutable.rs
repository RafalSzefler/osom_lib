use core::{borrow::Borrow, hash::Hash, marker::PhantomData};

use osom_lib_primitives::{length::Length, power_of_two::PowerOfTwo32};

use crate::{
    bytell::{
        configuration::BytellConfig,
        hash_table::{BytellHashTable, block_layout::BlockLayoutHolder, entry::Entry},
    },
    helpers::ptr_to_ref,
    traits::ImmutableHashTable,
};

struct BytellImmutableIter<'a, TKey: 'a, TValue: 'a, TConfig> {
    data: *mut u8,
    last_element_index: u32,
    blocks_count: PowerOfTwo32,
    _marker: PhantomData<(&'a TKey, &'a TValue, TConfig)>,
}

impl<'a, TKey: 'a, TValue: 'a, TConfig> BytellImmutableIter<'a, TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    #[inline(always)]
    pub const fn from_hash_table(
        table: &'a BytellHashTable<TKey, TValue, TConfig>,
    ) -> BytellImmutableIter<'a, TKey, TValue, TConfig> {
        Self {
            data: table.data,
            last_element_index: 0,
            blocks_count: table.blocks_count,
            _marker: PhantomData,
        }
    }
}

impl<'a, TKey: 'a, TValue: 'a, TConfig> Iterator for BytellImmutableIter<'a, TKey, TValue, TConfig> {
    type Item = (&'a TKey, &'a TValue);

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

                let kvp = ptr_to_ref!(entry.kvp());
                self.last_element_index = el_idx;
                return Some((&kvp.key, &kvp.value));
            }
        }
    }
}

impl<TKey, TValue, TConfig> ImmutableHashTable<TKey, TValue> for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    #[inline(always)]
    fn length(&self) -> Length {
        self.length()
    }

    #[inline(always)]
    fn contains<Q>(&self, key: &Q) -> bool
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.get_key_value(key).is_some()
    }

    #[inline(always)]
    fn get<Q>(&self, key: &Q) -> Option<&TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.get_key_value(key) {
            Some(pair) => Some(pair.1),
            None => None,
        }
    }

    #[inline(never)]
    fn get_key_value<Q>(&self, key: &Q) -> Option<(&TKey, &TValue)>
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

            loop {
                let kvp = ptr_to_ref!(entry.kvp());
                if kvp.key.borrow() == key {
                    return Some((&kvp.key, &kvp.value));
                }
                if let Some(next) = entry.next_link() {
                    entry = next;
                } else {
                    return None;
                }
            }
        }
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a TKey, &'a TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        BytellImmutableIter::from_hash_table(self)
    }
}
