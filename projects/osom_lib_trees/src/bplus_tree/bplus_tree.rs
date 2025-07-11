#![allow(clippy::needless_range_loop)]
use core::alloc::Layout;
use core::mem::MaybeUninit;

use osom_lib_alloc::{AllocatedMemory as _, Allocator, StdAllocator};

use crate::bplus_tree::node::{InternalNode, LeafNode};
use crate::bplus_tree::operation_results::{BPlusTreeInsertResult, BPlusTreeQueryMutResultIterator, BPlusTreeQueryResultIterator};
use crate::traits::{
    Compare, Ordering, Tree, TreeError, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult,
    TreeQueryResult, TreeTryInsertResult,
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
        Self::new().unwrap()
    }
}

fn recursive_drop<TKey, TValue, TAllocator, const NODE_CAPACITY: usize>(
    allocator: &mut TAllocator,
    node: &NodePtr<NODE_CAPACITY, TKey, TValue>,
) where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
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
            let edge = unsafe { edges[i].assume_init_ref() };
            if edge.is_null() {
                continue;
            }
            recursive_drop(allocator, edge);
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
    /// Creates a new B+ tree with the given allocator.
    ///
    /// It will allocate the root node straight away.
    ///
    /// # Errors
    ///
    /// For possible errors, see [`TreeError`].
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Result<Self, TreeError> {
        let layout = Layout::new::<LeafNode<NODE_CAPACITY, TKey, TValue>>();
        let root_ptr = allocator.allocate(layout)?;
        let root_ptr = unsafe { root_ptr.as_ptr::<LeafNode<NODE_CAPACITY, TKey, TValue>>() };
        unsafe { root_ptr.write(LeafNode::new()) };
        let root_node = NodePtr::from_leaf(root_ptr);

        Ok(Self {
            root: root_node,
            allocator,
        })
    }

    /// Creates a new B+ tree with the default allocator.
    ///
    /// It will allocate the root node straight away.
    ///
    /// # Errors
    ///
    /// For possible errors, see [`TreeError`].
    #[inline(always)]
    pub fn new() -> Result<Self, TreeError> {
        Self::with_allocator(TAllocator::default())
    }

    fn query_exact_internal<K>(&self, key: &K) -> BPlusTreeQueryExactResult<TKey, TValue>
    where
        TKey: Compare<K>,
    {
        let mut node = &self.root;

        if unsafe { node.as_data().len } == 0 {
            // This can happen only when the tree is empty.
            return BPlusTreeQueryExactResult::NotFound;
        }

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

        let matched_key = &leaf_node.data.keys()[place_idx];
        if matched_key.is_not_equal(key) {
            return BPlusTreeQueryExactResult::NotFound;
        }

        let value: *mut TValue = unsafe { leaf_node.values[place_idx].assume_init_mut() };
        BPlusTreeQueryExactResult::Found {
            key: matched_key,
            value: value,
        }
    }

    fn insert_internal(&mut self, key: TKey, value: TValue) -> Result<BPlusTreeInsertResult<TKey, TValue>, TreeError> {
        let mut node = &self.root;

        if unsafe { node.as_data().len } == 0 {
            // This can happen only when the tree is empty. In which case it has to be a leaf node
            // and we can just insert the key-value pair at 0.
            let leaf_node = unsafe { node.as_leaf() };
            leaf_node.data.keys_mut()[0] = key;
            leaf_node.data.len += 1;
            leaf_node.values[0] = MaybeUninit::new(value);
            unsafe {
                return Ok(BPlusTreeInsertResult::Inserted {
                    key: leaf_node.data.keys[0].assume_init_ref(),
                    value: leaf_node.values[0].assume_init_mut(),
                });
            }
        }

        while !node.is_leaf() {
            // Search for the corresponding leaf.
            let internal_node = unsafe { node.as_internal() };
            let place_idx = internal_node.data.first_greater_or_equal_position(&key);
            let edges = internal_node.edges.as_slice();
            node = unsafe { edges[place_idx].assume_init_ref() };
        }

        let leaf_node = unsafe { node.as_leaf() };
        let place_idx = leaf_node.data.first_greater_or_equal_position(&key);
        if place_idx < leaf_node.data.len as usize {
            // The key already exists.
            let matched_key = unsafe { leaf_node.data.keys[place_idx].assume_init_ref() };
            if matched_key.is_equal(&key) {
                let value: *mut TValue = unsafe { leaf_node.values[place_idx].assume_init_mut() };
                return Ok(BPlusTreeInsertResult::AlreadyExists {
                    key: matched_key,
                    value: value,
                });
            }
        }
       
        let leaf_node = if leaf_node.data.len == NODE_CAPACITY as u16 {
            // Split the leaf node.
            let new_leaf_node_ptr = self.allocator.allocate(Layout::new::<LeafNode<NODE_CAPACITY, TKey, TValue>>())?;
            let new_leaf_node_ptr = unsafe { new_leaf_node_ptr.as_ptr::<LeafNode<NODE_CAPACITY, TKey, TValue>>() };
            unsafe { new_leaf_node_ptr.write(LeafNode::new()) };

            let new_leaf_ref = unsafe { &mut *new_leaf_node_ptr };
            // Move edge pointers around.
            new_leaf_ref.prev = leaf_node;
            new_leaf_ref.next = leaf_node.next;
            leaf_node.next = new_leaf_node_ptr;

            // Move data
            let middle = (NODE_CAPACITY / 2) + 1;
            for idx in middle..NODE_CAPACITY {
                new_leaf_ref.values[idx - middle].write(unsafe { leaf_node.values[idx].assume_init_read() });
            }

            let mut needs_plit = true;
            let mut current_node = new_leaf_ref.;
            while needs_plit {
            }
            
            new_leaf_ref
        } else {
            leaf_node
        };
        todo!("Insert item in order")
    }
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> Tree for BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    type TKey = TKey;

    type TValue = TValue;

    #[allow(unused_variables, invalid_value, unreachable_code, clippy::uninit_assumed_init)]
    fn try_insert(&mut self, key: Self::TKey, value: Self::TValue) -> Result<TreeTryInsertResult, TreeError> {
        todo!()
    }

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
