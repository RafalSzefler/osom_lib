#![allow(clippy::cast_sign_loss)]

use core::ops::{Bound, RangeBounds};

use osom_lib_alloc::Allocator;

use osom_lib_primitives::Length;

use crate::{
    bplus_tree::{
        helpers::{self, deallocate_recursive},
        nodes::{LeafItem, LeafItemRange, NodeTaggedPtr},
        operation_results::{BPlusTreeQueryMutResult, BPlusTreeQueryResult},
    },
    traits::{
        Compare, Ordering, Tree, TreeError, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult,
        TreeQueryResult, TreeTryInsertResult,
    },
};

/// A B+ tree implementation.
///
/// # Notes
///
/// `TKey` has to be `Clone` in addition to `Ord`. That's because the tree will
/// keep duplicates of the same key inside itself. Be aware that it would be
/// best to have cloning a lightweight operation.
///
/// `NODE_CAPACITY` refers to the size of internal nodes. This has to be at least
/// `4` and at most `i16::MAX`. Bigger nodes means flatter and more efficient
/// B+ tree, but it also meast more space wasted. It is recommended to make
/// `NODE_CAPACITY` so that array of keys and/or array of values of this
/// size fills entire cache line.
#[must_use]
pub struct BPlusTree<TKey, TValue, TAllocator, const NODE_CAPACITY: usize>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    /// The allocator used to allocate the nodes.
    pub(super) allocator: TAllocator,
    pub(super) root: NodeTaggedPtr<NODE_CAPACITY, TKey, TValue>,
    pub(super) len: Length,
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    pub const MAX_SIZE: usize = Length::MAX;

    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        const {
            assert!(NODE_CAPACITY >= 4, "NODE_CAPACITY must be at least 4");
            assert!(
                NODE_CAPACITY <= i16::MAX as usize,
                "NODE_CAPACITY must be at most 32767"
            );
        };

        Self {
            allocator,
            root: NodeTaggedPtr::null(),
            len: Length::ZERO,
        }
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.len
    }

    pub(super) fn search_for_infimum<K>(&self, key: &K) -> LeafItem<NODE_CAPACITY, TKey, TValue>
    where
        TKey: Compare<K>,
    {
        if self.root.is_null() {
            return LeafItem::null();
        }

        let mut current_node = &self.root;
        while !current_node.is_leaf() {
            let internal_node = unsafe { current_node.as_internal() };
            let index = helpers::scan_node(key, internal_node.data().keys());
            current_node = &internal_node.edges()[index as usize];
        }

        let leaf = unsafe { current_node.as_leaf_mut() };
        let index = helpers::scan_node(key, leaf.data().keys());
        LeafItem {
            node: core::ptr::from_mut(leaf),
            index,
        }
    }

    /// # Safety
    ///
    /// It doesn't check whether root is null.
    unsafe fn min(&self) -> LeafItem<NODE_CAPACITY, TKey, TValue> {
        let mut current_node = &self.root;
        while !current_node.is_leaf() {
            let internal_node = unsafe { current_node.as_internal() };
            current_node = &internal_node.edges()[0];
        }

        let leaf = unsafe { current_node.as_leaf_mut() };
        LeafItem {
            node: core::ptr::from_mut(leaf),
            index: 0,
        }
    }

    /// # Safety
    ///
    /// It doesn't check whether root is null.
    unsafe fn max(&self) -> LeafItem<NODE_CAPACITY, TKey, TValue> {
        let mut current_node = &self.root;
        while !current_node.is_leaf() {
            let internal_node = unsafe { current_node.as_internal() };
            let index = internal_node.data().keys().len() - 1;
            current_node = &internal_node.edges().as_slice()[index.value() as usize];
        }

        let leaf = unsafe { current_node.as_leaf_mut() };
        LeafItem {
            node: core::ptr::from_mut(leaf),
            index: leaf.data().keys().len().value() - 1,
        }
    }

    pub(super) fn search_range<K>(&self, range: impl RangeBounds<K>) -> LeafItemRange<NODE_CAPACITY, TKey, TValue>
    where
        TKey: Compare<K>,
    {
        if self.root.is_null() {
            return LeafItemRange::null();
        }

        let start = match range.start_bound() {
            Bound::Included(key) => {
                let leaf_item = self.search_for_infimum(key);
                if unsafe { leaf_item.key().is_equal(key) } {
                    leaf_item
                } else {
                    leaf_item.next()
                }
            }
            Bound::Excluded(key) => {
                let leaf_item = self.search_for_infimum(key);
                leaf_item.next()
            }
            Bound::Unbounded => unsafe { self.min() },
        };
        let end = match range.end_bound() {
            Bound::Included(key) => self.search_for_infimum(key),
            Bound::Excluded(key) => {
                let leaf_item = self.search_for_infimum(key);
                if unsafe { leaf_item.key().is_equal(key) } {
                    leaf_item.prev()
                } else {
                    leaf_item
                }
            }
            Bound::Unbounded => unsafe { self.max() },
        };
        LeafItemRange { start, end }
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Default
    for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Drop for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        deallocate_recursive(&mut self.root, &mut self.allocator);
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Tree for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    type TKey = TKey;

    type TValue = TValue;

    fn try_insert(&mut self, key: Self::TKey, value: Self::TValue) -> Result<TreeTryInsertResult, TreeError> {
        self.internal_try_insert(key, value)
    }

    fn query_exact<K>(&self, key: &K) -> TreeQueryExactResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        let leaf_item = self.search_for_infimum(key);
        let leaf_key = unsafe { &*leaf_item.key_ptr() };
        if leaf_key.is_equal(key) {
            let leaf_value = unsafe { &*leaf_item.value_ptr() };
            TreeQueryExactResult::Found {
                key: leaf_key,
                value: leaf_value,
            }
        } else {
            TreeQueryExactResult::NotFound
        }
    }

    fn query_exact_mut<K>(&mut self, key: &K) -> TreeQueryExactMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        let leaf_item = self.search_for_infimum(key);
        let leaf_key = unsafe { &*leaf_item.key_ptr() };
        if leaf_key.is_equal(key) {
            let leaf_value = unsafe { &mut *leaf_item.value_ptr() };
            TreeQueryExactMutResult::Found {
                key: leaf_key,
                value: leaf_value,
            }
        } else {
            TreeQueryExactMutResult::NotFound
        }
    }

    fn query_range<K>(
        &self,
        range: impl RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        let leaf_item_range = self.search_range(range);
        BPlusTreeQueryResult::new(leaf_item_range, ordering)
    }

    fn query_range_mut<K>(
        &mut self,
        range: impl RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        let leaf_item_range = self.search_range(range);
        BPlusTreeQueryMutResult::new(leaf_item_range, ordering)
    }
}

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

#[cfg(feature = "std_alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "std_alloc")))]
/// Alias for [`BPlusTree`] with [`StdAllocator`] as the allocator
/// and with default `NODE_CAPACITY` set to `16`.
///
/// This alias is available only if the `std_alloc` feature is enabled.
pub type StdBPlusTree<TKey, TValue, const NODE_CAPACITY: usize = 16> =
    BPlusTree<TKey, TValue, StdAllocator, NODE_CAPACITY>;
