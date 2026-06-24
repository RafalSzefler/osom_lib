#![cfg(all(feature = "std", feature = "serde"))]

use osom_lib_btree::std::StdBTree;
use osom_lib_test_helpers::deserialize::{deserialize_json, serialize_json};

mod common;
use common::make_len;

#[test]
fn test_btree_serialization_int_keys() {
    let mut btree = StdBTree::new();
    btree.try_insert(1, "test".to_string()).unwrap();
    btree.try_insert(3, "test3".to_string()).unwrap();
    btree.try_insert(36, "test".to_string()).unwrap();
    btree.try_insert(2, "test2".to_string()).unwrap();
    btree.try_insert(-7, "test".to_string()).unwrap();
    btree.try_insert(10, "test 10".to_string()).unwrap();
    let result = serialize_json(&btree).unwrap();
    // Note: different order of keys (compare to: test_btree_serialization_string_keys) due to integer sorting of keys.
    assert_eq!(
        result,
        "{\"-7\":\"test\",\"1\":\"test\",\"2\":\"test2\",\"3\":\"test3\",\"10\":\"test 10\",\"36\":\"test\"}"
    );
}

#[test]
fn test_btree_serialization_string_keys() {
    let mut btree = StdBTree::new();
    btree.try_insert("1".to_string(), "test".to_string()).unwrap();
    btree.try_insert("3".to_string(), "test3".to_string()).unwrap();
    btree.try_insert("36".to_string(), "test".to_string()).unwrap();
    btree.try_insert("2".to_string(), "test2".to_string()).unwrap();
    btree.try_insert("-7".to_string(), "test".to_string()).unwrap();
    btree.try_insert("10".to_string(), "test 10".to_string()).unwrap();
    let result = serialize_json(&btree).unwrap();
    // Note: different order of keys (compare to: test_btree_serialization_int_keys) due to lexicographical sorting of keys.
    assert_eq!(
        result,
        "{\"-7\":\"test\",\"1\":\"test\",\"10\":\"test 10\",\"2\":\"test2\",\"3\":\"test3\",\"36\":\"test\"}"
    );
}

#[test]
fn test_deserialization_int_keys() {
    let result: StdBTree<i32, String> = deserialize_json(
        "{\"-7\":\"test\",\"1\":\"test\",\"10\":\"test 10\",\"2\":\"test2\",\"3\":\"test3\",\"36\":\"test\"}",
    )
    .unwrap();
    assert_eq!(result.len(), make_len(6));
    assert_eq!(result.get(&-7).unwrap().value, "test");
    assert_eq!(result.get(&1).unwrap().value, "test");
    assert_eq!(result.get(&10).unwrap().value, "test 10");
    assert_eq!(result.get(&2).unwrap().value, "test2");
    assert_eq!(result.get(&3).unwrap().value, "test3");
    assert_eq!(result.get(&36).unwrap().value, "test");

    let result2: StdBTree<i32, String> = deserialize_json(
        "{\"36\":\"test\", \"-7\":\"test\",\"1\":\"test\",\"10\":\"test 10\",\"2\":\"test2\",\"3\":\"test3\"}",
    )
    .unwrap();
    assert_eq!(result2.len(), make_len(6));
    assert_eq!(result2.get(&-7).unwrap().value, "test");
    assert_eq!(result2.get(&1).unwrap().value, "test");
    assert_eq!(result2.get(&10).unwrap().value, "test 10");
    assert_eq!(result2.get(&2).unwrap().value, "test2");
    assert_eq!(result2.get(&3).unwrap().value, "test3");
    assert_eq!(result2.get(&36).unwrap().value, "test");

    assert_eq!(result, result2);
}
