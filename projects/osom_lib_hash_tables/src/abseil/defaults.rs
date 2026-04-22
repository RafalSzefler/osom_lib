//! Contains the default, recommended configuration for the abseil hash table.

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::macros::reprc;

use super::hash_table::AbseilHashTable;
use crate::{
    abseil::configuration::AbseilConfig,
    helpers::{DefaultHashBuilder, MaxLoadFactor},
};

/// The default configuration for [`AbseilHashTable`].
///
/// It uses sip hash as the default hasher.
///
/// Additionally it uses `0.875` as the default max load factor.
#[reprc]
#[must_use]
pub struct DefaultAbseilConfig<TAllocator: Allocator> {
    build_hasher: DefaultHashBuilder,
    allocator: TAllocator,
}

unsafe impl<TAllocator: Allocator + Send> Send for DefaultAbseilConfig<TAllocator> where MaxLoadFactor: Send {}
unsafe impl<TAllocator: Allocator + Sync> Sync for DefaultAbseilConfig<TAllocator> where MaxLoadFactor: Sync {}

impl<TAllocator: Allocator> DefaultAbseilConfig<TAllocator> {
    /// Creates a new [`DefaultAbseilConfig`] with the default allocator.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new [`DefaultAbseilConfig`] with the specified allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            build_hasher: DefaultHashBuilder::new(),
            allocator,
        }
    }
}

impl<TAllocator: Allocator> Default for DefaultAbseilConfig<TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator> Clone for DefaultAbseilConfig<TAllocator> {
    fn clone(&self) -> Self {
        Self {
            build_hasher: self.build_hasher,
            allocator: self.allocator.clone(),
        }
    }
}

impl<TAllocator: Allocator> AbseilConfig for DefaultAbseilConfig<TAllocator> {
    type ConcreteBuildHasher = DefaultHashBuilder;

    type ConcreteAllocator = TAllocator;

    fn build_hasher(&self) -> &Self::ConcreteBuildHasher {
        &self.build_hasher
    }

    fn allocator(&self) -> &Self::ConcreteAllocator {
        &self.allocator
    }

    fn load_factor(&self) -> MaxLoadFactor {
        MaxLoadFactor::new(0.875)
    }
}

/// An alias for [`AbseilHashTable`] with [`DefaultAbseilConfig`].
pub type DefaultAbseilHashTable<TKey, TValue, TAllocator> =
    AbseilHashTable<TKey, TValue, DefaultAbseilConfig<TAllocator>>;

#[cfg(feature = "std")]
use osom_lib_alloc::std_allocator::StdAllocator;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// An alias for [`DefaultAbseilHashTable`] with [`StdAllocator`]. Requires `std` feature.
pub type StdAbseilHashTable<TKey, TValue> = DefaultAbseilHashTable<TKey, TValue, StdAllocator>;
