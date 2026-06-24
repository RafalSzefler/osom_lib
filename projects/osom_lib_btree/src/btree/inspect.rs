//! This module holds various helpers for inspecting the internal structure of the B-tree.

use crate::btree::node_ptr::BTreeNodePtr;

use super::{BTree, BTreeConfig};

/// Returns the maximum number of key-value pairs that a single node can hold.
/// This is an odd number.
#[inline(always)]
#[must_use]
pub const fn get_max_kvp_count<TKey, TValue, TConfig>(_: &BTree<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    TConfig::CHILDREN_COUNT - 1
}

/// Returns the minimum number of key-value pairs that a single node can hold.
/// This is typically half of the maximum number of key-value pairs.
#[inline(always)]
#[must_use]
pub const fn get_min_kvp_count<TKey, TValue, TConfig>(_: &BTree<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    (TConfig::CHILDREN_COUNT - 1) / 2
}

/// Returns the height of the B-tree.
///
/// Note: this function scans the entire tree recursively.
#[must_use]
pub fn get_height<TKey, TValue, TConfig>(btree: &BTree<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    if btree.data.is_null() {
        return 0;
    }

    get_height_recursive(btree.data)
}

fn get_height_recursive<TKey, TValue, TConfig>(node: BTreeNodePtr<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    unsafe {
        let mut current_height = 1;

        if node.is_leaf() {
            return current_height;
        }

        for index in 0..=node.len() {
            let child = node.children_ptr().add(index).read();
            if child.is_null() {
                continue;
            }
            let height = get_height_recursive(child);
            if height > current_height {
                current_height = height;
            }
        }

        current_height + 1
    }
}

/// Calculates the memory usage of the B-tree. Note that it does not inspect keys and values
/// internally. Meaning that if they store pointers internally, the memory they point to is not included.
///
/// The memory usage is calculated by summing the allocated memory size of each internal node.
///
/// It also does not include the memory usage of the `btree` struct itself,
/// i.e. `size_of(btree)` is not included.
#[must_use]
pub fn calculate_memory_usage<TKey, TValue, TConfig>(btree: &BTree<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    if btree.data.is_null() {
        return 0;
    }

    calculate_memory_usage_recursive(btree.data)
}

fn calculate_memory_usage_recursive<TKey, TValue, TConfig>(node: BTreeNodePtr<TKey, TValue, TConfig>) -> usize
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    let mut current_size = node.memory_layout().size();
    unsafe {
        if !node.is_leaf() {
            for index in 0..=node.len() {
                let child = node.children_ptr().add(index).read();
                if child.is_null() {
                    continue;
                }
                current_size += calculate_memory_usage_recursive(child);
            }
        }
    }

    current_size
}
