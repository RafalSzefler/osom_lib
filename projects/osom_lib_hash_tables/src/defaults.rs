//! Contains the default, recommended choice for the hash table.

use core::fmt::Debug;
use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::{kvp::KVP, length::Length};
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    abseil::{defaults::DefaultAbseilConfig, hash_table::AbseilHashTable},
    errors::{HashTableError, TryCloneHashTableError},
    traits::{ImmutableHashTable, MutableHashTable},
};

type InnerMap<TKey, TValue, TAllocator> = AbseilHashTable<TKey, TValue, DefaultAbseilConfig<TAllocator>>;

/// Represents the default hash table.
///
/// # Notes
///
/// At the moment it uses the [`AbseilHashTable`] as the underlying implementation.
/// This can change in the future though.
#[repr(transparent)]
#[must_use]
pub struct DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    inner: InnerMap<TKey, TValue, TAllocator>,
}

impl<TKey, TValue, TAllocator> Debug for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DefaultHashTable[{}]", stringify!(AbseilHashTable))
    }
}

unsafe impl<TKey, TValue, TAllocator> Send for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Send + Eq + Hash,
    TValue: Send,
    TAllocator: Allocator + Send,
    InnerMap<TKey, TValue, TAllocator>: Sync,
{
}

unsafe impl<TKey, TValue, TAllocator> Sync for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Sync + Eq + Hash,
    TValue: Sync,
    TAllocator: Allocator + Sync,
    InnerMap<TKey, TValue, TAllocator>: Sync,
{
}

unsafe impl<TKey, TValue, TAllocator> ReprC for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash + ReprC,
    TValue: ReprC,
    TAllocator: Allocator + ReprC,
    InnerMap<TKey, TValue, TAllocator>: ReprC,
{
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TKey>();
        osom_lib_reprc::hidden::is_reprc::<TValue>();
        osom_lib_reprc::hidden::is_reprc::<TAllocator>();
        osom_lib_reprc::hidden::is_reprc::<InnerMap<TKey, TValue, TAllocator>>();
    };
}

impl<TKey, TValue, TAllocator> DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
{
    /// Creates a new, empty [`DefaultHashTable`] with the default allocator.
    #[inline(always)]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new, empty [`DefaultHashTable`] with the specified allocator.
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            inner: AbseilHashTable::with_config(DefaultAbseilConfig::with_allocator(allocator)),
        }
    }

    /// Creates a new [`DefaultHashTable`] with the specified capacity and the default allocator.
    ///
    /// This likely will (over)allocate memory.
    ///
    /// # Errors
    ///
    /// See [`HashTableError`] for details.
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, HashTableError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`DefaultHashTable`] with the specified capacity and the specified allocator.
    ///
    /// This likely will (over)allocate memory.
    ///
    /// # Errors
    ///
    /// See [`HashTableError`] for details.
    #[inline(always)]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, HashTableError> {
        let inner = AbseilHashTable::with_capacity_and_config(capacity, DefaultAbseilConfig::with_allocator(allocator))?;
        Ok(Self { inner })
    }
}

impl<TKey, TValue, TAllocator> ImmutableHashTable<TKey, TValue> for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: ImmutableHashTable<TKey, TValue>,
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
    fn get_key_value<Q>(&self, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_key_value(key)
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> impl Iterator<Item = KVP<&'a TKey, &'a TValue>>
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
    InnerMap<TKey, TValue, TAllocator>: MutableHashTable<TKey, TValue>,
{
    #[inline(always)]
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut TValue>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_mut(key)
    }

    #[inline(always)]
    fn get_key_value_mut<Q>(&mut self, key: &Q) -> Option<KVP<&TKey, &mut TValue>>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_key_value_mut(key)
    }

    #[inline(always)]
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.remove_entry(key)
    }

    #[inline(always)]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = KVP<&'a TKey, &'a mut TValue>>
    where
        TKey: 'a,
        TValue: 'a,
        Self: 'a,
    {
        self.inner.iter_mut()
    }

    #[inline(always)]
    fn try_insert_or_update_with<FAdd, FUpdate>(
        &mut self,
        key: TKey,
        adder: FAdd,
        updater: FUpdate,
    ) -> Result<&mut TValue, crate::errors::HashTableError>
    where
        FAdd: FnOnce() -> TValue,
        FUpdate: FnOnce(&mut TValue),
    {
        self.inner.try_insert_or_update_with(key, adder, updater)
    }
}

impl<TKey, TValue, TAllocator> Default for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator + Default,
    InnerMap<TKey, TValue, TAllocator>: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<TKey, TValue, TAllocator> TryClone for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: TryClone<Error = TryCloneHashTableError>,
{
    type Error = TryCloneHashTableError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.inner.try_clone()?;
        Ok(Self { inner })
    }
}

impl<TKey, TValue, TAllocator> Clone for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: TryClone<Error = TryCloneHashTableError>,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("[DefaultHashTable::try_clone] failure")
    }
}

impl<TKey, TValue, TAllocator> PartialEq for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<TKey, TValue, TAllocator> Eq for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: Eq,
{
}

impl<TKey, TValue, TAllocator> Hash for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash,
    TAllocator: Allocator,
    InnerMap<TKey, TValue, TAllocator>: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[cfg(feature = "std")]
use osom_lib_alloc::std_allocator::StdAllocator;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// An alias for [`DefaultHashTable`] with [`StdAllocator`]. Requires `std` feature.
pub type StdDefaultHashTable<TKey, TValue> = DefaultHashTable<TKey, TValue, StdAllocator>;
