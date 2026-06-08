#![cfg(feature = "std")]
use osom_lib_arrays::traits::MutableArray as _;
use osom_structa_cvr::{CVRArray, CVRBool, CVRFloat, CVRInt, CVRObject, CVRString, std::StdCVR, tools::calculate_depth};

#[test]
fn test_calculate_depth_shallow() {
    let cvr = StdCVR::Null;
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::Bool(CVRBool::new(true));
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::Int(CVRInt::new(1));
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::String(CVRString::new("test").unwrap());
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::Float(CVRFloat::new(1.0 / 2.0));
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::Array(CVRArray::new());
    assert_eq!(calculate_depth(&cvr), 0);

    let cvr = StdCVR::Object(CVRObject::new());
    assert_eq!(calculate_depth(&cvr), 0);
}

#[test]
fn test_calculate_depth_deep() {
    let mut cvr = StdCVR::Array(CVRArray::new());
    assert_eq!(calculate_depth(&cvr), 0);

    {
        let cvr_array = cvr.as_array_mut().unwrap().inner_mut();
        cvr_array.try_push(StdCVR::Null).unwrap();
        cvr_array.try_push(StdCVR::Int(CVRInt::new(1))).unwrap();
        cvr_array.try_push(StdCVR::Null).unwrap();
        cvr_array
            .try_push(StdCVR::String(CVRString::new("test").unwrap()))
            .unwrap();
    }

    assert_eq!(calculate_depth(&cvr), 1);

    let mut nested_object = StdCVR::Object(CVRObject::new());
    nested_object
        .as_object_mut()
        .unwrap()
        .try_insert(CVRString::new("test2").unwrap(), StdCVR::Int(CVRInt::new(2)))
        .unwrap();

    {
        let cvr_array = cvr.as_array_mut().unwrap().inner_mut();
        cvr_array.try_push(nested_object).unwrap();
    }

    assert_eq!(calculate_depth(&cvr), 2);
}
