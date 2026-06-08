#![cfg(feature = "std")]

//! B-tree edge-case tests: ordering, capacity boundaries, and Drop for keys/values.

use core::sync::atomic::{AtomicU32, Ordering};

use osom_lib_btree::btree::inspect;
use osom_lib_btree::std::StdBTree;
use osom_lib_primitives::length::Length;
use std::collections::BTreeSet;
use std::sync::Arc;

mod common;
use common::{lcg_next, make_len};

mod drop_tracker {
    use super::*;

    #[derive(Clone)]
    pub struct DropTag {
        pub id: i32,
        counter: Arc<AtomicU32>,
    }

    impl DropTag {
        pub fn new(id: i32, counter: &Arc<AtomicU32>) -> Self {
            Self {
                id,
                counter: counter.clone(),
            }
        }
    }

    impl Drop for DropTag {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl PartialEq for DropTag {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for DropTag {}

    impl PartialOrd for DropTag {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for DropTag {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.id.cmp(&other.id)
        }
    }

    impl core::fmt::Debug for DropTag {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("DropTag").field("id", &self.id).finish()
        }
    }

    pub fn counter() -> Arc<AtomicU32> {
        Arc::new(AtomicU32::new(0))
    }

    pub fn drops(counter: &Arc<AtomicU32>) -> u32 {
        counter.load(Ordering::SeqCst)
    }
}

mod scale {
    pub const BULK: usize = if cfg!(miri) { 250 } else { 2_000 };
    pub const STRESS: usize = if cfg!(miri) { 400 } else { 4_000 };
    /// Default `CHILDREN_COUNT` is 17 => at most 16 keys per node.
    pub const REBALANCE_WINDOW: usize = if cfg!(miri) { 40 } else { 80 };
}

fn tag(counter: &Arc<AtomicU32>, id: i32) -> drop_tracker::DropTag {
    drop_tracker::DropTag::new(id, counter)
}

fn assert_sorted_keys(tree: &StdBTree<i32, i32>) {
    let mut it = tree.iter();
    let Some(previous) = it.next() else {
        return;
    };
    let mut previous_key = previous.key;

    for kvp in it {
        let key = kvp.key;
        assert!(previous_key < key, "keys out of order: {previous_key} then {key}");
        previous_key = key;
    }
}

fn assert_sorted_i32_keys<TValue>(tree: &StdBTree<i32, TValue>) {
    let mut previous: Option<i32> = None;
    for kvp in tree.iter() {
        let key = *kvp.key;
        if let Some(prev) = previous {
            assert!(prev < key, "keys out of order: {prev} then {key}");
        }
        previous = Some(key);
    }
}

fn collect_keys(tree: &StdBTree<i32, i32>) -> Vec<i32> {
    tree.iter().map(|kvp| *kvp.key).collect()
}

/// Sorted order, no duplicate keys, and `len` matches iterator length.
fn assert_tree_integrity(tree: &StdBTree<i32, i32>) {
    assert_sorted_keys(tree);
    let keys = collect_keys(tree);
    assert_eq!(keys.len(), tree.len().as_usize(), "len vs iter count");
    let unique: BTreeSet<_> = keys.iter().copied().collect();
    assert_eq!(unique.len(), keys.len(), "duplicate keys in tree");
}

fn fill_pseudo_random(tree: &mut StdBTree<i32, i32>, steps: i32, modulus: i32, state: &mut u64) {
    for _ in 0..steps {
        *state = lcg_next(*state);
        let key = (*state % modulus as u64) as i32;
        tree.try_insert(key, key).unwrap();
    }
}

// --- empty / minimal trees ---

#[test]
fn empty_tree_len_and_queries() {
    let tree: StdBTree<i32, i32> = StdBTree::new();
    assert_eq!(tree.len(), Length::ZERO);
    assert!(tree.get(&0).is_none());
    assert_eq!(tree.iter().count(), 0);
}

#[test]
fn drop_empty_tree_does_not_panic() {
    let tree: StdBTree<drop_tracker::DropTag, drop_tracker::DropTag> = StdBTree::new();
    drop(tree);
}

#[test]
fn single_element_insert_get_and_drop() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();
    tree.try_insert(42, tag(&counter, 99)).unwrap();
    assert_eq!(tree.len(), Length::ONE);
    assert_eq!(tree.get(&42).unwrap().value.id, 99);
    assert_eq!(drop_tracker::drops(&counter), 0);

    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 1);
}

