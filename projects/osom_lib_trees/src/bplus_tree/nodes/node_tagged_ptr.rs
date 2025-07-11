#![allow(dead_code, unused_variables)]

use core::{alloc::Layout, marker::PhantomData};

use osom_lib_alloc::{AllocatedMemory as _, AllocationError, Allocator};

use crate::bplus_tree::nodes::NodeData;

use super::{InternalNode, LeafNode};

/// This is a tagged pointer for nodes. We take advantage of alignment being
/// at least 2 bytes and store the information about type in the last bit
/// of the pointer (as usize).
///
/// Leaf nodes correspond to tag 0, while internal nodes correspond to tag 1.
#[repr(transparent)]
pub struct NodeTaggedPtr<const N: usize, TKey, TValue> {
    numeric_ptr: usize,
    phantom: PhantomData<([TKey; N], [TValue; N])>,
}

impl<const N: usize, TKey, TValue> Clone for NodeTaggedPtr<N, TKey, TValue> {
    fn clone(&self) -> Self {
        Self {
            numeric_ptr: self.numeric_ptr,
            phantom: PhantomData,
        }
    }
}

impl<const N: usize, TKey, TValue> PartialEq for NodeTaggedPtr<N, TKey, TValue> {
    fn eq(&self, other: &Self) -> bool {
        self.numeric_ptr == other.numeric_ptr
    }
}

impl<const N: usize, TKey, TValue> Eq for NodeTaggedPtr<N, TKey, TValue> {}

impl<const N: usize, TKey, TValue> NodeTaggedPtr<N, TKey, TValue> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            numeric_ptr: 0,
            phantom: PhantomData,
        }
    }

    /// Boxes a [`LeafNode`] by allocating memory for it and properly tagging it.
    ///
    /// # Errors
    ///
    /// Returns [`AllocationError`] if the allocation fails.
    pub fn box_leaf(allocator: &mut impl Allocator, leaf: LeafNode<N, TKey, TValue>) -> Result<Self, AllocationError> {
        const {
            assert!(
                align_of::<LeafNode<N, TKey, TValue>>() >= 2,
                "LeafNode must be aligned to at least 2 bytes"
            );
        }
        let layout = Layout::new::<LeafNode<N, TKey, TValue>>();
        let ptr = allocator.allocate(layout)?;
        let raw_ptr = unsafe { ptr.as_ptr::<LeafNode<N, TKey, TValue>>() };
        debug_assert!(raw_ptr.is_aligned(), "LeafNode must be aligned to at least 2 bytes");
        unsafe { raw_ptr.write(leaf) };
        let numeric_ptr = raw_ptr as usize;
        Ok(Self {
            numeric_ptr: numeric_ptr,
            phantom: PhantomData,
        })
    }

    /// Boxes an [`InternalNode`] by allocating memory for it and properly tagging it.
    ///
    /// # Errors
    ///
    /// Returns [`AllocationError`] if the allocation fails.
    pub fn box_internal(
        allocator: &mut impl Allocator,
        internal: InternalNode<N, TKey, TValue>,
    ) -> Result<Self, AllocationError> {
        const {
            assert!(
                align_of::<InternalNode<N, TKey, TValue>>() >= 2,
                "InternalNode must be aligned to at least 2 bytes"
            );
        }
        let layout = Layout::new::<InternalNode<N, TKey, TValue>>();
        let ptr = allocator.allocate(layout)?;
        let raw_ptr = unsafe { ptr.as_ptr::<InternalNode<N, TKey, TValue>>() };
        debug_assert!(raw_ptr.is_aligned(), "InternalNode must be aligned to at least 2 bytes");
        unsafe { raw_ptr.write(internal) };
        let numeric_ptr = (raw_ptr as usize) | 1;
        Ok(Self {
            numeric_ptr: numeric_ptr,
            phantom: PhantomData,
        })
    }

    /// Checks if the [`NodeTaggedPtr`] points to a [`LeafNode`] or [`InternalNode`].
    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        (self.numeric_ptr & 1) == 0
    }

    /// Checks if the [`NodeTaggedPtr`] is null.
    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.numeric_ptr == 0
    }

    /// Turns [`NodeTaggedPtr`] into a reference to a [`LeafNode`].
    ///
    /// # Safety
    ///
    /// The call is safe only if [`Self::is_leaf()`] returns `true`.
    #[inline(always)]
    pub const unsafe fn as_leaf(&self) -> &LeafNode<N, TKey, TValue> {
        unsafe { &*(self.raw_ptr() as *const LeafNode<N, TKey, TValue>) }
    }

    /// Turns [`NodeTaggedPtr`] into a mutable reference to a [`LeafNode`].
    ///
    /// # Safety
    ///
    /// The call is safe only if [`Self::is_leaf()`] returns `true`.
    #[inline(always)]
    pub const unsafe fn as_leaf_mut(&mut self) -> &mut LeafNode<N, TKey, TValue> {
        unsafe { &mut *(self.raw_ptr().cast()) }
    }

    /// Turns [`NodeTaggedPtr`] into a reference to an [`InternalNode`].
    ///
    /// # Safety
    ///
    /// The call is safe only if [`Self::is_leaf()`] returns `false`.
    #[inline(always)]
    pub const unsafe fn as_internal(&self) -> &InternalNode<N, TKey, TValue> {
        unsafe { &*(self.raw_ptr().cast()) }
    }

    /// Turns [`NodeTaggedPtr`] into a mutable reference to an [`InternalNode`].
    ///
    /// # Safety
    ///
    /// The call is safe only if [`Self::is_leaf()`] returns `false`.
    #[inline(always)]
    pub const unsafe fn as_internal_mut(&mut self) -> &mut InternalNode<N, TKey, TValue> {
        unsafe { &mut *(self.raw_ptr().cast()) }
    }

    /// Returns a reference to the [`NodeData`] of the node. This is more efficient
    /// than unwrapping the pointer and taking internally stored [`NodeData`].
    #[inline(always)]
    pub const fn node_data(&self) -> &NodeData<N, TKey, TValue> {
        unsafe { &*(self.raw_ptr().cast()) }
    }

    /// Returns a mutable reference to the [`NodeData`] of the node. This is more efficient
    /// than unwrapping the pointer and taking internally stored [`NodeData`].
    #[inline(always)]
    pub const fn node_data_mut(&mut self) -> &mut NodeData<N, TKey, TValue> {
        unsafe { &mut *(self.raw_ptr().cast()) }
    }

    #[inline(always)]
    const fn raw_ptr(&self) -> *mut () {
        (self.numeric_ptr & !1) as *mut ()
    }
}

pub fn deallocate_recursive<const N: usize, TKey, TValue>(
    ptr: &mut NodeTaggedPtr<N, TKey, TValue>,
    allocator: &mut impl Allocator,
) {
    unsafe {
        if ptr.is_leaf() {
            let leaf_ptr = ptr.as_leaf_mut();
            core::ptr::drop_in_place(leaf_ptr);
            let layout = Layout::new::<LeafNode<N, TKey, TValue>>();
            let aptr = allocator.convert_raw_ptr(leaf_ptr);
            aptr.deallocate(layout);
        } else {
            let internal_ptr = ptr.as_internal_mut();
            for edge in internal_ptr.edges_mut().as_mut_slice() {
                deallocate_recursive(edge, allocator);
            }
            core::ptr::drop_in_place(internal_ptr);
            let layout = Layout::new::<InternalNode<N, TKey, TValue>>();
            let aptr = allocator.convert_raw_ptr(internal_ptr);
            aptr.deallocate(layout);
        }
    }
}
