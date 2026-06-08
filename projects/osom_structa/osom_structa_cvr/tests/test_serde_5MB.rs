#![cfg(all(feature = "std", feature = "serde"))]
#![cfg(not(miri))]
#![allow(non_snake_case)]

mod common;

use osom_structa_cvr::{serde::CVRSeed, std::StdCVRDeserializeContext};
use priv_osom_lib_tests::deserialize::deserialize_json_with_seed;

#[test]
fn test_serde_5MB() {
    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = CVRSeed { context: &mut context };
    let result = deserialize_json_with_seed(common::TEXT_5MB, seed).unwrap();

    let array = result.as_array().unwrap().inner_ref().as_ref();
    assert_eq!(array.len(), 15840);

    for cvr_object in array {
        let cvr_object = cvr_object.as_object().unwrap();
        assert_eq!(cvr_object.len().as_usize(), 5);
        assert!(cvr_object.contains_key("name"));
        assert!(cvr_object.contains_key("language"));
        assert!(cvr_object.contains_key("id"));
        assert!(cvr_object.contains_key("bio"));
        assert!(cvr_object.contains_key("version"));
    }

    let first_cvr_object = array[0].as_object().unwrap();
    assert_eq!(first_cvr_object.len().as_usize(), 5);
    let name = first_cvr_object
        .get("name")
        .unwrap()
        .as_string()
        .unwrap()
        .as_immutable_string();
    assert_eq!(name.as_str(), "Adeel Solangi");

    let language = first_cvr_object
        .get("language")
        .unwrap()
        .as_string()
        .unwrap()
        .as_immutable_string();
    assert_eq!(language.as_str(), "Sindhi");

    let id = first_cvr_object
        .get("id")
        .unwrap()
        .as_string()
        .unwrap()
        .as_immutable_string();
    assert_eq!(id.as_str(), "V59OF92YF627HFY0");

    let bio = first_cvr_object
        .get("bio")
        .unwrap()
        .as_string()
        .unwrap()
        .as_immutable_string();
    assert_eq!(
        bio.as_str(),
        "Donec lobortis eleifend condimentum. Cras dictum dolor lacinia lectus vehicula rutrum. Maecenas quis nisi nunc. Nam tristique feugiat est vitae mollis. Maecenas quis nisi nunc."
    );

    let version = *first_cvr_object.get("version").unwrap().as_fraction().unwrap();
    let version_f64: f64 = version.try_into().unwrap();
    assert!((version_f64 - 6.1).abs() < 1e-10);

    let validate_strong_count = |field_name: &str| {
        let name_key = first_cvr_object
            .get_key_value(field_name)
            .unwrap()
            .0
            .as_immutable_string();
        assert_eq!(name_key.strong_count(), 15841);
    };

    validate_strong_count("name");
    validate_strong_count("language");
    validate_strong_count("id");
    validate_strong_count("bio");
    validate_strong_count("version");
}
