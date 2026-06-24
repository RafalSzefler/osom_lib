#![cfg(all(feature = "std", feature = "serde"))]

use osom_lib_test_helpers::deserialize::{deserialize_json, serialize_json};
use rstest::rstest;
use serde::Deserialize;
use serde::Serialize;

use osom_lib_hash_tables::abseil::defaults::StdAbseilHashTable;
use osom_lib_hash_tables::bytell::defaults::StdBytellHashTable;
use osom_lib_hash_tables::defaults::StdDefaultHashTable;
use osom_lib_hash_tables::traits::MutableHashTable;
use osom_lib_primitives::length::Length;

fn make_length(value: u32) -> Length {
    Length::try_from_u32(value).unwrap()
}

const EXPECTED1: &str = "{\"5\":\"test\",\"-3\":\"foo\"}";
const EXPECTED2: &str = "{\"-3\":\"foo\",\"5\":\"test\"}";

fn build_map<TMap: MutableHashTable<i32, String>>(builder: impl FnOnce() -> TMap) -> TMap {
    let mut map = builder();
    map.insert(5, "test".to_string());
    map.insert(-3, "foo".to_string());
    map
}

#[rstest]
#[case::abseil(|| StdAbseilHashTable::new())]
#[case::bytell(|| StdBytellHashTable::new())]
#[case::default(|| StdDefaultHashTable::new())]
fn test_serialization<TMap: MutableHashTable<i32, String> + Serialize>(#[case] builder: impl FnOnce() -> TMap) {
    let map = build_map(builder);
    let result = serialize_json(&map).unwrap();
    assert!(
        result == EXPECTED1 || result == EXPECTED2,
        "Expected either [{EXPECTED1}] or [{EXPECTED2}], but got [{result}]"
    );
}

fn infer<T>() -> T {
    panic!("Should not be called");
}

#[rstest]
#[case::abseil(infer::<StdAbseilHashTable<i32, String>>)]
#[case::bytell(infer::<StdBytellHashTable<i32, String>>)]
#[case::default(infer::<StdDefaultHashTable<i32, String>>)]
fn test_deserialization<'de, TMap: MutableHashTable<i32, String> + Deserialize<'de>>(
    #[case] _infer: impl FnOnce() -> TMap,
) {
    let map: TMap = deserialize_json(EXPECTED1).unwrap();
    assert_eq!(map.length(), make_length(2));
    assert_eq!(map.get(&5).unwrap(), "test");
    assert_eq!(map.get(&-3).unwrap(), "foo");
}
