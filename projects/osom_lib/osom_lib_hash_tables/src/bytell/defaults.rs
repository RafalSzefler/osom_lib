//! Contains the default, recommended configuration for the bytell hash table.

use osom_lib_alloc::traits::Allocator;
use osom_lib_hashes::siphash::SipHashBuilder;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::{
    bytell::{configuration::BytellConfig, hash_table::BytellHashTable, hash_to_index::FibonacciHashToIndex},
    helpers::{DefaultHashBuilder, MaxLoadFactor},
};

/// The default configuration for [`BytellHashTable`].
///
/// It uses [`FibonacciHashToIndex`] as the default hash-to-index policy.
///
/// Additionally it uses `0.9375` as the default max load factor.
#[reprc]
#[must_use]
pub struct DefaultBytellConfig<TAllocator: Allocator> {
    hash_to_index: FibonacciHashToIndex,
    build_hasher: DefaultHashBuilder,
    allocator: TAllocator,
}

unsafe impl<TAllocator: Allocator + Send> Send for DefaultBytellConfig<TAllocator>
where
    SipHashBuilder: Send,
    FibonacciHashToIndex: Send,
    MaxLoadFactor: Send,
{
}

unsafe impl<TAllocator: Allocator + Sync> Sync for DefaultBytellConfig<TAllocator>
where
    SipHashBuilder: Sync,
    FibonacciHashToIndex: Sync,
    MaxLoadFactor: Sync,
{
}

impl<TAllocator: Allocator> DefaultBytellConfig<TAllocator> {
    /// Creates a new [`DefaultBytellConfig`] with the default allocator.
    #[inline(always)]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new [`DefaultBytellConfig`] with the specified allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        #[allow(clippy::default_constructed_unit_structs)]
        {
            Self {
                hash_to_index: FibonacciHashToIndex::default(),
                build_hasher: DefaultHashBuilder::new(),
                allocator: allocator,
            }
        }
    }
}

impl<TAllocator: Allocator + TryClone> TryClone for DefaultBytellConfig<TAllocator> {
    type Error = <TAllocator as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self {
            hash_to_index: self.hash_to_index,
            build_hasher: self.build_hasher,
            allocator: self.allocator.try_clone()?,
        })
    }
}

impl<TAllocator: Allocator + Default> Default for DefaultBytellConfig<TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator + TryClone> BytellConfig for DefaultBytellConfig<TAllocator> {
    type ConcreteHashToIndex = FibonacciHashToIndex;
    type ConcreteBuildHasher = DefaultHashBuilder;
    type ConcreteAllocator = TAllocator;

    fn build_hasher(&self) -> &Self::ConcreteBuildHasher {
        &self.build_hasher
    }

    fn hash_to_index(&self) -> &Self::ConcreteHashToIndex {
        &self.hash_to_index
    }

    fn hash_to_index_mut(&mut self) -> &mut Self::ConcreteHashToIndex {
        &mut self.hash_to_index
    }

    fn allocator_mut(&mut self) -> &mut Self::ConcreteAllocator {
        &mut self.allocator
    }

    fn load_factor(&self) -> MaxLoadFactor {
        MaxLoadFactor::new(0.9375)
    }
}

/// An alias for [`BytellHashTable`] with [`DefaultBytellConfig`].
pub type DefaultBytellHashTable<TKey, TValue, TAllocator> =
    BytellHashTable<TKey, TValue, DefaultBytellConfig<TAllocator>>;

#[cfg(feature = "std")]
use osom_lib_alloc::std_allocator::StdAllocator;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// An alias for [`DefaultBytellHashTable`] with [`StdAllocator`]. Requires `std` feature.
pub type StdBytellHashTable<TKey, TValue> = DefaultBytellHashTable<TKey, TValue, StdAllocator>;
