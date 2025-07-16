use core::ptr::null_mut;

use osom_lib_arrays::FixedArray;

use super::InternalNode;

pub struct NodeData<const N: usize, TKey, TValue> {
    /// Pointer to the parent node. If this is not null, then it has to be
    /// an internal node.
    parent: *mut InternalNode<N, TKey, TValue>,

    /// Keys on the node.
    keys: FixedArray<TKey, N>,
}

impl<const N: usize, TKey, TValue> NodeData<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            parent: null_mut(),
            keys: FixedArray::new(),
        }
    }

    #[inline(always)]
    pub const fn get_parent(&self) -> *mut InternalNode<N, TKey, TValue> {
        self.parent
    }

    #[inline(always)]
    pub const fn set_parent(&mut self, parent: *mut InternalNode<N, TKey, TValue>) {
        self.parent = parent;
    }

    #[inline(always)]
    pub const fn keys(&self) -> &FixedArray<TKey, N> {
        &self.keys
    }

    #[inline(always)]
    pub const fn keys_mut(&mut self) -> &mut FixedArray<TKey, N> {
        &mut self.keys
    }
}