// --- insert / overwrite ---

#[test]
fn insert_overwrite_drops_previous_value_when_returned_pair_is_dropped() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();

    assert!(tree.try_insert(1, tag(&counter, 100)).unwrap().is_none());
    assert_eq!(drop_tracker::drops(&counter), 0);

    let old = tree
        .try_insert(1, tag(&counter, 200))
        .unwrap()
        .expect("overwrite should return previous value");
    assert_eq!(old.id, 100);
    assert_eq!(drop_tracker::drops(&counter), 0);

    drop(old);
    assert_eq!(drop_tracker::drops(&counter), 1);

    assert_eq!(*tree.get(&1).unwrap().key, 1);
    assert_eq!(tree.get(&1).unwrap().value.id, 200);
    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 2);
}

#[test]
fn repeated_overwrites_drop_each_replaced_value() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();

    for id in 0..20 {
        let _ = tree.try_insert(7, tag(&counter, id));
    }
    // 19 overwrites => 19 drops; one value still in the tree.
    assert_eq!(drop_tracker::drops(&counter), 19);
    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 20);
}

#[test]
fn insert_discards_duplicate_key_on_overwrite() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();
    tree.try_insert(1, tag(&counter, 10)).unwrap();

    let _ = tree.try_insert(1, tag(&counter, 20)).unwrap();
    // The discarded key (id 1) is dropped when try_insert returns.
    assert_eq!(drop_tracker::drops(&counter), 1);
    assert_eq!(tree.get(&1).unwrap().value.id, 20);
}

// --- Drop: whole tree ---

#[test]
fn drop_tree_drops_all_stored_values() {
    let counter = drop_tracker::counter();
    let n = scale::BULK;
    {
        let mut tree = StdBTree::new();
        for key in 0..n as i32 {
            tree.try_insert(key, tag(&counter, key)).unwrap();
        }
        assert_eq!(drop_tracker::drops(&counter), 0);
    }
    assert_eq!(drop_tracker::drops(&counter), n as u32);
}

#[test]
fn drop_tree_drops_keys_when_key_type_needs_drop() {
    let counter = drop_tracker::counter();
    let n = 50;
    {
        let mut tree = StdBTree::new();
        for key in 0..n {
            tree.try_insert(tag(&counter, key), key).unwrap();
        }
        assert_eq!(drop_tracker::drops(&counter), 0);
    }
    assert_eq!(drop_tracker::drops(&counter), n as u32);
}

#[test]
fn drop_tree_with_strings_does_not_panic() {
    let mut tree = StdBTree::new();
    for idx in 0..100 {
        tree.try_insert(idx, format!("payload-{idx}")).unwrap();
    }
    drop(tree);
}

// --- ordering / integrity ---

#[test]
fn iteration_sorted_after_reverse_sequential_inserts() {
    let mut tree = StdBTree::new();
    for key in (0..scale::BULK as i32).rev() {
        tree.try_insert(key, key).unwrap();
    }
    assert_sorted_keys(&tree);
    let keys = collect_keys(&tree);
    assert_eq!(keys.len(), scale::BULK);
    for (idx, key) in keys.iter().enumerate() {
        assert_eq!(*key, idx as i32);
    }
}

