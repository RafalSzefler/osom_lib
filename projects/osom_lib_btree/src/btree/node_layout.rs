use core::{alloc::Layout, marker::PhantomData};

use super::BTreeConfig;

/// In memory layout for a B-tree node.
pub struct BTreeNodeLayout<TKey, TValue, TConfig>
where
    TConfig: BTreeConfig,
{
    /// Offset of the `length` field of the key-value pairs in the node.
    pub len_offset: usize,

    /// Offset of the `values` array in the node.
    pub values_offset: usize,

    /// Offset of the `parent` pointer in the node.
    pub parent_offset: usize,

    /// Offset of the `keys` array in the node.
    pub keys_offset: usize,

    /// Offset of the `children` array in the node. Note that
    /// if it is a leaf node, then it has no children. And
    /// then this becomes an offset to empty space.
    pub children_offset: usize,

    /// Total layout of the node.
    pub total_layout: Layout,

    _phantom: PhantomData<(TKey, TValue, TConfig)>,
}

const fn layout_for<T>() -> Layout {
    const { unsafe { Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()) } }
}

const fn layout_for_array<T>(size: usize) -> Layout {
    let total_size = size_of::<T>() * size;
    unsafe { Layout::from_size_align_unchecked(total_size, align_of::<T>()) }
}

impl<TKey, TValue, TConfig> BTreeNodeLayout<TKey, TValue, TConfig>
where
    TConfig: BTreeConfig,
{
    pub const fn new(is_leaf: bool) -> Self {
        const {
            assert!(
                TConfig::CHILDREN_COUNT >= 4,
                "BTreeConfig::CHILDREN_COUNT must be greater or equal to 4"
            );
            assert!(
                TConfig::CHILDREN_COUNT < 65536,
                "BTreeConfig::CHILDREN_COUNT must be less than 65536"
            );
            assert!(
                size_of::<TKey>() < 65536,
                "size_of::<TKey>() must be less than 65536 bytes"
            );
            assert!(
                size_of::<TValue>() < 65536,
                "size_of::<TValue>() must be less than 65536 bytes"
            );
        }

        let keys_count = TConfig::CHILDREN_COUNT - 1;
        let layout = unsafe { Layout::from_size_align_unchecked(0, 1) };

        let Ok((layout, len_offset)) = layout.extend(layout_for::<usize>()) else {
            panic!("Failed to calculate BTreeNodeLayout for length");
        };

        let Ok((layout, values_offset)) = layout.extend(layout_for_array::<TValue>(keys_count)) else {
            panic!("Failed to calculate BTreeNodeLayout for TValue");
        };

        let Ok((layout, parent_offset)) = layout.extend(layout_for::<*mut u8>()) else {
            panic!("Failed to calculate BTreeNodeLayout for parent");
        };

        let Ok((layout, keys_offset)) = layout.extend(layout_for_array::<TKey>(keys_count)) else {
            panic!("Failed to calculate BTreeNodeLayout for TKey");
        };

        let children_count = if is_leaf { 1 } else { TConfig::CHILDREN_COUNT };

        let Ok((layout, children_offset)) = layout.extend(layout_for_array::<*mut u8>(children_count)) else {
            panic!("Failed to calculate BTreeNodeLayout for edges");
        };

        let total_layout = layout;

        Self {
            len_offset: len_offset,
            keys_offset: keys_offset,
            values_offset: values_offset,
            children_offset: children_offset,
            parent_offset: parent_offset,
            total_layout: total_layout,
            _phantom: PhantomData,
        }
    }
}
