#![allow(dead_code, unused_variables)]

use core::{marker::PhantomData, mem::MaybeUninit};

use osom_lib_primitives::ConstSumArray;

use crate::traits::Compare;

pub struct NodeData<const N: usize, TKey, TValue> {
    /// Pointer to the parent node. If this is not null, then it has to be
    /// an internal node.
    pub parent: NodePtr<N, TKey, TValue>,

    /// Keys of the node.
    pub keys: [MaybeUninit<TKey>; N],

    /// Index of the node in the parent node. The following must hold:
    /// `parent.edges[parent_idx] == self`.
    pub parent_idx: u16,

    /// Number of keys in the node.
    pub len: u16,
}

impl<const N: usize, TKey, TValue> Drop for NodeData<N, TKey, TValue> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<TKey>() {
            for i in 0..self.len {
                unsafe {
                    let key = self.keys[i as usize].as_mut_ptr();
                    core::ptr::drop_in_place(key);
                }
            }
        }
    }
}

impl<const N: usize, TKey, TValue> NodeData<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            parent: NodePtr::null(),
            keys: uninit_array(),
            parent_idx: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub const fn keys(&self) -> &[TKey] {
        unsafe { core::slice::from_raw_parts(self.keys.as_ptr().cast(), self.len as usize) }
    }

    #[inline(always)]
    pub const fn keys_mut(&mut self) -> &mut [TKey] {
        unsafe { core::slice::from_raw_parts_mut(self.keys.as_mut_ptr().cast(), self.len as usize) }
    }

    pub fn first_greater_or_equal_position<K>(&self, key: &K) -> usize
    where
        TKey: Compare<K>,
    {
        let slice = self.keys();
        let mut idx = 0;
        loop {
            if idx == slice.len() {
                break idx;
            }

            let current_key = &slice[idx];
            if current_key.is_greater_or_equal(key) {
                break idx;
            }

            idx += 1;
        }
    }
}

pub struct LeafNode<const N: usize, TKey, TValue> {
    pub data: NodeData<N, TKey, TValue>,
    pub prev: *mut LeafNode<N, TKey, TValue>,
    pub next: *mut LeafNode<N, TKey, TValue>,
    pub values: [MaybeUninit<TValue>; N],
}

pub struct LeafRef<const N: usize, TKey, TValue> {
    pub leaf: *mut LeafNode<N, TKey, TValue>,
    pub idx: u32,
}

impl<const N: usize, TKey, TValue> LeafRef<N, TKey, TValue> {
    #[inline(always)]
    pub const fn clone(&self) -> Self {
        Self {
            leaf: self.leaf,
            idx: self.idx,
        }
    }

    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            leaf: core::ptr::null_mut(),
            idx: 0,
        }
    }

    #[inline(always)]
    pub fn equals(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.leaf, other.leaf) && self.idx == other.idx
    }

    pub const fn next(&self) -> Self {
        if self.is_null() {
            return self.clone();
        }

        let leaf = unsafe { &*self.leaf };
        if self.idx == leaf.data.len as u32 {
            return Self {
                leaf: leaf.next,
                idx: 0,
            };
        }

        LeafRef {
            leaf: self.leaf,
            idx: self.idx + 1,
        }
    }

    pub const fn prev(&self) -> Self {
        if self.is_null() {
            return self.clone();
        }

        let leaf = unsafe { &*self.leaf };
        if self.idx == 0 {
            let prev = leaf.prev;
            if prev.is_null() {
                return Self { leaf: prev, idx: 0 };
            }

            let prev_ref = unsafe { &*prev };
            return Self {
                leaf: prev,
                idx: prev_ref.data.len as u32 - 1,
            };
        }

        Self {
            leaf: self.leaf,
            idx: self.idx - 1,
        }
    }

    pub const fn is_null(&self) -> bool {
        self.leaf.is_null()
    }
}

impl<const N: usize, TKey, TValue> Drop for LeafNode<N, TKey, TValue> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<TValue>() {
            for i in 0..self.data.len {
                unsafe {
                    let ptr = self.values[i as usize].as_mut_ptr();
                    core::ptr::drop_in_place(ptr);
                }
            }
        }
    }
}

impl<const N: usize, TKey, TValue> LeafNode<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: NodeData::new(),
            values: uninit_array(),
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        }
    }
}

pub struct InternalNode<const N: usize, TKey, TValue> {
    pub data: NodeData<N, TKey, TValue>,
    pub edges: ConstSumArray<N, 1, MaybeUninit<NodePtr<N, TKey, TValue>>>,
}

impl<const N: usize, TKey, TValue> Drop for InternalNode<N, TKey, TValue> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<NodePtr<N, TKey, TValue>>() {
            let slice = self.edges.as_mut_slice();
            for i in 0..=self.data.len {
                unsafe {
                    let edge = slice[i as usize].as_mut_ptr();
                    core::ptr::drop_in_place(edge);
                }
            }
        }
    }
}

impl<const N: usize, TKey, TValue> InternalNode<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: NodeData::new(),
            edges: ConstSumArray::new(uninit_array(), uninit_array()),
        }
    }
}

/// A pointer to a node in the B+ tree. It is a tagged pointer that
/// takes advantage of the fact that each node's alignment has to be
/// at least 2. It utilizes the last bit to distinguish between leaf
/// and internal nodes.
#[derive(Debug)]
#[repr(transparent)]
pub struct NodePtr<const N: usize, TKey, TValue> {
    numeric_ptr: usize,
    phantom: PhantomData<(TKey, TValue)>,
}

impl<const N: usize, TKey, TValue> NodePtr<N, TKey, TValue> {
    const fn validate() {
        const {
            assert!(N >= 4, "N must be at least 4");
            assert!(N < 32767, "N must be less than 32767");
            assert!(
                align_of::<InternalNode<N, TKey, TValue>>() >= 2,
                "Invalid InternalNode alignment"
            );
            assert!(
                align_of::<LeafNode<N, TKey, TValue>>() >= 2,
                "Invalid LeafNode alignment"
            );
        }
    }

    #[inline(always)]
    pub fn from_internal(ptr: *mut InternalNode<N, TKey, TValue>) -> Self {
        Self::validate();
        let numeric_ptr = ptr as usize;
        debug_assert!(numeric_ptr % 2 == 0, "Invalid alignment of InternalNode ptr");

        Self {
            numeric_ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn from_leaf(ptr: *mut LeafNode<N, TKey, TValue>) -> Self {
        Self::validate();
        let numeric_ptr = ptr as usize;
        debug_assert!(numeric_ptr % 2 == 0, "Invalid alignment of LeafNode ptr");
        let numeric_ptr = numeric_ptr | 1;

        Self {
            numeric_ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn null() -> Self {
        Self::validate();
        Self {
            numeric_ptr: 0,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.numeric_ptr == 0
    }

    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        self.numeric_ptr & 1 == 1
    }

    #[inline(always)]
    const fn as_ptr(&self) -> *mut () {
        let numeric_ptr = self.numeric_ptr & !1;
        numeric_ptr as *mut ()
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub unsafe fn as_internal(&self) -> &mut InternalNode<N, TKey, TValue> {
        unsafe { &mut *self.as_ptr().cast() }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub unsafe fn as_leaf(&self) -> &mut LeafNode<N, TKey, TValue> {
        unsafe { &mut *self.as_ptr().cast() }
    }
}

#[inline(always)]
const fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}
