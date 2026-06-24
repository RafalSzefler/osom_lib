//! Contains the default, recommended configuration for the B-tree.
use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::btree::{BTree, BTreeConfig};

/// The default configuration for [`BTree`].
#[reprc]
#[repr(transparent)]
#[derive(Debug, Default)]
#[must_use]
pub struct DefaultBTreeConfig<TAllocator: Allocator> {
    allocator: TAllocator,
}

impl<TAllocator: Allocator> DefaultBTreeConfig<TAllocator> {
    /// Creates a new [`DefaultBTreeConfig`] with the default allocator.
    #[inline]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new [`DefaultBTreeConfig`] with the specified allocator.
    #[inline(always)]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        Self { allocator }
    }
}

impl<TAllocator: Allocator> BTreeConfig for DefaultBTreeConfig<TAllocator> {
    type ConcreteAllocator = TAllocator;

    const CHILDREN_COUNT: usize = 16;

    #[inline(always)]
    fn allocator_mut(&mut self) -> &mut Self::ConcreteAllocator {
        &mut self.allocator
    }
}

/// An alias for [`BTree`] with [`DefaultBTreeConfig`].
pub type DefaultBTree<TKey, TValue, TAllocator> = BTree<TKey, TValue, DefaultBTreeConfig<TAllocator>>;

impl<TAllocator: Allocator + TryClone> TryClone for DefaultBTreeConfig<TAllocator> {
    type Error = <TAllocator as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self {
            allocator: self.allocator.try_clone()?,
        })
    }
}
