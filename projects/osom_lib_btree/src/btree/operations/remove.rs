#![allow(clippy::needless_return)]
use core::{borrow::Borrow, ops::DerefMut};

use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::{kvp::KVP, length::Length};

use crate::btree::{BTree, BTreeConfig, node_ptr::BTreeNodePtr};

use super::helpers;

impl<TKey, TValue, TConfig> BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    const MIN_KVP_LEN: usize = (TConfig::CHILDREN_COUNT - 1) / 2;

    /// Removes a key-value pair from the [`BTree`].
    ///
    /// Returns the removed key-value pair if the key was present, otherwise `None`.
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if self.data.is_null() {
            return None;
        }

        self.remove_recursive(self.data, key)
    }

    fn remove_recursive<Q>(&mut self, node: BTreeNodePtr<TKey, TValue, TConfig>, key: &Q) -> Option<KVP<TKey, TValue>>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        unsafe {
            match helpers::search_key(node, key) {
                helpers::SearchKeyResult::ExactMatch { index } => {
                    self.total_len = Length::new_unchecked(self.total_len.as_u32() - 1);
                    return Some(if node.is_leaf() {
                        self.remove_leaf_node(node, index)
                    } else {
                        self.remove_internal_node(node, index)
                    });
                }
                helpers::SearchKeyResult::InsertionIndex { index } => {
                    if node.is_leaf() {
                        return None;
                    }

                    let child = node.children_ptr().add(index).read();
                    self.remove_recursive(child, key)
                }
            }
        }
    }

    fn remove_leaf_node<Q>(&mut self, node: BTreeNodePtr<TKey, TValue, TConfig>, index: usize) -> KVP<TKey, TValue>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        unsafe {
            // Remove the (key, value) pair from the given node.
            let key = node.keys_ptr().add(index).read();
            let value = node.values_ptr().add(index).read();
            node.remove_at(index);
            let new_len = node.len() - 1;
            node.len_ptr().write(new_len);

            // We didn't go below the minimum node length, we are done.
            if new_len > Self::MIN_KVP_LEN {
                return KVP { key, value };
            }

            // Otherwise get the parent. If there is no parent, then this is root.
            // Root is special and it can have arbitrarily small number of children, we are done.
            let parent = node.parent_ptr().read();
            if parent.is_null() {
                return KVP { key, value };
            }

            // If we have parent, then try to steal items from left or right sibling.
            let index = Self::get_index_in_parent(node);

            if Self::try_steal_left(node, index) {
                return KVP { key, value };
            }

            if Self::try_steal_right(node, index) {
                return KVP { key, value };
            }

            // If we couldn't steal then we have to merge with a sibling.
            if self.try_merge_with_left(node, index) {
                self.maybe_shrink_root();
                return KVP { key, value };
            }

            if self.try_merge_with_right(node, index) {
                self.maybe_shrink_root();
                return KVP { key, value };
            }

            // B Tree invariants ensure that one of the previous operations succeeds.
            unreachable!("remove_leaf_node: failed to remove (key, value) pair from leaf node");
        }
    }

    /// If the root is an internal node with no keys, replace it with its only child.
    fn maybe_shrink_root(&mut self) {
        unsafe {
            let root = self.data;
            if root.is_null() || root.is_leaf() {
                return;
            }

            if root.len() == 0 {
                let child = root.children_ptr().read();
                debug_check_or_release_hint!(!child.is_null(), "maybe_shrink_root: root has no children");
                child.parent_ptr().write(BTreeNodePtr::NULL);
                self.data = child;
                root.deallocate(self.config.deref_mut());
            }
        }
    }

    fn remove_internal_node<Q>(
        &mut self,
        mut node: BTreeNodePtr<TKey, TValue, TConfig>,
        index: usize,
    ) -> KVP<TKey, TValue>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        unsafe {
            // We swap the found value with either the direct predecessor or successor.
            // That element lives in a child node. We then recursively remove it.
            // That way we will reach a leaf node ultimately.
            loop {
                let child;
                let child_key;
                let child_value;
                let child_index;

                if index > 0 {
                    child = node.children_ptr().add(index - 1).read();
                    debug_check_or_release_hint!(!child.is_null(), "remove_internal_node: child is null");
                    child_index = child.len() - 1;
                    child_key = child.keys_ptr().add(child_index);
                    child_value = child.values_ptr().add(child_index);
                } else {
                    child = node.children_ptr().add(index + 1).read();
                    debug_check_or_release_hint!(!child.is_null(), "remove_internal_node: child is null");
                    child_index = 0;
                    child_key = child.keys_ptr().add(child_index);
                    child_value = child.values_ptr().add(child_index);
                }

                let current_key = node.keys_ptr().add(index);
                let current_value = node.values_ptr().add(index);
                current_key.swap(child_key);
                current_value.swap(child_value);

                if child.is_leaf() {
                    return self.remove_leaf_node(child, child_index);
                }

                node = child;
            }
        }
    }

    fn get_index_in_parent(node: BTreeNodePtr<TKey, TValue, TConfig>) -> usize {
        unsafe {
            debug_check_or_release_hint!(!node.is_null(), "get_index_in_parent: node is null");
            let parent = node.parent_ptr().read();
            debug_check_or_release_hint!(!parent.is_null(), "get_index_in_parent: parent is null");
            let children = parent.children_ptr();
            for i in 0..=parent.len() {
                let child = children.add(i).read();
                if child == node {
                    return i;
                }
            }
            panic!("node is not a child of its parent");
        }
    }

    fn try_steal_left(node: BTreeNodePtr<TKey, TValue, TConfig>, index_in_parent: usize) -> bool {
        unsafe {
            debug_check_or_release_hint!(!node.is_null(), "try_rotate_left: node is null");
            debug_check_or_release_hint!(node.is_leaf(), "try_rotate_left: node is not a leaf node");

            if index_in_parent == 0 {
                return false;
            }

            let parent = node.parent_ptr().read();
            debug_check_or_release_hint!(!parent.is_null(), "try_rotate_left: parent is null");
            let children = parent.children_ptr();
            let left_sibling = children.add(index_in_parent - 1).read();
            debug_check_or_release_hint!(!left_sibling.is_null(), "try_rotate_left: left sibling is null");
            debug_check_or_release_hint!(
                left_sibling.is_leaf(),
                "try_rotate_left: left sibling is not a leaf node"
            );
            if left_sibling.len() <= Self::MIN_KVP_LEN {
                return false;
            }
            let promoted_key = left_sibling.keys_ptr().add(left_sibling.len() - 1).read();
            let promoted_value = left_sibling.values_ptr().add(left_sibling.len() - 1).read();

            left_sibling.remove_at(left_sibling.len() - 1);
            let new_len = left_sibling.len() - 1;
            left_sibling.len_ptr().write(new_len);

            let parent_key = parent.keys_ptr().add(index_in_parent - 1).read();
            let parent_value = parent.values_ptr().add(index_in_parent - 1).read();
            parent.keys_ptr().add(index_in_parent - 1).write(promoted_key);
            parent.values_ptr().add(index_in_parent - 1).write(promoted_value);
            node.insert_at(0, parent_key, parent_value);
            node.len_ptr().write(node.len() + 1);
            return true;
        }
    }

    fn try_steal_right(node: BTreeNodePtr<TKey, TValue, TConfig>, index_in_parent: usize) -> bool {
        unsafe {
            debug_check_or_release_hint!(!node.is_null(), "try_rotate_right: node is null");
            debug_check_or_release_hint!(node.is_leaf(), "try_rotate_right: node is not a leaf node");

            let parent = node.parent_ptr().read();
            debug_check_or_release_hint!(!parent.is_null(), "try_rotate_right: parent is null");

            if index_in_parent == parent.len() {
                return false;
            }

            let children = parent.children_ptr();
            let right_sibling = children.add(index_in_parent + 1).read();
            debug_check_or_release_hint!(!right_sibling.is_null(), "try_rotate_right: right sibling is null");
            debug_check_or_release_hint!(
                right_sibling.is_leaf(),
                "try_rotate_right: right sibling is not a leaf node"
            );
            if right_sibling.len() <= Self::MIN_KVP_LEN {
                return false;
            }

            let promoted_key = right_sibling.keys_ptr().add(0).read();
            let promoted_value = right_sibling.values_ptr().add(0).read();

            right_sibling.remove_at(0);
            let new_len = right_sibling.len() - 1;
            right_sibling.len_ptr().write(new_len);

            let parent_key = parent.keys_ptr().add(index_in_parent).read();
            let parent_value = parent.values_ptr().add(index_in_parent).read();
            parent.keys_ptr().add(index_in_parent).write(promoted_key);
            parent.values_ptr().add(index_in_parent).write(promoted_value);
            node.insert_at(node.len(), parent_key, parent_value);
            node.len_ptr().write(node.len() + 1);
            return true;
        }
    }

    fn try_merge_with_left(&mut self, node: BTreeNodePtr<TKey, TValue, TConfig>, index_in_parent: usize) -> bool {
        unsafe {
            debug_check_or_release_hint!(!node.is_null(), "try_rotate_right: node is null");
            debug_check_or_release_hint!(node.is_leaf(), "try_rotate_right: node is not a leaf node");

            if index_in_parent == 0 {
                return false;
            }

            let parent = node.parent_ptr().read();
            debug_check_or_release_hint!(!parent.is_null(), "try_rotate_right: parent is null");
            let left_sibling = parent.children_ptr().add(index_in_parent - 1).read();
            debug_check_or_release_hint!(!left_sibling.is_null(), "try_rotate_left: left sibling is null");
            debug_check_or_release_hint!(
                left_sibling.is_leaf(),
                "try_rotate_left: left sibling is not a leaf node"
            );

            // Left sibling and current node have at most TConfig::CHILDREN_COUNT - 2 items together.
            // So we add separator in parent to left sibling, then move all items from node
            // to the sibling. And finally we remove separator from parent and remove node,
            // and set the left sibling as the new child. Node is then deallocated.
            let left_len = left_sibling.len();
            let node_len = node.len();
            let separator_key = parent.keys_ptr().add(index_in_parent - 1).read();
            let separator_value = parent.values_ptr().add(index_in_parent - 1).read();
            left_sibling.insert_at(left_len, separator_key, separator_value);
            left_sibling
                .keys_ptr()
                .add(left_len + 1)
                .copy_from_nonoverlapping(node.keys_ptr(), node_len);
            left_sibling
                .values_ptr()
                .add(left_len + 1)
                .copy_from_nonoverlapping(node.values_ptr(), node_len);
            left_sibling.len_ptr().write(left_len + node_len + 1);
            parent.remove_at(index_in_parent - 1);
            parent.remove_child_at(index_in_parent);
            parent.len_ptr().write(parent.len() - 1);
            node.deallocate(self.config.deref_mut());
            return true;
        }
    }

    fn try_merge_with_right(&mut self, node: BTreeNodePtr<TKey, TValue, TConfig>, index_in_parent: usize) -> bool {
        unsafe {
            debug_check_or_release_hint!(!node.is_null(), "try_rotate_right: node is null");
            debug_check_or_release_hint!(node.is_leaf(), "try_rotate_right: node is not a leaf node");
            let parent = node.parent_ptr().read();
            debug_check_or_release_hint!(!parent.is_null(), "try_rotate_right: parent is null");

            if index_in_parent == parent.len() {
                return false;
            }

            let right_sibling = parent.children_ptr().add(index_in_parent + 1).read();
            debug_check_or_release_hint!(!right_sibling.is_null(), "try_rotate_right: right sibling is null");
            debug_check_or_release_hint!(
                right_sibling.is_leaf(),
                "try_rotate_right: right sibling is not a leaf node"
            );

            // Merge the right sibling into `node`, mirroring `try_merge_with_left`.
            let node_len = node.len();
            let right_len = right_sibling.len();
            let separator_key = parent.keys_ptr().add(index_in_parent).read();
            let separator_value = parent.values_ptr().add(index_in_parent).read();
            node.insert_at(node_len, separator_key, separator_value);
            node.keys_ptr()
                .add(node_len + 1)
                .copy_from_nonoverlapping(right_sibling.keys_ptr(), right_len);
            node.values_ptr()
                .add(node_len + 1)
                .copy_from_nonoverlapping(right_sibling.values_ptr(), right_len);
            node.len_ptr().write(node_len + right_len + 1);
            parent.remove_at(index_in_parent);
            parent.remove_child_at(index_in_parent + 1);
            parent.len_ptr().write(parent.len() - 1);
            right_sibling.deallocate(self.config.deref_mut());
            return true;
        }
    }
}
