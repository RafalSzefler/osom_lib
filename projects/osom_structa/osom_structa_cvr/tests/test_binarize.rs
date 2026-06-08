#![cfg(feature = "std")]
use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::traits::MutableArray as _;
use rstest::rstest;

use osom_structa_cvr::{CVR, CVRArray, CVRBool, CVRFloat, CVRInt, CVRObject, CVRString, std::StdCVR, tools::binarize};

fn binarize_to_vec(cvr: &CVR<impl Allocator>) -> Vec<u8> {
    let mut buffer = Vec::new();
    binarize(cvr, |data| buffer.extend_from_slice(data));
    buffer
}

#[test]
fn test_binarize_null() {
    let cvr = StdCVR::Null;
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, [b'N']);
}

#[rstest]
#[case(true, &[b'B', b'1'])]
#[case(false, &[b'B', b'0'])]
fn test_binarize_bool(#[case] input: bool, #[case] expected: &[u8]) {
    let cvr = StdCVR::Bool(CVRBool::new(input));
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, expected);
}

#[rstest]
#[case(0, &[b'I', 0])]
#[case(1, &[b'I', 2])]
#[case(-1, &[b'I', 1])]
#[case(5123, &[b'I', 134, 80])]
#[case(-5123, &[b'I', 133, 80])]
#[case(32315643643, &[b'I', 246, 219, 204, 226, 240, 1])]
#[case(-860826, &[b'I', 179, 138, 105])]
#[case(i128::MIN, &[b'I', 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3])]
fn test_binarize_int(#[case] input: i128, #[case] expected: &[u8]) {
    let cvr = StdCVR::Int(CVRInt::new(input));
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, expected);
}

#[rstest]
#[case("", &[b'S', 0])]
#[case("ABC", &[b'S', 6, b'A', b'B', b'C'])]
#[case("1", &[b'S', 2, b'1'])]
#[case("1234567890ABCDEFGH", &[b'S', 36, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H'])]
fn test_binarize_string(#[case] input: &str, #[case] expected: &[u8]) {
    let cvr = StdCVR::String(CVRString::new(input).unwrap());
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, expected);
}

#[rstest]
#[case(0.5, &[b'F', 0, 0, 0, 0, 0, 0, 224, 63])]
#[case(-1.0/3.0, &[b'F', 85, 85, 85, 85, 85, 85, 213, 191])]
#[case(-12332.0/37.0, &[b'F', 76, 145, 207, 186, 193, 212, 116, 192])]
fn test_binarize_fraction(#[case] input: f64, #[case] expected: &[u8]) {
    let fraction = CVRFloat::new(input);
    assert_eq!(fraction.inner(), input);
    let cvr = StdCVR::Float(fraction);
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, expected);
}

#[test]
fn test_binarize_array_empty() {
    let cvr = StdCVR::Array(CVRArray::new());
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, [b'A', 0]);
}

#[test]
fn test_binarize_array_full() {
    let mut cvr = StdCVR::Array(CVRArray::new());
    let cvr_array = cvr.as_array_mut().unwrap().inner_mut();
    cvr_array.try_push(StdCVR::Null).unwrap();
    cvr_array.try_push(StdCVR::Int(CVRInt::new(1))).unwrap();
    cvr_array.try_push(StdCVR::Null).unwrap();
    cvr_array
        .try_push(StdCVR::String(CVRString::new("test").unwrap()))
        .unwrap();
    cvr_array.try_push(StdCVR::Bool(CVRBool::new(true))).unwrap();
    let result = binarize_to_vec(&cvr);
    assert_eq!(
        result,
        [
            b'A', 10, b'N', b'I', 2, b'N', b'S', 8, b't', b'e', b's', b't', b'B', b'1'
        ]
    );
}

#[test]
fn test_binarize_object_empty() {
    let cvr = StdCVR::Object(CVRObject::new());
    let result = binarize_to_vec(&cvr);
    assert_eq!(result, [b'O', 0]);
}

#[test]
fn test_binarize_object_full() {
    let mut cvr = StdCVR::Object(CVRObject::new());
    let cvr_object = cvr.as_object_mut().unwrap();
    cvr_object
        .try_insert(CVRString::new("test").unwrap(), StdCVR::Int(CVRInt::new(1)))
        .unwrap();
    cvr_object
        .try_insert(CVRString::new("abc").unwrap(), StdCVR::Null)
        .unwrap();

    let mut cvr_array = StdCVR::Array(CVRArray::new());
    let cvr_array_mut = cvr_array.as_array_mut().unwrap().inner_mut();
    cvr_array_mut.try_push(StdCVR::Null).unwrap();
    cvr_array_mut.try_push(StdCVR::Int(CVRInt::new(-3))).unwrap();

    cvr_object.try_insert(CVRString::new("x").unwrap(), cvr_array).unwrap();
    let result = binarize_to_vec(&cvr);
    assert_eq!(
        result,
        [
            b'O', 6, b'S', 6, b'a', b'b', b'c', b'N', b'S', 8, b't', b'e', b's', b't', b'I', 2, b'S', 2, b'x', b'A', 4,
            b'N', b'I', 5,
        ]
    );
}
