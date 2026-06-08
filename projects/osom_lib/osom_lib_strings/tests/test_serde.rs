#![cfg(all(feature = "std", feature = "serde"))]
use osom_lib_strings::immutable::{
    serde::{CachedStringSeed, StdStringCache},
    std::StdImmutableString,
};
use rstest::rstest;

#[rstest]
#[case("", "\"\"")]
#[case("aBc", "\"aBc\"")]
#[case("a. x ya", "\"a. x ya\"")]
#[case("A \" B", "\"A \\\" B\"")]
fn test_serialization(#[case] input: &str, #[case] expected: &str) {
    let imm = StdImmutableString::from_str_slice(input).unwrap();
    priv_osom_lib_tests::deserialize::serialize_json(&imm).unwrap();
    let result = priv_osom_lib_tests::deserialize::serialize_json(&imm).unwrap();
    assert_eq!(&result, expected);
}

#[rstest]
#[case("\"\"", "")]
#[case("\"aBc\"", "aBc")]
#[case("\"a. x ya\"", "a. x ya")]
#[case("\"A \\\" B\"", "A \" B")]
fn test_deserialization(#[case] input: &str, #[case] expected: &str) {
    let result: StdImmutableString = priv_osom_lib_tests::deserialize::deserialize_json(input).unwrap();
    assert_eq!(result.as_str(), expected);
}

#[test]
fn test_cached_deserialization() {
    const TEST_JSON1: &str = "\"test\"";
    const TEST_STRING1: &str = "test";
    const TEST_JSON2: &str = "\"test other\"";
    const TEST_STRING2: &str = "test other";

    let mut cache = StdStringCache::new();

    let seed1 = CachedStringSeed::new(&mut cache);
    let result: StdImmutableString =
        priv_osom_lib_tests::deserialize::deserialize_json_with_seed(TEST_JSON1, seed1).unwrap();
    assert_eq!(result.as_str(), TEST_STRING1);

    let seed2 = CachedStringSeed::new(&mut cache);
    let result2: StdImmutableString =
        priv_osom_lib_tests::deserialize::deserialize_json_with_seed(TEST_JSON1, seed2).unwrap();
    assert_eq!(result2.as_str(), TEST_STRING1);

    assert!(result.ptr_eq(&result2));

    let seed3 = CachedStringSeed::new(&mut cache);
    let result3: StdImmutableString =
        priv_osom_lib_tests::deserialize::deserialize_json_with_seed(TEST_JSON2, seed3).unwrap();
    assert_eq!(result3.as_str(), TEST_STRING2);

    assert!(!result.ptr_eq(&result3));
}
