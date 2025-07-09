use core::mem::ManuallyDrop;

use osom_lib_alloc::{Allocator, StdAllocator};

use crate::bplus_tree::node::NodePtr;

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
