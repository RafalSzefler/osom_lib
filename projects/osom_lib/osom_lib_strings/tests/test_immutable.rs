#![cfg(feature = "std")]
use rstest::rstest;

use osom_lib_strings::immutable::WeakUpgradeError;
use osom_lib_strings::immutable::std::StdImmutableStringBuilder;

#[rstest]
#[case("")]
#[case("foo")]
#[case("AbZasdfqwddeGD2315364. dfq")]
fn test_immutable_string(#[case] input: &str) {
    let mut builder = StdImmutableStringBuilder::new().unwrap();
    builder.try_push(input).unwrap();
    let string = builder.build().unwrap();
    assert_eq!(string.as_str(), input);
    assert_eq!(string.strong_count(), 1);
    assert_eq!(string.weak_count(), 1);
    let clone = string.clone();
    assert_eq!(clone.as_str(), input);
    assert_eq!(string.strong_count(), 2);
    assert_eq!(string.weak_count(), 1);
    assert_eq!(clone.strong_count(), 2);
    assert_eq!(clone.weak_count(), 1);

    drop(clone);
    assert_eq!(string.strong_count(), 1);
    assert_eq!(string.weak_count(), 1);

    let weak = string.downgrade().unwrap();
    assert_eq!(string.strong_count(), 1);
    assert_eq!(string.weak_count(), 2);

    let up = weak.upgrade().unwrap();
    assert_eq!(string.strong_count(), 2);
    assert_eq!(string.weak_count(), 2);
    drop(up);

    assert_eq!(string.strong_count(), 1);
    assert_eq!(string.weak_count(), 2);
    drop(string);

    assert_eq!(weak.strong_count(), 0);
    assert_eq!(weak.weak_count(), 1);
    assert!(matches!(weak.upgrade(), Err(WeakUpgradeError::NoStrongReferencesAlive)));
}

#[test]
fn test_immutable_string_chunks() {
    let mut builder = StdImmutableStringBuilder::new().unwrap();
    builder.try_push("abc").unwrap();
    builder.try_push("xyz").unwrap();
    builder.try_push("  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.").unwrap();
    builder.shrink_to_fit().unwrap();
    let result = builder.build().unwrap();
    assert_eq!(
        result.as_str(),
        "abcxyz  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
    );
    assert_eq!(
        result.as_c_str(),
        "abcxyz  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\0"
    )
}

#[rstest]
#[case("", "\0")]
#[case("abc", "abc\0")]
#[case(" a%@#", " a%@#\0")]
#[case("  234 asdfa.  ", "  234 asdfa.  \0")]
fn test_c_str(#[case] input: &str, #[case] expected: &str) {
    let mut builder = StdImmutableStringBuilder::new().unwrap();
    builder.try_push(input).unwrap();
    let string = builder.build().unwrap();
    assert_eq!(string.as_str(), &expected[0..expected.len() - 1]);
    assert_eq!(string.as_c_str(), expected);
}
