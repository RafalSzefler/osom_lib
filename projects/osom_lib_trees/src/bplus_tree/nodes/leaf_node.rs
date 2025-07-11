use osom_lib_arrays::FixedArray;

use super::NodeData;

#[repr(C)]
pub struct LeafNode<const N: usize, TKey, TValue> {
    data: NodeData<N, TKey, TValue>,

    values: FixedArray<TValue, N>,
}
