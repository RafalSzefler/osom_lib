use core::borrow::Borrow;
use core::hash::Hash;

use osom_lib_primitives::kvp::KVP;
use osom_lib_primitives::length::Length;

use crate::abseil::hash_table::abseil_unsafe_iter::AbseilUnsafeIter;
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
            Some(kvp) => Some(kvp.value),
            None => None,
        }
    }

    fn get_key_value<Q>(&self, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let result = self.get_key_value_raw(key)?;
        unsafe { Some(result.as_ref_unchecked().as_ref_kvp()) }
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> impl Iterator<Item = KVP<&'a TKey, &'a TValue>>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        AbseilUnsafeIter::from_hash_table(self).map(|kvp| ptr_to_ref!(kvp).as_ref_kvp())
    }
}
