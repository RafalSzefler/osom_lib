#![allow(clippy::implicit_hasher)]
use core::borrow::Borrow;
use core::hash::Hash;

use std::collections::{HashMap, hash_map::Entry};

use osom_lib_primitives::kvp::KVP;
use osom_lib_primitives::length::Length;

use crate::errors::HashTableError;
use crate::traits::{ImmutableHashTable, MutableHashTable};

impl<TKey, TValue> ImmutableHashTable<TKey, TValue> for HashMap<TKey, TValue>
where
    TKey: Hash + Eq,
{
    #[inline]
    fn length(&self) -> Length {
        Length::try_from_usize(self.len()).unwrap()
    }

    #[inline]
    fn contains<Q>(&self, key: &Q) -> bool
    where
        TKey: Borrow<Q>,
        Q: Eq + core::hash::Hash + ?Sized,
    {
        self.contains_key(key)
    }

    #[inline]
    fn get<Q>(&self, key: &Q) -> Option<&TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + core::hash::Hash + ?Sized,
    {
        self.get(key)
    }

    #[inline]
    fn get_key_value<Q>(&self, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + core::hash::Hash + ?Sized,
    {
        self.get_key_value(key).map(Into::into)
    }

    #[inline]
    fn iter<'a>(&'a self) -> impl Iterator<Item = KVP<&'a TKey, &'a TValue>>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.iter().map(Into::into)
    }
}

impl<TKey, TValue> MutableHashTable<TKey, TValue> for HashMap<TKey, TValue>
where
    TKey: Hash + Eq,
{
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.get_mut(key)
    }

    fn get_key_value_mut<Q>(&mut self, _key: &Q) -> Option<KVP<&TKey, &mut TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        panic!("std::HashMap does not support get_key_value_mut");
    }

    #[inline]
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.remove_entry(key).map(Into::into)
    }

    #[inline]
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
        let reference = match self.entry(key) {
            Entry::Occupied(mut o) => {
                updater(o.get_mut());
                o.into_mut()
            }
            Entry::Vacant(v) => v.insert(adder()),
        };

        Ok(reference)
    }

    #[inline]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = KVP<&'a TKey, &'a mut TValue>>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.iter_mut().map(Into::into)
    }
}
