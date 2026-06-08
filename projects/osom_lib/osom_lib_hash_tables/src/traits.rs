//! Defines hash table traits.

use core::borrow::Borrow;
use core::hash::Hash;

use osom_lib_primitives::kvp::KVP;
use osom_lib_primitives::length::Length;

use crate::errors::HashTableError;

/// Represents an immutable hash table.
pub trait ImmutableHashTable<TKey, TValue>: Sized
where
    TKey: Hash + Eq,
{
    /// Returns the number of `(TKey, TValue)` pairs the table
    /// contains. This typically **does not** correspond to the actual
    /// size of the table in bytes.
    fn length(&self) -> Length;

    /// Checks if the table contains the corresponding `key`.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn contains<Q>(&self, key: &Q) -> bool
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Checks if the table contains the corresponding `key`, and if so then returns
    /// the reference to the `TValue`, or `None` otherwise.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn get<Q>(&self, key: &Q) -> Option<&TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Checks if the table contains the corresponding `key`, and if so then returns
    /// the `(&TKey, &TValue)` pair or `None` otherwise.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn get_key_value<Q>(&self, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Checks if the table is empty. This does not mean that it doesn't take space
    /// in memory. Equivalent to `self.length() == 0` check.
    fn is_empty(&self) -> bool {
        self.length() == Length::ZERO
    }

    /// Returns an iterator over the key-value pairs in the table.
    ///
    /// # Notes
    ///
    /// The iterator yields `(&TKey, &TValue)` tuples representing each key-value pair
    /// in the hash table.
    fn iter<'a>(&'a self) -> impl Iterator<Item = KVP<&'a TKey, &'a TValue>> + 'a
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a;
}

