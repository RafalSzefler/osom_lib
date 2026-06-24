use core::borrow::Borrow;

use osom_lib_primitives::kvp::KVP;

use crate::btree::{BTree, BTreeConfig, node_ptr::BTreeNodePtr};

use super::helpers;

impl<TKey, TValue, TConfig> BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    /// Returns the key-value pair with key matching the given `key`.
    /// Return `None` if the key is not present in the [`BTree`].
    pub fn get<Q>(&self, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.search_node(self.data, key)
    }

    #[allow(clippy::unused_self)]
    fn search_node<Q>(&self, mut node: BTreeNodePtr<TKey, TValue, TConfig>, key: &Q) -> Option<KVP<&TKey, &TValue>>
    where
        TKey: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        unsafe {
            loop {
                if node.is_null() {
                    return None;
                }

                let index = match helpers::search_key(node, key) {
                    helpers::SearchKeyResult::ExactMatch { index } => {
                        let key = node.keys_ptr().add(index).as_ref_unchecked();
                        let value = node.values_ptr().add(index).as_ref_unchecked();
                        return Some(KVP { key, value });
                    }
                    helpers::SearchKeyResult::InsertionIndex { index } => index,
                };

                if node.is_leaf() {
                    return None;
                }

                let child = node.children_ptr().add(index).read();
                node = child;
            }
        }
    }
}
