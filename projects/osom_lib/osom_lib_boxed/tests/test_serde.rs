#![cfg(all(feature = "std", feature = "serde"))]

use priv_osom_lib_tests::deserialize::{deserialize_json, serialize_json};

use core::ops::Deref;
use osom_lib_boxed::std::StdCBox;
use rstest::rstest;

#[rstest]
#[case(0, "0")]
#[case(1, "1")]
#[case(-1, "-1")]
#[case(5123, "5123")]
fn test_serialization(#[case] value: i32, #[case] expected: &str) {
    let box_ = StdCBox::new(value).unwrap();
    let result = serialize_json(&box_).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("5123", 5123)]
fn test_deserialization(#[case] value: &str, #[case] expected: i32) {
    let result: StdCBox<i32> = deserialize_json(value).unwrap();
    assert_eq!(result.deref(), &expected);
}