/// An extension of [`ImmutableHashTable`] that allows the actual modifications
/// to the table.
pub trait MutableHashTable<TKey, TValue>: ImmutableHashTable<TKey, TValue>
where
    TKey: Hash + Eq,
{
    /// Checks if the table contains the corresponding `key`, and if so then returns
    /// a mutable reference to the `TValue`, or `None` otherwise.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Checks if the table contains the corresponding `key`, and if so then returns
    /// a mutable reference to the `TValue`, or `None` otherwise.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn get_key_value_mut<Q>(&mut self, key: &Q) -> Option<KVP<&TKey, &mut TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Inserts given `(TKey, TValue)` pair into the table.
    ///
    /// Return `None` if the `key` didn't already exist in the table.
    ///
    /// Otherwise returns the old `TValue`.
    ///
    /// # Panics
    ///
    /// Whenever the corresponding [`MutableHashTable::try_insert`] would fail.
    fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
        self.try_insert(key, value)
            .expect("[MutableHashTable::try_insert] failure")
    }

    /// Tries to insert given `(TKey, TValue)` pair into the table.
    ///
    /// Return `None` if the `key` didn't already exist in the table.
    ///
    /// Otherwise returns the old `TValue`.
    ///
    /// # Errors
    ///
    /// For details see [`HashTableError`].
    fn try_insert(&mut self, key: TKey, value: TValue) -> Result<Option<TValue>, HashTableError> {
        let value_ptr = &raw const value;
        let adder = || unsafe { value_ptr.read() };

        let mut result: Option<TValue> = None;
        let updater = |current: &mut TValue| {
            let value = unsafe { value_ptr.read() };
            let old_value = core::mem::replace(current, value);
            result = Some(old_value);
        };

        let _ = self.try_insert_or_update_with(key, adder, updater)?;
        core::mem::forget(value);
        Ok(result)
    }

    /// Removes entire entry from the table. Returns `(TKey, TValue)` pair
    /// for the matching `key` or `None` if there is no match.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    /// Searches the table for a given `key`. If the table contains it, then
    /// it runs `updater` on the corresponding `TValue`. Otherwise runs `adder`
    /// to add a new `TValue` to the table. Returns the mutable reference to the
    /// final `TValue`.
    ///
    /// # Notes
    ///
    /// The implementation has to guarantee that one of: `adder` or `updater` will be called
    /// during its execution, but not both.
    ///
    /// # Panics
    ///
    /// Whenever the corresponding [`MutableHashTable::try_insert_or_update_with`] would fail.
    fn insert_or_update_with<FAdd, FUpdate>(&mut self, key: TKey, adder: FAdd, updater: FUpdate) -> &mut TValue
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        self.try_insert_or_update_with(key, adder, updater)
            .expect("[MutableHashTable::try_insert_or_update_with] failure")
    }

    /// Searches the table for a given `key`. If the table contains it, then
    /// it runs `updater` on the corresponding `TValue`. Otherwise runs `adder`
    /// to add a new `TValue` to the table. Returns the mutable reference to the
    /// final `TValue`.
    ///
    /// # Notes
    ///
    /// The implementation has to guarantee that one of: `adder` or `updater` will be called
    /// during its execution, but not both.
    ///
    /// # Errors
    ///
    /// For details see [`HashTableError`].
    fn try_insert_or_update_with<FAdd, FUpdate>(
        &mut self,
        key: TKey,
        adder: FAdd,
        updater: FUpdate,
    ) -> Result<&mut TValue, HashTableError>
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue);

    /// Removes entire entry from the table. Similar to [`MutableHashTable::remove_entry`],
    /// but returns `TValue` only for the matching `key` or `None` if there is no match.
    ///
    /// # Notes
    ///
    /// The borrowed `Q` type's `Hash` and `Eq` must match those for `TKey`.
    #[inline(always)]
    fn remove<Q>(&mut self, key: &Q) -> Option<TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        if let Some(kvp) = self.remove_entry(key) {
            let (_, value) = kvp.unpack();
            Some(value)
        } else {
            None
        }
    }

    /// Retrieves an existing `TValue`, or inserts a default one.
    ///
    /// Internally equivalent to `self.try_get_or_insert_default(key)`.
    ///
    /// # Panics
    ///
    /// Whenever the corresponding [`MutableHashTable::try_get_or_insert_default`] would fail.
    #[inline(always)]
    fn get_or_insert_default(&mut self, key: TKey) -> &mut TValue
    where
        TValue: Default,
    {
        self.try_get_or_insert_default(key)
            .expect("[MutableHashTable::try_get_or_insert_default] failure")
    }

    /// Retrieves an existing `TValue`, or inserts a default one.
    ///
    /// Internally equivalent to `self.try_insert_or_update_with(key, TValue::default, |_| {})`.
    ///
    /// # Errors
    ///
    /// For details see [`HashTableError`].
    #[inline(always)]
    fn try_get_or_insert_default(&mut self, key: TKey) -> Result<&mut TValue, HashTableError>
    where
        TValue: Default,
    {
        self.try_insert_or_update_with(key, TValue::default, |_| {})
    }

    /// Retrieves an existing `TValue`, or inserts the passed one.
    ///
    /// Internally equivalent to `self.try_get_or_insert(key, value)`.
    ///
    /// # Panics
    ///
    /// Whenever the corresponding [`MutableHashTable::try_get_or_insert`] would fail.
    #[inline(always)]
    fn get_or_insert(&mut self, key: TKey, value: TValue) -> &mut TValue {
        self.try_get_or_insert(key, value)
            .expect("[MutableHashTable::try_get_or_insert] failure")
    }

    /// Retrieves an existing `TValue`, or inserts the passed one.
    ///
    /// Internally equivalent to `self.try_insert_or_update_with(key, || value, || {})`.
    ///
    /// # Errors
    ///
    /// For details see [`HashTableError`].
    #[inline(always)]
    fn try_get_or_insert(&mut self, key: TKey, value: TValue) -> Result<&mut TValue, HashTableError> {
        self.try_insert_or_update_with(key, || value, |_| {})
    }

    /// Returns a mutable iterator over the key-value pairs in the table.
    ///
    /// # Notes
    ///
    /// The iterator yields `(&TKey, &mut TValue)` tuples representing each key-value pair
    /// in the hash table.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = KVP<&'a TKey, &'a mut TValue>> + 'a
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a;
}
