#![allow(clippy::needless_range_loop)]
use core::alloc::Layout;
use core::mem::MaybeUninit;

use osom_lib_alloc::{AllocatedMemory as _, Allocator, StdAllocator};

use crate::bplus_tree::node::{InternalNode, LeafNode};
use crate::bplus_tree::operation_results::{BPlusTreeQueryMutResultIterator, BPlusTreeQueryResultIterator};
use crate::traits::{
    Compare, Ordering, Tree, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult, TreeQueryResult,
};

use super::node::NodePtr;
use super::operation_results::BPlusTreeQueryExactResult;

#[must_use]
pub struct BPlusTree<
    TKey,
    TValue,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
    const NODE_CAPACITY: usize = 16,
> where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    /// The root node of the tree.
    root: NodePtr<NODE_CAPACITY, TKey, TValue>,

    allocator: TAllocator,
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Drop for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        recursive_drop(&mut self.allocator, &self.root);
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

fn recursive_drop<TKey, TValue, TAllocator, const NODE_CAPACITY: usize>(
    allocator: &mut TAllocator,
    node: &NodePtr<NODE_CAPACITY, TKey, TValue>,
) where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    if node.is_null() {
        return;
    }

    if node.is_leaf() {
        let leaf = unsafe { node.as_leaf() };
        if core::mem::needs_drop::<TValue>() {
            for i in 0..leaf.data.len as usize {
                unsafe { core::ptr::drop_in_place(leaf.values[i].as_mut_ptr()) };
            }
        }

        if core::mem::needs_drop::<TKey>() {
            for i in 0..leaf.data.len as usize {
                unsafe { core::ptr::drop_in_place(leaf.data.keys[i].as_mut_ptr()) };
            }
        }

        unsafe {
            let ptr = allocator.convert_raw_ptr(leaf);
            let layout = Layout::new::<LeafNode<NODE_CAPACITY, TKey, TValue>>();
            ptr.deallocate(layout);
        }
    } else {
        let internal_node = unsafe { node.as_internal() };
        let edges = internal_node.edges.as_slice();
        for i in 0..=internal_node.data.len as usize {
            recursive_drop(allocator, unsafe { edges[i].assume_init_ref() });
        }

        if core::mem::needs_drop::<TKey>() {
            for i in 0..internal_node.data.len as usize {
                unsafe { core::ptr::drop_in_place(internal_node.data.keys[i].as_mut_ptr()) };
            }
        }

        unsafe {
            let ptr = allocator.convert_raw_ptr(internal_node);
            let layout = Layout::new::<InternalNode<NODE_CAPACITY, TKey, TValue>>();
            ptr.deallocate(layout);
        }
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    #[inline(always)]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            root: NodePtr::null(),
            allocator: allocator,
        }
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    fn query_exact_internal<K>(&self, key: &K) -> BPlusTreeQueryExactResult<TKey, TValue>
    where
        TKey: Compare<K>,
    {
        if self.root.is_null() {
            return BPlusTreeQueryExactResult::NotFound;
        }

        let mut node = &self.root;

        while !node.is_leaf() {
            let internal_node = unsafe { node.as_internal() };
            let place_idx = internal_node.data.first_greater_or_equal_position(key);
            let edges = internal_node.edges.as_slice();
            node = unsafe { edges[place_idx].assume_init_ref() };
        }

        let leaf_node = unsafe { node.as_leaf() };
        let place_idx = leaf_node.data.first_greater_or_equal_position(key);
        if place_idx == leaf_node.data.len as usize {
            return BPlusTreeQueryExactResult::NotFound;
        }

        let key: *const TKey = &leaf_node.data.keys()[place_idx];
        let value: *mut TValue = unsafe { leaf_node.values[place_idx].assume_init_mut() };
        BPlusTreeQueryExactResult::Found { key, value }
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Tree for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    type TKey = TKey;

    type TValue = TValue;

    fn query_exact<K>(&self, key: &K) -> TreeQueryExactResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        self.query_exact_internal(key).into()
    }

    fn query_exact_mut<K>(&mut self, key: &K) -> TreeQueryExactMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        self.query_exact_internal(key).into()
    }

    #[allow(unused_variables, invalid_value, unreachable_code, clippy::uninit_assumed_init)]
    fn query_range<K>(
        &self,
        range: impl core::ops::RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        todo!();
        unsafe { MaybeUninit::<BPlusTreeQueryResultIterator<NODE_CAPACITY, TKey, TValue>>::uninit().assume_init() }
    }

    #[allow(unused_variables, invalid_value, unreachable_code, clippy::uninit_assumed_init)]
    fn query_range_mut<K>(
        &mut self,
        range: impl core::ops::RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>,
    {
        todo!();
        unsafe { MaybeUninit::<BPlusTreeQueryMutResultIterator<NODE_CAPACITY, TKey, TValue>>::uninit().assume_init() }
    }
}
