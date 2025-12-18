#![allow(clippy::implicit_hasher)]
use core::borrow::Borrow;
use core::hash::Hash;

use std::collections::{HashMap, hash_map::Entry};

use osom_lib_primitives::length::Length;

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
    fn get_key_value<Q>(&self, key: &Q) -> Option<(&TKey, &TValue)>
    where
        TKey: Borrow<Q>,
        Q: Eq + core::hash::Hash + ?Sized,
    {
        self.get_key_value(key)
    }

    #[inline]
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a TKey, &'a TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.iter()
    }
}

impl<TKey, TValue> MutableHashTable<TKey, TValue> for HashMap<TKey, TValue>
where
    TKey: Hash + Eq,
{
    #[inline]
    fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
        self.insert(key, value)
    }

    #[inline]
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(TKey, TValue)>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.remove_entry(key)
    }

    #[inline]
    fn insert_or_update_with<FAdd, FUpdate>(&mut self, key: TKey, adder: FAdd, updater: FUpdate) -> &mut TValue
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        match self.entry(key) {
            Entry::Occupied(mut o) => {
                updater(o.get_mut());
                o.into_mut()
            }
            Entry::Vacant(v) => v.insert(adder()),
        }
    }

    #[inline]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a TKey, &'a mut TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.iter_mut()
    }
}
