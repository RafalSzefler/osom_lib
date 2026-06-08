#![cfg(feature = "std")]

use osom_lib_arrays::traits::MutableArray as _;
use osom_lib_numbers::iterators::ConstPermutationGenerator;
use osom_structa_cvr::{CVRArray, CVRBool, CVRFloat, CVRInt, CVRObject, CVRString, std::StdCVR};

#[test]
fn test_comparison_null() {
    let cvr1 = StdCVR::Null;
    let cvr2 = StdCVR::Null;
    assert_eq!(cvr1, cvr2);
}

#[test]
fn test_comparison_bool() {
    let cvr1 = StdCVR::Bool(CVRBool::new(true));
    let cvr2 = StdCVR::Bool(CVRBool::new(true));
    let cvr3 = StdCVR::Bool(CVRBool::new(false));
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_int() {
    let cvr1 = StdCVR::Int(CVRInt::new(1));
    let cvr2 = StdCVR::Int(CVRInt::new(1));
    let cvr3 = StdCVR::Int(CVRInt::new(2));
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_float() {
    let cvr1 = StdCVR::Float(CVRFloat::new(1.0));
    let cvr2 = StdCVR::Float(CVRFloat::new(1.0));
    let cvr3 = StdCVR::Float(CVRFloat::new(2.0));
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_string() {
    let cvr1 = StdCVR::String(CVRString::new("test").unwrap());
    let cvr2 = StdCVR::String(CVRString::new("test").unwrap());
    let cvr3 = StdCVR::String(CVRString::new("test2").unwrap());
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_array() {
    let cvr1 = StdCVR::Array(CVRArray::new());
    let cvr2 = StdCVR::Array(CVRArray::new());
    let mut cvr3 = StdCVR::Array(CVRArray::new());
    cvr3.as_array_mut().unwrap().inner_mut().try_push(StdCVR::Null).unwrap();
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_object() {
    let cvr1 = StdCVR::Object(CVRObject::new());
    let cvr2 = StdCVR::Object(CVRObject::new());
    let mut cvr3 = StdCVR::Object(CVRObject::new());
    cvr3.as_object_mut()
        .unwrap()
        .try_insert(CVRString::new("test").unwrap(), StdCVR::Null)
        .unwrap();
    assert_eq!(cvr1, cvr2);
    assert_ne!(cvr1, cvr3);
}

#[test]
fn test_comparison_object_with_key_order() {
    let keys = [
        "test1",
        "test2",
        "asdf  asd",
        "FooBaz",
        "314 nklvfsn-745 m 3245 3\r5",
        "osom_lib",
    ];
    let mut cvr1 = StdCVR::Object(CVRObject::new());

    for key in keys {
        cvr1.as_object_mut()
            .unwrap()
            .try_insert(CVRString::new(key).unwrap(), StdCVR::Null)
            .unwrap();
    }

    let permutations = ConstPermutationGenerator::<6>::new();
    assert_eq!(permutations.length(), keys.len());

    for permutation in permutations {
        let mut target_cvr = StdCVR::Object(CVRObject::new());
        for idx in permutation {
            target_cvr
                .as_object_mut()
                .unwrap()
                .try_insert(CVRString::new(keys[idx]).unwrap(), StdCVR::Null)
                .unwrap();
        }
        assert_eq!(cvr1, target_cvr);
    }
}
