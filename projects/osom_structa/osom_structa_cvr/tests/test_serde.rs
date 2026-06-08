#![cfg(all(feature = "serde", feature = "std"))]
use osom_lib_arrays::traits::MutableArray as _;
use osom_lib_primitives::length::Length;
use rstest::rstest;

use osom_structa_cvr::serde::CVRSeed;
use osom_structa_cvr::{
    CVRBool, CVRFloat, CVRInt, CVRString,
    std::{StdCVR, StdCVRArray, StdCVRDeserializeContext, StdCVRObject},
};
use priv_osom_lib_tests::deserialize::{deserialize_json, deserialize_json_with_seed, serialize_json};

#[test]
fn test_null_serialization() {
    let cvr = StdCVR::Null;
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, "null");
}

#[test]
fn test_bool_serialization() {
    let cvr = StdCVR::Bool(CVRBool::new(true));
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, "true");

    let cvr = StdCVR::Bool(CVRBool::new(false));
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, "false");
}

#[test]
fn test_bool_deserialization() {
    let result1 = deserialize_json::<CVRBool>("true").unwrap();
    assert_eq!(result1, CVRBool::new(true));

    let result2 = deserialize_json::<CVRBool>("false").unwrap();
    assert_eq!(result2, CVRBool::new(false));
}

#[rstest]
#[case("11")]
#[case("\"test\"")]
#[case("null")]
fn test_bool_deserialization_error(#[case] input: &str) {
    let _ = deserialize_json::<CVRBool>(input).unwrap_err();
}

#[rstest]
#[case(0, "0")]
#[case(1, "1")]
#[case(-1, "-1")]
#[case(5123, "5123")]
#[case(-5123, "-5123")]
fn test_int_serialization(#[case] input: i128, #[case] expected: &str) {
    let cvr = StdCVR::Int(CVRInt::new(input));
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("5123", 5123)]
#[case("-5123", -5123)]
fn test_int_deserialization(#[case] input: &str, #[case] expected: i128) {
    let result = deserialize_json::<CVRInt>(input).unwrap();
    assert_eq!(result, CVRInt::new(expected));
}

#[rstest]
#[case("true")]
#[case("false")]
#[case("\"test\"")]
#[case("null")]
fn test_int_deserialization_error(#[case] input: &str) {
    let _ = deserialize_json::<CVRInt>(input).unwrap_err();
}

/// Fraction deserialization and serialization tests are special,
/// since we do fuzzy comparison.
#[rstest]
#[case(0.0)]
#[case(5.0/7.0)]
#[case(-13.0)]
fn test_fraction_de_se_rialization(#[case] input: f64) {
    use priv_osom_lib_tests::deserialize::deserialize_json_with_seed;

    const THRESHOLD: f64 = 1e-10;

    let fraction = CVRFloat::new(input);
    let cvr = StdCVR::Float(fraction);
    let result = serialize_json(&cvr).unwrap();

    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = CVRSeed { context: &mut context };
    let parsed_cvr = deserialize_json_with_seed(&result, seed).unwrap();

    let parsed_fraction = parsed_cvr.as_fraction().unwrap();

    let parsed_f64: f64 = parsed_fraction.inner();
    let expected_f64: f64 = fraction.try_into().unwrap();
    assert!((parsed_f64 - expected_f64).abs() < THRESHOLD);
}

#[rstest]
#[case("test", "\"test\"")]
#[case("test with spaces", "\"test with spaces\"")]
#[case("test with \"quotes\"", "\"test with \\\"quotes\\\"\"")]
#[case("test with \\backslash", "\"test with \\\\backslash\"")]
#[case("test with \n newline", "\"test with \\n newline\"")]
#[case("test with \r carriage return", "\"test with \\r carriage return\"")]
#[case("test with \t tab", "\"test with \\t tab\"")]
#[case("test with \u{0000} null", "\"test with \\u0000 null\"")]
fn test_string_serialization(#[case] input: &str, #[case] expected: &str) {
    let cvr = StdCVR::String(CVRString::new(input).unwrap());
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("11")]
#[case("true")]
#[case("null")]
fn test_string_deserialization_error(#[case] input: &str) {
    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = osom_structa_cvr::serde::CVRStringSeed { context: &mut context };
    let _ = deserialize_json_with_seed(input, seed).unwrap_err();
}

