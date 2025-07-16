#![allow(clippy::mut_from_ref)]
use core::marker::PhantomData;

use osom_lib_alloc::{AllocationError, Allocator};

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
        let ptr = allocator.allocate_for_type::<LeafNode<N, TKey, TValue>>()?;
        debug_assert!(ptr.is_aligned(), "LeafNode must be aligned to at least 2 bytes");
        unsafe { ptr.write(leaf) };
        let numeric_ptr = ptr.as_ptr() as usize;
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
        let ptr = allocator.allocate_for_type::<InternalNode<N, TKey, TValue>>()?;
        debug_assert!(ptr.is_aligned(), "InternalNode must be aligned to at least 2 bytes");
        unsafe { ptr.write(internal) };
        let numeric_ptr = ptr.as_ptr() as usize | 1;
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
    pub const unsafe fn as_leaf_mut(&self) -> &mut LeafNode<N, TKey, TValue> {
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
    pub const unsafe fn as_internal_mut(&self) -> &mut InternalNode<N, TKey, TValue> {
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