#[test]
fn insert_around_node_capacity_boundary_stays_sorted() {
    let mut tree = StdBTree::new();
    let n = scale::REBALANCE_WINDOW as i32;

    for key in 0..n {
        tree.try_insert(key, key).unwrap();
        assert_sorted_keys(&tree);
    }

    for key in (0..n).rev() {
        tree.try_insert(key + n, key + n).unwrap();
        assert_sorted_keys(&tree);
    }

    assert_eq!(tree.len(), make_len(scale::REBALANCE_WINDOW * 2));
}

#[test]
fn ascending_insert_with_drop_tags_then_drop_tree() {
    let counter = drop_tracker::counter();
    let n = scale::STRESS as i32;
    let mut tree = StdBTree::new();

    for key in 0..n {
        tree.try_insert(key, tag(&counter, key)).unwrap();
    }
    assert_eq!(tree.len(), make_len(scale::STRESS));
    assert_sorted_i32_keys(&tree);

    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), scale::STRESS as u32);
}

#[test]
fn descending_insert_stays_sorted() {
    let mut tree = StdBTree::new();
    let n = scale::STRESS as i32;

    for key in (0..n).rev() {
        tree.try_insert(key, key).unwrap();
    }
    assert_sorted_keys(&tree);
    assert_eq!(collect_keys(&tree), (0..n).collect::<Vec<_>>());
}

#[test]
fn pseudo_random_keys_stay_sorted() {
    let mut tree = StdBTree::new();
    let n = scale::STRESS as i32;
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;

    for _ in 0..n {
        state = lcg_next(state);
        let key = (state % n as u64) as i32;
        tree.try_insert(key, key).unwrap();
    }
    assert_sorted_keys(&tree);
}

#[test]
fn get_and_iter_do_not_drop_stored_values() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();
    for key in 0..20 {
        tree.try_insert(key, tag(&counter, key)).unwrap();
    }

    for key in 0..20 {
        let _ = tree.get(&key);
    }
    for kvp in tree.iter() {
        let _ = kvp;
    }
    assert_eq!(drop_tracker::drops(&counter), 0);
    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 20);
}

#[test]
fn iter_mut_borrows_values_without_dropping() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();
    for key in 0..10 {
        tree.try_insert(key, tag(&counter, key * 10)).unwrap();
    }

    for kvp in tree.iter_mut() {
        let key = kvp.key;
        assert_eq!(kvp.value.id, key * 10);
    }
    assert_eq!(drop_tracker::drops(&counter), 0);
    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 10);
}

#[test]
fn overwrite_via_iter_mut_drops_replaced_value() {
    let counter = drop_tracker::counter();
    let mut tree = StdBTree::new();
    tree.try_insert(0, tag(&counter, 0)).unwrap();

    for kvp in tree.iter_mut() {
        let _old = core::mem::replace(kvp.value, tag(&counter, 1));
    }
    assert_eq!(drop_tracker::drops(&counter), 1);
    assert_eq!(tree.get(&0).unwrap().value.id, 1);
    drop(tree);
    assert_eq!(drop_tracker::drops(&counter), 2);
}

// --- try_insert_or_update ---

#[test]
fn try_insert_or_update_inserts_then_updates_in_place() {
    let mut tree = StdBTree::new();

    let value = tree
        .try_insert_or_update(7, || 100, |_| panic!("updater must not run on first insert"))
        .unwrap();
    assert_eq!(*value, 100);
    assert_eq!(tree.len(), Length::ONE);

    let value = tree
        .try_insert_or_update(7, || panic!("adder must not run on update"), |slot| *slot = 200)
        .unwrap();
    assert_eq!(*value, 200);
    assert_eq!(tree.len(), Length::ONE);
    assert_eq!(*tree.get(&7).unwrap().value, 200);
}

// --- split / overwrite regressions ---

