#![allow(dead_code, unused_variables)]

use osom_lib_alloc::Allocator;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;
use osom_lib_primitives::Length;

use crate::bplus_tree::nodes::NodeTaggedPtr;

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
/// B+ tree, but it also meast more space wasted.
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
    /// The allocator used to allocate the nodes.
    allocator: TAllocator,
    root: NodeTaggedPtr<NODE_CAPACITY, TKey, TValue>,
}

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    pub const MAX_SIZE: usize = Length::MAX;

    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            allocator,
            root: NodeTaggedPtr::null(),
        }
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
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
