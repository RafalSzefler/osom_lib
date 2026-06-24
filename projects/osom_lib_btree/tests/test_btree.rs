#![cfg(feature = "std")]
use std::{convert::Infallible, hash::Hasher};

use osom_lib_btree::btree::inspect;
use osom_lib_btree::std::StdBTree;
use osom_lib_primitives::length::Length;

mod common;
pub use common::{lcg_next, make_len};
use osom_lib_try_clone::TryClone;

#[test]
pub fn test_btree_simply() {
    let mut btree = StdBTree::new();
    assert_eq!(btree.len(), Length::ZERO);

    btree.try_insert(1, "test".to_string()).unwrap();
    assert_eq!(btree.len(), Length::ONE);
    assert_eq!(btree.get(&1).unwrap().value, &"test");

    btree.try_insert(2, "test2".to_string()).unwrap();
    assert_eq!(btree.len(), make_len(2));
    assert_eq!(btree.get(&2).unwrap().value, &"test2");

    btree.try_insert(1, "test overwrite".to_string()).unwrap();
    assert_eq!(btree.len(), make_len(2));
    assert_eq!(btree.get(&1).unwrap().value, &"test overwrite");

    for idx in 3..100 {
        let text = format!("test idx: {}", idx);
        btree.try_insert(idx, text.clone()).unwrap();
        assert_eq!(btree.len(), make_len(idx));
        assert_eq!(btree.get(&idx).unwrap().value, &text);
    }

    for idx in 0..3 {
        let text = format!("test idx: {}", idx);
        btree.try_insert(idx, text.clone()).unwrap();
    }

    for (order, kvp) in btree.iter().enumerate() {
        let stored_key = *kvp.key;
        let stored_value = kvp.value;
        assert_eq!(stored_key, order);
        assert_eq!(stored_value, &format!("test idx: {}", order));
    }
}

#[test]
pub fn test_btree_massive() {
    const ITERATIONS: usize = cfg_select! {
        miri => 5_000,  // miri is significantly slower than pure Rust
        _ => 2_000_000
    };

    let mut btree = StdBTree::new();
    assert_eq!(btree.len(), Length::ZERO);

    let mut current: u64 = cfg_select! {
        miri => {
            0x76543210abcdef00
        },
        _ => {
            // Miri doesn't support SystemTime.
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }
    };

    for idx in 0..ITERATIONS {
        current = lcg_next(current);
        btree.try_insert(current, idx).unwrap();
    }

    let mut iterator = btree.iter();
    let mut first = *iterator.next().unwrap().key;

    for item in iterator {
        let current_key = *item.key;
        assert!(first < current_key, "keys are not in order: {first} < {current_key}");
        first = current_key;
    }
}

#[test]
pub fn test_btree_with_removal() {
    let mut btree = StdBTree::new();
    assert_eq!(inspect::get_height(&btree), 0);
    for idx in -5..5 {
        btree.try_insert(idx, format!("test {idx}").to_string()).unwrap();
        assert_eq!(inspect::get_height(&btree), 1);
    }

    assert_eq!(btree.len(), make_len(10));
    assert_eq!(inspect::get_height(&btree), 1);

    let (key, value) = btree.remove(&-1).unwrap().unpack();
    assert_eq!(btree.len(), make_len(9));
    assert_eq!(key, -1);
    assert_eq!(value, "test -1");
    assert_eq!(inspect::get_height(&btree), 1);

    assert!(btree.get(&-1).is_none());
    assert!(btree.remove(&-1).is_none());
    assert_eq!(inspect::get_height(&btree), 1);

    btree.try_insert(-1, "test XYZ".to_string()).unwrap();
    assert_eq!(btree.len(), make_len(10));
    assert_eq!(btree.get(&-1).unwrap().value, &"test XYZ");

    let (key, value) = btree.remove(&-1).unwrap().unpack();
    assert_eq!(key, -1);
    assert_eq!(value, "test XYZ");

    assert!(btree.get(&-1).is_none());
    assert!(btree.remove(&-1).is_none());
    assert_eq!(inspect::get_height(&btree), 1);
}