#[test]
fn overwrite_after_many_splits_does_not_duplicate_keys() {
    let n = scale::STRESS as i32;
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut tree = StdBTree::new();

    fill_pseudo_random(&mut tree, n, n, &mut state);
    let len_after_fill = tree.len();

    for _ in 0..n {
        state = lcg_next(state);
        let key = (state % n as u64) as i32;
        if tree.get(&key).is_none() {
            continue;
        }
        let len_before = tree.len();
        tree.try_insert(key, key.wrapping_add(1)).unwrap();
        assert_eq!(tree.len(), len_before, "overwrite of {key} changed len");
    }

    assert_eq!(tree.len(), len_after_fill);
    assert_tree_integrity(&tree);
}

#[test]
fn overwrite_after_partial_random_fill_matches_promoted_separator_regression() {
    let n = scale::STRESS as i32;
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut tree = StdBTree::new();

    // First corruption appeared at step 1086 with the default STRESS seed.
    fill_pseudo_random(&mut tree, 1086, n, &mut state);
    let len = tree.len();

    state = lcg_next(state);
    let key = (state % n as u64) as i32;
    assert!(tree.get(&key).is_some());
    tree.try_insert(key, key.wrapping_mul(2)).unwrap();
    assert_eq!(tree.len(), len);
    assert_eq!(*tree.get(&key).unwrap().value, key.wrapping_mul(2));
    assert_eq!(collect_keys(&tree).iter().filter(|k| **k == key).count(), 1);
    assert_tree_integrity(&tree);
}

#[test]
fn repeated_overwrite_same_key_stays_sorted_with_splits() {
    let mut tree = StdBTree::new();
    for key in 0..scale::REBALANCE_WINDOW as i32 {
        tree.try_insert(key, key).unwrap();
    }

    let len = tree.len();
    for round in 0..scale::REBALANCE_WINDOW {
        let key = (round % scale::REBALANCE_WINDOW) as i32;
        tree.try_insert(key, key.wrapping_add(round as i32)).unwrap();
        assert_eq!(tree.len(), len);
        assert_tree_integrity(&tree);
    }
}

// --- weird insert patterns ---

#[test]
fn stride_inserts_along_node_capacity_stay_sorted() {
    const STRIDE: i32 = 16;
    let mut tree = StdBTree::new();
    let limit = scale::STRESS as i32;

    for key in (0..limit).step_by(STRIDE as usize) {
        tree.try_insert(key, key).unwrap();
        assert_tree_integrity(&tree);
    }

    assert_eq!(tree.len(), make_len((limit as usize).div_ceil(STRIDE as usize)));
}

#[test]
fn zigzag_inserts_from_both_ends_stay_sorted() {
    let mut tree = StdBTree::new();
    let n = scale::BULK as i32;

    for offset in 0..n {
        tree.try_insert(offset, offset).unwrap();
        tree.try_insert(n - 1 - offset, n - 1 - offset).unwrap();
        assert_tree_integrity(&tree);
    }

    assert_eq!(collect_keys(&tree), (0..n).collect::<Vec<_>>());
}

#[test]
fn alternating_two_keys_with_splits_stays_sorted() {
    let mut tree = StdBTree::new();
    let rounds = scale::STRESS;

    for round in 0..rounds {
        let key = (round % 2) as i32;
        tree.try_insert(key, round as i32).unwrap();
        assert_tree_integrity(&tree);
    }

    assert_eq!(tree.len(), make_len(2));
    assert_eq!(*tree.get(&0).unwrap().value, (rounds - 2) as i32);
    assert_eq!(*tree.get(&1).unwrap().value, (rounds - 1) as i32);
}

#[test]
fn mixed_negative_positive_random_inserts_stay_sorted() {
    let mut tree = StdBTree::new();
    let span = scale::STRESS as i32;
    let mut state: u64 = 0x0123_4567_89AB_CDEF;

    for _ in 0..scale::STRESS {
        state = lcg_next(state);
        let key = (state % (span as u64 * 2 + 1)) as i32 - span;
        tree.try_insert(key, key).unwrap();
    }

    assert_tree_integrity(&tree);
}

