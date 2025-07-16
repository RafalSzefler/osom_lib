#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use osom_lib_alloc::Allocator;
use osom_lib_primitives::Length;

use crate::traits::{Compare, TreeError, TreeTryInsertResult};

use super::nodes::{LeafNode, NodeTaggedPtr};
use super::{BPlusTree, helpers};

impl<TKey, TValue, TAllocator, const NODE_CAPACITY: usize> BPlusTree<TKey, TValue, TAllocator, NODE_CAPACITY>
where
    TKey: Clone + Ord,
    TAllocator: Allocator,
{
    pub(super) fn internal_try_insert(&mut self, key: TKey, value: TValue) -> Result<TreeTryInsertResult, TreeError> {
        if self.root.is_null() {
            let mut leaf = LeafNode::new();
            leaf.data_mut().keys_mut().push(key).unwrap();
            leaf.values_mut().push(value).unwrap();
            self.root = NodeTaggedPtr::box_leaf(&mut self.allocator, leaf)?;
            self.len = Length::ONE;
            return Ok(TreeTryInsertResult::Inserted);
        }

        let mut current_node = &mut self.root;
        while !current_node.is_leaf() {
            let internal_node = unsafe { current_node.as_internal_mut() };
            let index = helpers::scan_node(&key, internal_node.data().keys());
            current_node = &mut internal_node.edges_mut()[index as usize];
        }

        let leaf = unsafe { current_node.as_leaf_mut() };

        let index = helpers::scan_node(&key, leaf.data().keys());
        {
            if index < leaf.data().keys().len().value() {
                let leaf_key = &leaf.data().keys()[index as usize];
                if leaf_key.is_equal(&key) {
                    return Ok(TreeTryInsertResult::AlreadyExists);
                }
            }

            if index > 0 {
                let leaf_key = &leaf.data().keys()[(index - 1) as usize];
                if leaf_key.is_equal(&key) {
                    return Ok(TreeTryInsertResult::AlreadyExists);
                }
            }
        }

        if self.len.value() as usize >= Self::MAX_SIZE {
            return Err(TreeError::TreeTooBig);
        }
        self.len.add(1).unwrap();

        if leaf.data().keys().len().value() < NODE_CAPACITY as i32 {
            leaf.data_mut().keys_mut().push(key).unwrap();
            leaf.values_mut().push(value).unwrap();
            let index = helpers::move_last_into_order(leaf.data_mut().keys_mut());
            helpers::move_last_into_position(leaf.values_mut(), index);
        } else {
            todo!("Implement leaf node split")
        }

        Ok(TreeTryInsertResult::Inserted)
    }
}