#[test]
pub fn test_btree_height_increase() {
    let mut btree = StdBTree::new();
    assert_eq!(inspect::get_max_kvp_count(&btree), 15);
    assert_eq!(inspect::get_min_kvp_count(&btree), 7);
    assert_eq!(inspect::get_height(&btree), 0);
    let mut current: u64 = 0x1234567890abcdef;
    for idx in 0..15 {
        current = lcg_next(current);
        btree.try_insert(idx, current).unwrap();
        assert_eq!(inspect::get_height(&btree), 1);
    }

    current = lcg_next(current);
    btree.try_insert(23, current).unwrap();
    assert_eq!(inspect::get_height(&btree), 2);

    let _ = btree.remove(&23).unwrap();
    assert_eq!(inspect::get_height(&btree), 2);
    for idx in 0..15 {
        let _ = btree.remove(&idx).unwrap();
        assert_eq!(inspect::get_height(&btree), 1);
    }
}

#[derive(Debug)]
struct TryClonableString(String);

impl TryClone for TryClonableString {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self(self.0.clone()))
    }
}

impl From<&str> for TryClonableString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for TryClonableString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T: AsRef<str>> PartialEq<T> for TryClonableString {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl Eq for TryClonableString {}

#[test]
pub fn test_btree_clone() {
    let mut btree = StdBTree::<i32, TryClonableString>::new();
    btree.try_insert(1, "test".into()).unwrap();
    btree.try_insert(2, "test2".into()).unwrap();
    btree.try_insert(3, "test3".into()).unwrap();
    assert_eq!(btree.len(), make_len(3));
    assert_eq!(btree.get(&1).unwrap().value, &"test");
    assert_eq!(btree.get(&2).unwrap().value, &"test2");
    assert_eq!(btree.get(&3).unwrap().value, &"test3");

    let btree2 = btree.clone();
    assert_eq!(btree2.len(), make_len(3));
    assert_eq!(btree2.get(&1).unwrap().value, &"test");
    assert_eq!(btree2.get(&2).unwrap().value, &"test2");
    assert_eq!(btree2.get(&3).unwrap().value, &"test3");

    drop(btree);

    assert_eq!(btree2.len(), make_len(3));
    assert_eq!(btree2.get(&1).unwrap().value, &"test");
    assert_eq!(btree2.get(&2).unwrap().value, &"test2");
    assert_eq!(btree2.get(&3).unwrap().value, &"test3");
}

#[test]
pub fn test_btree_eq() {
    let mut btree = StdBTree::<i32, i32>::new();
    btree.try_insert(1, 0).unwrap();
    btree.try_insert(2, 0).unwrap();
    btree.try_insert(3, 0).unwrap();
    assert_eq!(btree.len(), make_len(3));

    let mut btree2 = StdBTree::<i32, i32>::new();
    assert_eq!(btree2.len(), make_len(0));
    assert_ne!(btree, btree2);
    assert_ne!(calculate_hash(&btree), calculate_hash(&btree2));

    btree2.try_insert(1, 0).unwrap();
    btree2.try_insert(2, 0).unwrap();

    assert_ne!(btree, btree2);
    assert_ne!(calculate_hash(&btree), calculate_hash(&btree2));

    btree2.try_insert(3, 5).unwrap();
    assert_ne!(btree, btree2);
    assert_ne!(calculate_hash(&btree), calculate_hash(&btree2));

    let _ = btree2.remove(&1).unwrap();
    assert_ne!(btree, btree2);
    assert_ne!(calculate_hash(&btree), calculate_hash(&btree2));

    btree2.try_insert(1, 0).unwrap();
    btree2.try_insert(3, 0).unwrap();
    assert_eq!(btree, btree2);
    assert_eq!(calculate_hash(&btree), calculate_hash(&btree2));
}

fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
