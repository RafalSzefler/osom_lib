use core::mem::ManuallyDrop;

use osom_lib_alloc::{Allocator, StdAllocator};

use crate::traits::{Compare,Tree, TreeQueryResult, TreeQueryMutResult, TreeQueryExactResult, TreeQueryExactMutResult};

use super::node::NodePtr;

const NODE_CAPACITY: usize = 16;

pub struct BPlusTree<
    TKey,
    TValue,
    #[cfg(feature = "std_alloc")]
    TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))]
    TAllocator
>
where TKey: Ord,
    TAllocator: Allocator
{
    /// The root node of the tree.
    root: NodePtr<NODE_CAPACITY, TKey, TValue>,

    /// The allocator used to allocate the nodes of the tree.
    /// It has to be `ManuallyDrop` to ensure that nodes are dropped
    /// before it.
    allocator: ManuallyDrop<TAllocator>,
}

impl<TKey, TValue, TAllocator> BPlusTree<TKey, TValue, TAllocator>
where TKey: Ord,
    TAllocator: Allocator
{
    #[inline(always)]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            root: NodePtr::null(),
            allocator: ManuallyDrop::new(allocator),
        }
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }
}

impl<TKey, TValue, TAllocator> Tree for BPlusTree<TKey, TValue, TAllocator>
where TKey: Ord,
    TAllocator: Allocator
{
    type TKey = TKey;

    type TValue = TValue;

    fn query_exact<K>(&self, key: &K) -> TreeQueryExactResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>
    {
        todo!()
        // if self.root.is_null() {
        //     return TreeQueryExactResult::NotFound;
        // }

        // let mut node = &self.root;

        // while !node.is_leaf() {
        //     let internal_node = unsafe { node.as_internal() };
        //     let place_idx = internal_node.data.first_greater_or_equal_position(key);
        //     let edges = internal_node.edges.as_slice();
        //     node = unsafe { edges[place_idx as usize].assume_init_ref() };
        // }

        // let leaf_node = unsafe { node.as_leaf() };
        // let place_idx = leaf_node.data.first_greater_or_equal_position(key);
        // if place_idx == leaf_node.data.len as usize {
        //     return TreeQueryExactResult::NotFound;
        // }

        // let key = &leaf_node.data.keys()[place_idx];
        // let value = unsafe { &leaf_node.values[place_idx].assume_init_ref() };
        // TreeQueryExactResult::Found { key, value }
    }

    fn query_exact_mut<K>(&mut self, key: &K) -> TreeQueryExactMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K> {
        todo!()
    }

    fn query_range<K>(
        &self,
        range: impl core::ops::RangeBounds<K>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K> {
        todo!()
    }

    fn query_range_mut<K>(
        &mut self,
        range: impl core::ops::RangeBounds<K>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K> {
        todo!()
    }
}