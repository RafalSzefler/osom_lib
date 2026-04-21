use core::hash::Hash;

use osom_lib_primitives::length::Length;

use crate::abseil::hash_table::abseil_unsafe_iter::AbseilUnsafeIter;
use crate::abseil::hash_table::platform::{PlatformImpl, PlatformOps};
use crate::abseil::utils::probe_block_indexes;
use crate::helpers::ptr_to_ref;
use crate::traits::ImmutableHashTable;

use crate::abseil::{configuration::AbseilConfig, hash_table::AbseilHashTable};

impl<TKey, TValue, TConfig> ImmutableHashTable<TKey, TValue> for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    #[inline(always)]
    fn length(&self) -> Length {
        self.length()
    }

    #[inline(always)]
    fn contains<Q>(&self, key: &Q) -> bool
    where
        TKey: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.get_key_value(key).is_some()
    }

    #[inline(always)]
    fn get<Q>(&self, key: &Q) -> Option<&TValue>
    where
        TKey: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.get_key_value(key) {
            Some(pair) => Some(pair.1),
            None => None,
        }
    }

    fn get_key_value<Q>(&self, key: &Q) -> Option<(&TKey, &TValue)>
    where
        TKey: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let blocks_count = self.blocks_count();
        let (h1, h2) = self.config.calculate_partial_hashes(key);
        let abseil_layout = self.abseil_layout();

        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index, &abseil_layout);
            let control_bytes = ptr_to_ref!(block.control_block_ptr());
            let mut scan_result = PlatformImpl::matching_block_scan(control_bytes, h2);
            for matching_index in scan_result.matching_indexes {
                let kvp = ptr_to_ref!(block.key_value_pair_at_index(matching_index));
                if kvp.key.borrow() == key {
                    return Some((&kvp.key, &kvp.value));
                }
            }

            if scan_result.empty_buckets.next().is_some() {
                return None;
            }
        }

        None
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a TKey, &'a TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        AbseilUnsafeIter::from_hash_table(self).map(|kvp| {
            let kvp_ref = ptr_to_ref!(kvp);
            (&kvp_ref.key, &kvp_ref.value)
        })
    }
}
