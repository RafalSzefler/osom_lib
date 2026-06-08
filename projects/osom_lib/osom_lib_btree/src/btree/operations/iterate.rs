#![allow(clippy::needless_return)]

use osom_lib_arrays::fixed_array::InlineFixedArray;
use osom_lib_arrays::traits::MutableArray;
use osom_lib_primitives::kvp::KVP;

use crate::btree::{BTree, BTreeConfig, node_ptr::BTreeNodePtr};

struct Frame<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    node: BTreeNodePtr<TKey, TValue, TConfig>,
    index: usize,
}

const MAX_TREE_DEPTH: usize = 70;

struct BTreeIterator<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    stack: InlineFixedArray<MAX_TREE_DEPTH, Frame<TKey, TValue, TConfig>>,
}

impl<TKey, TValue, TConfig> BTreeIterator<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    pub fn new(tree: &BTree<TKey, TValue, TConfig>) -> Self {
        const {
            assert!(
                TConfig::CHILDREN_COUNT >= 4,
                "BTreeConfig::CHILDREN_COUNT must be greater or equal to 4"
            );
        }

        // With CHILDREN_COUNT at least 4, there is no way that
        // the tree ever reaches the max depth MAX_TREE_DEPTH.
        // This means that the iterator takes around ~1kb of
        // stack space, but avoids recursion entirely.

        let mut iter = Self {
            stack: InlineFixedArray::new(),
        };
        if !tree.data.is_null() {
            iter.push_left(tree.data);
        }
        iter
    }

    pub fn push_left(&mut self, mut node: BTreeNodePtr<TKey, TValue, TConfig>) {
        unsafe {
            loop {
                self.stack
                    .try_push(Frame { node, index: 0 })
                    .expect("[BTreeIterator] failed to push frame to stack");

                let child = node.children_ptr().read();

                if child.is_null() {
                    break;
                }

                node = child;
            }
        }
    }
}

impl<TKey, TValue, TConfig> Iterator for BTreeIterator<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    type Item = KVP<*mut TKey, *mut TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                let stack_slice = self.stack.as_mut();
                let top_frame = stack_slice.last_mut()?;

                let node_len = top_frame.node.len();
                if top_frame.index < node_len {
                    let i = top_frame.index;
                    top_frame.index += 1;

                    let key = top_frame.node.keys_ptr().add(i);
                    let value = top_frame.node.values_ptr().add(i);
                    let item = KVP { key, value };

                    if !top_frame.node.is_leaf() {
                        let next_child = top_frame.node.children_ptr().add(i + 1).read();
                        self.push_left(next_child);
                    }

                    return Some(item);
                }

                self.stack
                    .try_pop()
                    .expect("[BTreeIterator] failed to pop frame from stack");
            }
        }
    }
}

impl<TKey, TValue, TConfig> BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    /// Iterates over the tree in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = KVP<&TKey, &TValue>> {
        BTreeIterator::new(self).map(|kvp| unsafe {
            let (key, value) = kvp.unpack();
            KVP {
                key: key.as_ref_unchecked(),
                value: value.as_ref_unchecked(),
            }
        })
    }

    /// Iterates over the tree in ascending order.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = KVP<&TKey, &mut TValue>> {
        BTreeIterator::new(self).map(|kvp| unsafe {
            let (key, value) = kvp.unpack();
            KVP {
                key: key.as_ref_unchecked(),
                value: value.as_mut_unchecked(),
            }
        })
    }
}
