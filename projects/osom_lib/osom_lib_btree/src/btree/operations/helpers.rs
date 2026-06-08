use core::{borrow::Borrow, cmp::Ordering};

use osom_lib_macros::debug_check_or_release_hint;

use crate::btree::{BTreeConfig, node_ptr::BTreeNodePtr};

pub enum SearchKeyResult {
    /// Key already present at `index`.
    ExactMatch { index: usize },

    /// Insertion index for `key`. This is the index
    /// where the key should be inserted to maintain the order.
    InsertionIndex { index: usize },
}

pub unsafe fn search_key<TKey, TValue, TConfig, Q>(node: BTreeNodePtr<TKey, TValue, TConfig>, key: &Q) -> SearchKeyResult
where
    TKey: Ord,
    Q: Ord + ?Sized,
    TKey: Borrow<Q>,
    TConfig: BTreeConfig,
{
    const {
        assert!(
            TConfig::CHILDREN_COUNT <= 32,
            "BTreeConfig::CHILDREN_COUNT must be less than or equal to 32 for seq scan to be acceptable."
        );
    }

    debug_check_or_release_hint!(!node.is_null(), "BTree::search_node: node is null");

    unsafe {
        let keys = node.keys_ptr();
        let len = node.len();

        for index in 0..len {
            let stored_key = keys.add(index).as_ref_unchecked();
            let comparison = stored_key.borrow().cmp(key);
            if comparison == Ordering::Equal {
                return SearchKeyResult::ExactMatch { index };
            }
            if comparison == Ordering::Greater {
                return SearchKeyResult::InsertionIndex { index };
            }
        }

        SearchKeyResult::InsertionIndex { index: len }
    }
}
