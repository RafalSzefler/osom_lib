#![cfg(all(feature = "std", feature = "serde"))]
use osom_lib_strings::shared::serde::{CachedStringSeed, StdStringCache};
use osom_lib_strings::std::{StdOwnedString, StdSharedString};
use osom_lib_test_helpers::deserialize::{deserialize_json, serialize_json};
use rstest::rstest;

#[rstest]
#[case("", "\"\"")]
#[case("aBc", "\"aBc\"")]
#[case("a. x ya", "\"a. x ya\"")]
#[case("A \" B", "\"A \\\" B\"")]
fn test_serialization(#[case] input: &str, #[case] expected: &str) {
    let imm = StdSharedString::from_str_slice(input).unwrap();
    osom_lib_test_helpers::deserialize::serialize_json(&imm).unwrap();
    let result = osom_lib_test_helpers::deserialize::serialize_json(&imm).unwrap();
    assert_eq!(&result, expected);
}

#[rstest]
#[case("\"\"", "")]
#[case("\"aBc\"", "aBc")]
#[case("\"a. x ya\"", "a. x ya")]
#[case("\"A \\\" B\"", "A \" B")]
fn test_deserialization(#[case] input: &str, #[case] expected: &str) {
    let result: StdSharedString = osom_lib_test_helpers::deserialize::deserialize_json(input).unwrap();
    assert_eq!(result.as_str(), expected);
}

#[test]
fn test_cached_deserialization() {
    const TEST_JSON1: &str = "\"test\"";
    const TEST_STRING1: &str = "test";
    const TEST_JSON2: &str = "\"test other\"";
    const TEST_STRING2: &str = "test other";

    let mut cache = StdStringCache::default();

    let seed1 = CachedStringSeed::new(&mut cache);
    let result: StdSharedString =
        osom_lib_test_helpers::deserialize::deserialize_json_with_seed(TEST_JSON1, seed1).unwrap();
    assert_eq!(result.as_str(), TEST_STRING1);

    let seed2 = CachedStringSeed::new(&mut cache);
    let result2: StdSharedString =
        osom_lib_test_helpers::deserialize::deserialize_json_with_seed(TEST_JSON1, seed2).unwrap();
    assert_eq!(result2.as_str(), TEST_STRING1);

    assert!(result.ptr_eq(&result2));

    let seed3 = CachedStringSeed::new(&mut cache);
    let result3: StdSharedString =
        osom_lib_test_helpers::deserialize::deserialize_json_with_seed(TEST_JSON2, seed3).unwrap();
    assert_eq!(result3.as_str(), TEST_STRING2);

    assert!(!result.ptr_eq(&result3));
}

#[rstest]
#[case("", "\"\"")]
#[case("aBc", "\"aBc\"")]
#[case("a. x ya", "\"a. x ya\"")]
#[case("A \" B", "\"A \\\" B\"")]
#[case("12345678", "\"12345678\"")]
#[case("123456789", "\"123456789\"")]
fn test_owned_string_serialization(#[case] input: &str, #[case] expected: &str) {
    let owned = StdOwnedString::try_from_str(input).unwrap();
    let result = serialize_json(&owned).unwrap();
    assert_eq!(&result, expected);
}

#[rstest]
#[case("\"\"", "")]
#[case("\"aBc\"", "aBc")]
#[case("\"a. x ya\"", "a. x ya")]
#[case("\"A \\\" B\"", "A \" B")]
#[case("\"12345678\"", "12345678")]
#[case("\"123456789\"", "123456789")]
fn test_owned_string_deserialization(#[case] input: &str, #[case] expected: &str) {
    let result: StdOwnedString = deserialize_json(input).unwrap();
    assert_eq!(result.as_ref(), expected);
}

#[test]
fn test_owned_string_serde_round_trip() {
    let original = StdOwnedString::try_from_str("round trip").unwrap();
    let json = serialize_json(&original).unwrap();
    let restored: StdOwnedString = deserialize_json(&json).unwrap();
    assert_eq!(restored, original);
}
