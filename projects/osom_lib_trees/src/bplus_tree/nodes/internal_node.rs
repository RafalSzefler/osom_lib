#![allow(dead_code, unused_variables)]

use osom_lib_arrays::DoubleFixedArray;

use super::{NodeData, NodeTaggedPtr};

#[repr(C)]
pub struct InternalNode<const N: usize, TKey, TValue> {
    data: NodeData<N, TKey, TValue>,

    /// Keys of the node.
    edges: DoubleFixedArray<NodeTaggedPtr<N, TKey, TValue>, N, 1>,
}

impl<const N: usize, TKey, TValue> InternalNode<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: NodeData::new(),
            edges: DoubleFixedArray::new(),
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
    pub const fn edges(&self) -> &DoubleFixedArray<NodeTaggedPtr<N, TKey, TValue>, N, 1> {
        &self.edges
    }

    #[inline(always)]
    pub const fn edges_mut(&mut self) -> &mut DoubleFixedArray<NodeTaggedPtr<N, TKey, TValue>, N, 1> {
        &mut self.edges
    }
}
