#![cfg(all(feature = "std", feature = "serde"))]

use osom_lib_test_helpers::deserialize::{deserialize_json, serialize_json};

use osom_lib_primitives::length::Length;
use rstest::rstest;
use serde::{Deserialize, Serialize};

use osom_lib_arrays::fixed_array::InlineFixedArray;
use osom_lib_arrays::std::{StdDynamicArray, StdFixedArray, StdInlineDynamicArray};
use osom_lib_arrays::traits::MutableArray;

const TEST_ARRAY: &[i32] = &[0, -15, 3, 47, 12612];
const TEST_JSON: &str = "[0,-15,3,47,12612]";

#[rstest]
#[case::dynamic_array(|| StdDynamicArray::new())]
#[case::inline_dynamic_array(|| StdInlineDynamicArray::<3, _>::new())]
#[case::fixed_array(|| StdFixedArray::with_capacity(Length::try_from_usize(10).unwrap()).unwrap())]
#[case::inline_fixed_array(|| InlineFixedArray::<10, _>::new())]
fn test_serialization<TArray: MutableArray<i32> + Serialize>(#[case] builder: impl FnOnce() -> TArray) {
    let mut arr = builder();
    arr.push_slice(TEST_ARRAY);
    let result = serialize_json(&arr).unwrap();
    assert_eq!(result, TEST_JSON);
}

fn infer<T>() -> T {
    panic!("Should not be called");
}

#[rstest]
#[case::dynamic_array(infer::<StdDynamicArray<i32>>)]
#[case::inline_dynamic_array(infer::<StdInlineDynamicArray<3, i32>>)]
#[case::inline_fixed_array(infer::<InlineFixedArray<10, i32>>)]
fn test_deserialization<'de, TArray: MutableArray<i32> + Deserialize<'de>>(#[case] _infer: impl FnOnce() -> TArray) {
    let result: TArray = deserialize_json(TEST_JSON).unwrap();
    assert_eq!(result.as_ref(), TEST_ARRAY);
}
