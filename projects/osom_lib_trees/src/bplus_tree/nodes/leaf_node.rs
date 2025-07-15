use osom_lib_arrays::FixedArray;

use super::NodeData;

#[repr(C)]
pub struct LeafNode<const N: usize, TKey, TValue> {
    data: NodeData<N, TKey, TValue>,
    next: *mut LeafNode<N, TKey, TValue>,
    prev: *mut LeafNode<N, TKey, TValue>,
    values: FixedArray<TValue, N>,
}

impl<const N: usize, TKey, TValue> LeafNode<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: NodeData::new(),
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            values: FixedArray::new(),
        }
    }

    #[inline(always)]
    pub const fn data(&self) -> &NodeData<N, TKey, TValue> {
        &self.data
    }

    #[inline(always)]
    pub const fn data_mut(&mut self) -> &mut NodeData<N, TKey, TValue> {
        &mut self.data
    }

    #[inline(always)]
    pub const fn values(&self) -> &FixedArray<TValue, N> {
        &self.values
    }

    #[inline(always)]
    pub const fn values_mut(&mut self) -> &mut FixedArray<TValue, N> {
        &mut self.values
    }

    #[inline(always)]
    pub const fn get_next(&self) -> *mut LeafNode<N, TKey, TValue> {
        self.next
    }

    #[inline(always)]
    pub const fn get_prev(&self) -> *mut LeafNode<N, TKey, TValue> {
        self.prev
    }

    #[inline(always)]
    pub const fn set_next(&mut self, next: *mut LeafNode<N, TKey, TValue>) {
        self.next = next;
    }

    #[inline(always)]
    pub const fn set_prev(&mut self, prev: *mut LeafNode<N, TKey, TValue>) {
        self.prev = prev;
    }
}