#[test]
fn test_array_serialization() {
    let mut array = StdCVRArray::new();
    array.inner_mut().try_push(StdCVR::Null).unwrap();
    array.inner_mut().try_push(StdCVR::Int(CVRInt::new(1))).unwrap();
    array.inner_mut().try_push(StdCVR::Null).unwrap();
    array
        .inner_mut()
        .try_push(StdCVR::String(CVRString::new("test").unwrap()))
        .unwrap();
    array.inner_mut().try_push(StdCVR::Bool(CVRBool::new(true))).unwrap();
    let cvr = StdCVR::Array(array);
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, "[null,1,null,\"test\",true]");
}

#[test]
fn test_array_deserialization() {
    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = osom_structa_cvr::serde::CVRArraySeed { context: &mut context };
    let result = deserialize_json_with_seed("[null,1,null,\"test\",true]", seed).unwrap();
    let arr = result.inner_ref().as_ref();
    assert_eq!(arr.len(), 5);
    assert_eq!(arr[0], StdCVR::Null);
    assert_eq!(arr[1], StdCVR::Int(CVRInt::new(1)));
    assert_eq!(arr[2], StdCVR::Null);
    assert_eq!(arr[3], StdCVR::String(CVRString::new("test").unwrap()));
    assert_eq!(arr[4], StdCVR::Bool(CVRBool::new(true)));
}

#[rstest]
#[case("11")]
#[case("\"test\"")]
#[case("true")]
#[case("null")]
fn test_array_deserialization_error(#[case] input: &str) {
    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = osom_structa_cvr::serde::CVRArraySeed { context: &mut context };
    let _ = deserialize_json_with_seed(input, seed);
}

#[test]
fn test_object_serialization() {
    let mut object = StdCVRObject::new();
    object
        .try_insert(CVRString::new("test").unwrap(), StdCVR::Null)
        .unwrap();
    object
        .try_insert(CVRString::new("test2").unwrap(), StdCVR::Int(CVRInt::new(2)))
        .unwrap();

    let inner_object = StdCVRObject::new();
    object
        .try_insert(CVRString::new("test4").unwrap(), StdCVR::Object(inner_object))
        .unwrap();

    let mut inner_array = StdCVRArray::new();
    inner_array.inner_mut().try_push(StdCVR::Null).unwrap();
    inner_array.inner_mut().try_push(StdCVR::Int(CVRInt::new(3))).unwrap();
    object
        .try_insert(CVRString::new("test5").unwrap(), StdCVR::Array(inner_array))
        .unwrap();

    let cvr = StdCVR::Object(object);
    let result = serialize_json(&cvr).unwrap();
    assert_eq!(result, "{\"test\":null,\"test2\":2,\"test4\":{},\"test5\":[null,3]}");
}

#[test]
fn test_object_deserialization() {
    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = osom_structa_cvr::serde::CVRObjectSeed { context: &mut context };
    let result =
        deserialize_json_with_seed("{\"test\":null,\"test2\":2,\"test4\":{},\"test5\":[null,3]}", seed).unwrap();
    assert_eq!(result.len().as_usize(), 4);
    assert_eq!(result.get("test").unwrap(), &StdCVR::Null);
    assert_eq!(result.get("test2").unwrap(), &StdCVR::Int(CVRInt::new(2)));
    assert_eq!(result.get("test4").unwrap(), &StdCVR::Object(StdCVRObject::new()));

    let mut inner_array = StdCVRArray::with_capacity(Length::try_from_u32(2).unwrap()).unwrap();
    inner_array.inner_mut().try_push(StdCVR::Null).unwrap();
    inner_array.inner_mut().try_push(StdCVR::Int(CVRInt::new(3))).unwrap();
    assert_eq!(result.get("test5").unwrap(), &StdCVR::Array(inner_array));
}
