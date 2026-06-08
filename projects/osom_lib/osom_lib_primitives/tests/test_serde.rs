#![cfg(feature = "serde")]
use priv_osom_lib_tests::deserialize::{deserialize_json, serialize_json};
use rstest::rstest;

use osom_lib_primitives::coption::COption;
use osom_lib_primitives::kvp::KVP;
use osom_lib_primitives::length::Length;
use osom_lib_primitives::power_of_two::{PowerOfTwo32, PowerOfTwo64};

fn length(value: u32) -> Length {
    Length::try_from_u32(value).unwrap()
}

fn power_of_two_32(value: u32) -> PowerOfTwo32 {
    PowerOfTwo32::new(value).unwrap()
}

fn power_of_two_64(value: u64) -> PowerOfTwo64 {
    PowerOfTwo64::new(value).unwrap()
}

#[rstest]
#[case(COption::Some(0), "0")]
#[case(COption::Some(1), "1")]
#[case(COption::Some(-1), "-1")]
#[case(COption::Some(5123), "5123")]
#[case(COption::None, "null")]
fn test_coption_serialization(#[case] value: COption<i32>, #[case] expected: &str) {
    let result = serialize_json(&value).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", COption::Some(0))]
#[case("1", COption::Some(1))]
#[case("-1", COption::Some(-1))]
#[case("5123", COption::Some(5123))]
#[case("null", COption::None)]
fn test_coption_deserialization(#[case] json: &str, #[case] expected: COption<i32>) {
    let result: COption<i32> = deserialize_json(json).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(COption::Some(0))]
#[case(COption::Some(-42))]
#[case(COption::Some(5123))]
#[case(COption::None)]
fn test_coption_round_trip(#[case] value: COption<i32>) {
    let json = serialize_json(&value).unwrap();
    let restored: COption<i32> = deserialize_json(&json).unwrap();
    assert_eq!(restored, value);
}

#[rstest]
#[case(Length::ZERO, "0")]
#[case(Length::ONE, "1")]
#[case(length(42), "42")]
#[case(Length::MAX_LENGTH, "2147481599")]
fn test_length_serialization(#[case] value: Length, #[case] expected: &str) {
    let result = serialize_json(&value).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", Length::ZERO)]
#[case("1", Length::ONE)]
#[case("42", length(42))]
#[case("2147481599", Length::MAX_LENGTH)]
fn test_length_deserialization(#[case] json: &str, #[case] expected: Length) {
    let result: Length = deserialize_json(json).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(Length::ZERO)]
#[case(Length::ONE)]
#[case(length(5123))]
#[case(Length::MAX_LENGTH)]
fn test_length_round_trip(#[case] value: Length) {
    let json = serialize_json(&value).unwrap();
    let restored: Length = deserialize_json(&json).unwrap();
    assert_eq!(restored, value);
}

#[rstest]
#[case(power_of_two_32(0), "0")]
#[case(power_of_two_32(1), "1")]
#[case(power_of_two_32(2), "2")]
#[case(power_of_two_32(1024), "1024")]
fn test_power_of_two_32_serialization(#[case] value: PowerOfTwo32, #[case] expected: &str) {
    let result = serialize_json(&value).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", power_of_two_32(0))]
#[case("1", power_of_two_32(1))]
#[case("2", power_of_two_32(2))]
#[case("1024", power_of_two_32(1024))]
fn test_power_of_two_32_deserialization(#[case] json: &str, #[case] expected: PowerOfTwo32) {
    let result: PowerOfTwo32 = deserialize_json(json).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(power_of_two_32(0))]
#[case(power_of_two_32(1))]
#[case(power_of_two_32(64))]
#[case(power_of_two_32(1 << 20))]
fn test_power_of_two_32_round_trip(#[case] value: PowerOfTwo32) {
    let json = serialize_json(&value).unwrap();
    let restored: PowerOfTwo32 = deserialize_json(&json).unwrap();
    assert_eq!(restored, value);
}

#[rstest]
#[case(power_of_two_64(0), "0")]
#[case(power_of_two_64(1), "1")]
#[case(power_of_two_64(2), "2")]
#[case(power_of_two_64(1 << 32), "4294967296")]
fn test_power_of_two_64_serialization(#[case] value: PowerOfTwo64, #[case] expected: &str) {
    let result = serialize_json(&value).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", power_of_two_64(0))]
#[case("1", power_of_two_64(1))]
#[case("2", power_of_two_64(2))]
#[case("4294967296", power_of_two_64(1 << 32))]
fn test_power_of_two_64_deserialization(#[case] json: &str, #[case] expected: PowerOfTwo64) {
    let result: PowerOfTwo64 = deserialize_json(json).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(power_of_two_64(0))]
#[case(power_of_two_64(1))]
#[case(power_of_two_64(64))]
#[case(power_of_two_64(1 << 40))]
fn test_power_of_two_64_round_trip(#[case] value: PowerOfTwo64) {
    let json = serialize_json(&value).unwrap();
    let restored: PowerOfTwo64 = deserialize_json(&json).unwrap();
    assert_eq!(restored, value);
}

#[rstest]
#[case(KVP { key: 0, value: "test" }, "[0,\"test\"]")]
#[case(KVP { key: 1, value: "foo" }, "[1,\"foo\"]")]
#[case(KVP { key: -1, value: "bar" }, "[-1,\"bar\"]")]
#[case(KVP { key: 5123, value: "baz" }, "[5123,\"baz\"]")]
fn test_kvp_serialization(#[case] value: KVP<i32, &str>, #[case] expected: &str) {
    let result = serialize_json(&value).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("[0,\"test\"]", KVP { key: 0, value: "test" })]
#[case("[1,\"foo\"]", KVP { key: 1, value: "foo" })]
#[case("[-1,\"bar\"]", KVP { key: -1, value: "bar" })]
#[case("[5123,\"baz\"]", KVP { key: 5123, value: "baz" })]
fn test_kvp_deserialization(#[case] json: &str, #[case] expected: KVP<i32, &str>) {
    let result: KVP<i32, &str> = deserialize_json(json).unwrap();
    assert_eq!(result, expected);
}
