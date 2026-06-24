#![allow(clippy::needless_return)]
use core::ops::DerefMut;

use osom_lib_primitives::length::Length;

use crate::btree::{BTree, BTreeConfig, node_ptr::BTreeNodePtr};
use crate::errors::BTreeError;

use super::helpers::{self, SearchKeyResult};

impl<TKey, TValue, TConfig> BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    /// Inserts a `(key, adder())` pair into the tree if it doesn't exist, or updates the value
    /// with `update(value)` call.
    ///
    /// # Notes
    ///
    /// * If the key already exists, then this method discards `key`.
    /// * This method guarantees that either `adder` or `updater` will be called, but not both.
    ///
    /// # Errors
    ///
    /// See [`BTreeError`] for details.
    pub fn try_insert_or_update(
        &mut self,
        key: TKey,
        adder: impl FnOnce() -> TValue,
        updater: impl FnOnce(&mut TValue),
    ) -> Result<&mut TValue, BTreeError> {
        unsafe {
            if self.data.is_null() {
                let root = BTreeNodePtr::allocate_leaf_node(self.config.deref_mut())?;
                root.insert_at(0, key, adder());
                root.len_ptr().write(root.len() + 1);
                let new_len = self.total_len.as_u32().unchecked_add(1);
                self.total_len = Length::new_unchecked(new_len);
                self.data = root;
                return Ok(root.values_ptr().add(0).as_mut_unchecked());
            }

            if self.data.is_full() {
                let old_root = self.data;
                let new_root = BTreeNodePtr::allocate_internal_node(self.config.deref_mut())?;
                new_root.children_ptr().write(old_root);
                let (promoted_key, promoted_value, right) = old_root.split(self.config.deref_mut())?;
                new_root.insert_at(0, promoted_key, promoted_value);
                new_root.insert_child_at(1, right);
                new_root.len_ptr().write(new_root.len() + 1);
                right.parent_ptr().write(new_root);
                old_root.parent_ptr().write(new_root);
                self.data = new_root;
            }

            self.try_insert_or_update_non_full(self.data, key, adder, updater)
        }
    }

    /// Inserts a `(key, value)` pair into the tree.
    ///
    /// Returns `None` if the key was not already present, or the previous value if it was.
    ///
    /// # Notes
    ///
    /// This method discards `key` if it already exists.
    ///
    /// # Errors
    ///
    /// See [`BTreeError`] for details.
    pub fn try_insert(&mut self, key: TKey, value: TValue) -> Result<Option<TValue>, BTreeError> {
        let value_ptr = &raw const value;
        let adder = || unsafe { value_ptr.read() };

        let mut result: Option<TValue> = None;
        let updater = |current: &mut TValue| {
            let value = unsafe { value_ptr.read() };
            let old_value = core::mem::replace(current, value);
            result = Some(old_value);
        };

        let _ = self.try_insert_or_update(key, adder, updater)?;
        core::mem::forget(value);
        Ok(result)
    }

    unsafe fn try_insert_or_update_non_full(
        &mut self,
        node: BTreeNodePtr<TKey, TValue, TConfig>,
        key: TKey,
        adder: impl FnOnce() -> TValue,
        updater: impl FnOnce(&mut TValue),
    ) -> Result<&mut TValue, BTreeError> {
        unsafe {
            match helpers::search_key(node, &key) {
                SearchKeyResult::ExactMatch { index } => {
                    let value_mut = node.values_ptr().add(index).as_mut_unchecked();
                    updater(value_mut);
                    return Ok(value_mut);
                }
                SearchKeyResult::InsertionIndex { mut index } => {
                    if node.is_leaf() {
                        if self.total_len == Length::MAX_LENGTH {
                            return Err(BTreeError::TreeSizeOutOfRange);
                        }
                        node.insert_at(index, key, adder());
                        node.len_ptr().write(node.len() + 1);
                        let new_len = self.total_len.as_u32().unchecked_add(1);
                        self.total_len = Length::new_unchecked(new_len);
                        return Ok(node.values_ptr().add(index).as_mut_unchecked());
                    }

                    let mut child = node.children_ptr().add(index).read();
                    if child.is_full() {
                        let (promoted_key, promoted_value, right) = child.split(self.config.deref_mut())?;
                        let equals_promoted_key = key == promoted_key;
                        node.insert_at(index, promoted_key, promoted_value);
                        node.insert_child_at(index + 1, right);
                        node.len_ptr().write(node.len() + 1);
                        right.parent_ptr().write(node);
                        if equals_promoted_key {
                            let value_mut = node.values_ptr().add(index).as_mut_unchecked();
                            updater(value_mut);
                            return Ok(value_mut);
                        }
                        if &key > node.keys_ptr().add(index).as_ref_unchecked() {
                            index += 1;
                        }
                        child = node.children_ptr().add(index).read();
                    }

                    return self.try_insert_or_update_non_full(child, key, adder, updater);
                }
            }
        }
    }
}