#[test]
fn shuffle_insert_via_lcg_permutation_stays_sorted() {
    let n = scale::BULK as i32;
    let mut tree = StdBTree::new();
    let mut state: u64 = 0xFACEFEED;

    for step in 0..n {
        state = lcg_next(state);
        let key = (state % n as u64) as i32;
        tree.try_insert(key, step).unwrap();
        assert_tree_integrity(&tree);
    }

    assert!(tree.len().as_usize() <= n as usize);
    assert_tree_integrity(&tree);
}

// --- removal edge cases ---

#[test]
fn remove_missing_key_is_noop() {
    let mut tree = StdBTree::new();
    for key in 0..20 {
        tree.try_insert(key, key).unwrap();
    }

    let len = tree.len();
    assert!(tree.remove(&99).is_none());
    assert!(tree.remove(&-1).is_none());
    assert_eq!(tree.len(), len);
    assert_tree_integrity(&tree);
}

#[test]
fn remove_sequential_in_reverse_order_then_refill() {
    let n = scale::REBALANCE_WINDOW as i32;
    let mut tree = StdBTree::new();

    for key in 0..n {
        tree.try_insert(key, key).unwrap();
    }
    assert_tree_integrity(&tree);

    for key in (0..n).rev() {
        let removed = tree.remove(&key).unwrap();
        assert_eq!(removed.unpack().0, key);
        assert_tree_integrity(&tree);
    }
    assert_eq!(tree.len(), Length::ZERO);
    assert_eq!(tree.iter().count(), 0);

    for key in 0..n {
        tree.try_insert(key, key * 10).unwrap();
    }
    assert_tree_integrity(&tree);
    for key in 0..n {
        assert_eq!(*tree.get(&key).unwrap().value, key * 10);
    }
}

#[test]
fn interleaved_insert_and_overwrite_stays_sorted() {
    let n = scale::REBALANCE_WINDOW as i32;
    let mut tree = StdBTree::new();
    let mut state: u64 = 0xC0FFEE;

    for step in 0..n * 4 {
        state = lcg_next(state);
        let key = (state % n as u64) as i32;
        if step % 2 == 0 {
            tree.try_insert(key, step).unwrap();
        } else {
            let _ = tree.try_insert(key, step);
        }
        assert_tree_integrity(&tree);
    }
}

#[test]
fn removing_all_keys_empties_tree_and_drops_height() {
    let mut tree = StdBTree::new();
    let count = inspect::get_max_kvp_count(&tree) + 2;

    for key in 0..count as i32 {
        tree.try_insert(key, key).unwrap();
    }
    let height_after_inserts = inspect::get_height(&tree);
    assert!(height_after_inserts >= 2, "expected tree taller than a single node");

    for key in 0..count as i32 {
        let _ = tree.remove(&key).unwrap();
    }

    assert_eq!(tree.len(), Length::ZERO);
    assert!(tree.get(&0).is_none());
    assert!(inspect::get_height(&tree) <= height_after_inserts);
}

// --- iterator / lookup consistency ---

#[test]
fn get_matches_iter_for_every_stored_key() {
    let mut tree = StdBTree::new();
    for key in 0..scale::BULK as i32 {
        tree.try_insert(key, key * 3).unwrap();
    }

    for kvp in tree.iter() {
        let key = *kvp.key;
        let via_get = tree.get(&key).expect("get missing key present in iter");
        assert_eq!(*via_get.key, key);
        assert_eq!(*via_get.value, key * 3);
    }
}

#[test]
fn iter_mut_updates_visible_to_get() {
    let mut tree = StdBTree::new();
    for key in 0..30 {
        tree.try_insert(key, key).unwrap();
    }

    for kvp in tree.iter_mut() {
        *kvp.value *= 2;
    }

    for key in 0..30 {
        assert_eq!(*tree.get(&key).unwrap().value, key * 2);
    }
    assert_tree_integrity(&tree);
}
