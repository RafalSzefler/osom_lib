//! Contains the default, recommended choice for the hash table.

use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::{macros::reprc, traits::ReprC};

use crate::{
    bytell::{defaults::DefaultBytellConfig, errors::BytellError, hash_table::BytellHashTable},
    traits::{ImmutableHashTable, MutableHashTable},
};

/// Represents a general possible error for the default hash table.
#[reprc]
#[repr(u8)]
#[must_use]
pub enum DefaultHashTableError {
    /// The underlying allocator returned an error, likely due to out of memory.
    AllocationError = 0,

    /// The table is too big to be allocated.
    TableTooBigError = 1,
}

impl From<BytellError> for DefaultHashTableError {
    fn from(error: BytellError) -> Self {
        match error {
            BytellError::AllocationError => DefaultHashTableError::AllocationError,
            BytellError::TableTooBigError => DefaultHashTableError::TableTooBigError,
        }
    }
}

/// Represents the default hash table.
///
/// # Notes
///
/// At the moment it uses [`BytellHashTable`] as the underlying implementation.
/// This can change in the future though.
#[derive(PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
#[must_use]
pub struct DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    inner: BytellHashTable<TKey, TValue, DefaultBytellConfig<TAllocator>>,
}

unsafe impl<TKey, TValue, TAllocator> ReprC for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash + ReprC,
    TValue: ReprC,
    TAllocator: Allocator + ReprC,
{
    const CHECK: () = const {
        let () = <BytellHashTable<TKey, TValue, DefaultBytellConfig<TAllocator>> as ReprC>::CHECK;
    };
}

impl<TKey, TValue, TAllocator> DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    /// Creates a new, empty [`DefaultHashTable`] with the default allocator.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new, empty [`DefaultHashTable`] with the specified allocator.
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            inner: BytellHashTable::with_config(DefaultBytellConfig::with_allocator(allocator)),
        }
    }

    /// Creates a new [`DefaultHashTable`] with the specified capacity and the default allocator.
    ///
    /// This likely will (over)allocate memory.
    ///
    /// # Errors
    ///
    /// See [`DefaultHashTableError`] for details.
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, DefaultHashTableError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`DefaultHashTable`] with the specified capacity and the specified allocator.
    ///
    /// This likely will (over)allocate memory.
    ///
    /// # Errors
    ///
    /// See [`DefaultHashTableError`] for details.
    #[inline(always)]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, DefaultHashTableError> {
        let inner = BytellHashTable::with_capacity_and_config(
            capacity.as_u32(),
            DefaultBytellConfig::with_allocator(allocator),
        )?;
        Ok(Self { inner })
    }
}

impl<TKey, TValue, TAllocator> ImmutableHashTable<TKey, TValue> for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    #[inline(always)]
    fn length(&self) -> Length {
        self.inner.length()
    }

    #[inline(always)]
    fn contains<Q>(&self, key: &Q) -> bool
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.contains(key)
    }

    #[inline(always)]
    fn get<Q>(&self, key: &Q) -> Option<&TValue>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(key)
    }

    #[inline(always)]
    fn get_key_value<Q>(&self, key: &Q) -> Option<(&TKey, &TValue)>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_key_value(key)
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a TKey, &'a TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.inner.iter()
    }
}

impl<TKey, TValue, TAllocator> MutableHashTable<TKey, TValue> for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    #[inline(always)]
    fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
        self.inner.insert(key, value)
    }

    #[inline(always)]
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(TKey, TValue)>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.remove_entry(key)
    }

    #[inline(always)]
    fn insert_or_update_with<FAdd, FUpdate>(&mut self, key: TKey, adder: FAdd, updater: FUpdate) -> &mut TValue
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        self.inner.insert_or_update_with(key, adder, updater)
    }

    #[inline(always)]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a TKey, &'a mut TValue)>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.inner.iter_mut()
    }
}

impl<TKey, TValue, TAllocator> Default for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
use osom_lib_alloc::std_allocator::StdAllocator;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// An alias for [`DefaultHashTable`] with [`StdAllocator`]. Requires `std` feature.
pub type StdHashTable<TKey, TValue> = DefaultHashTable<TKey, TValue, StdAllocator>;
