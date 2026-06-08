use core::hash::Hash;

use crate::btree::{BTree, BTreeConfig};

impl<TKey, TValue, TConfig> PartialEq for BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TValue: PartialEq,
    TConfig: BTreeConfig,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut right_iter = other.iter();

        // We compare elements item by item, taking advantage of the fact
        // that B-trees are ordered.
        for left_kvp in self.iter() {
            let right_kvp = right_iter
                .next()
                .expect("[PartialEq] left and right iterators should have the same length");

            if left_kvp.unpack() != right_kvp.unpack() {
                return false;
            }
        }

        true
    }
}

impl<TKey, TValue, TConfig> Eq for BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TValue: Eq,
    TConfig: BTreeConfig,
{
}

impl<TKey, TValue, TConfig> Hash for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + Hash,
    TValue: Hash,
    TConfig: BTreeConfig,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for kvp in self.iter() {
            kvp.key.hash(state);
            kvp.value.hash(state);
        }
    }
}
