#![cfg(feature = "std_alloc")]

use osom_lib_strings::{StdImmutableString, StdImmutableWeakString};
use rstest::rstest;

const TEXT: &str = "Hello, world!";

#[inline(always)]
fn new_string(text: &str) -> StdImmutableString {
    StdImmutableString::new(text).unwrap()
}

#[test]
fn test_immutable_string() {
    let string = new_string(TEXT);
    assert_eq!(string.as_str(), TEXT);
}

#[test]
fn test_immutable_string_clone() {
    let string = new_string(TEXT);
    macro_rules! assert_strong_weak {
        ($imm:expr, $strong:expr, $weak:expr) => {
            assert_eq!(StdImmutableString::strong_count($imm), $strong);
            assert_eq!(StdImmutableString::weak_count($imm), $weak);
        };
    }
    assert_strong_weak!(&string, 1, 1);
    let clone = string.clone();
    assert_strong_weak!(&string, 2, 1);
    let clone2 = clone.clone();
    assert_strong_weak!(&string, 3, 1);
    drop(clone);
    assert_strong_weak!(&string, 2, 1);
    drop(string);
    assert_strong_weak!(&clone2, 1, 1);
    assert_eq!(clone2.as_str(), TEXT);
}

#[test]
fn test_immutable_string_weak() {
    let string = new_string(TEXT);
    macro_rules! assert_strong_weak {
        ($imm:expr, $strong:expr, $weak:expr) => {
            assert_eq!(StdImmutableString::strong_count($imm), $strong);
            assert_eq!(StdImmutableString::weak_count($imm), $weak);
        };
    }
    macro_rules! assert_strong_weak2 {
        ($imm:expr, $strong:expr, $weak:expr) => {
            assert_eq!(StdImmutableWeakString::strong_count($imm), $strong);
            assert_eq!(StdImmutableWeakString::weak_count($imm), $weak);
        };
    }
    assert_strong_weak!(&string, 1, 1);
    let clone = string.clone();
    assert_strong_weak!(&string, 2, 1);
    let weak = StdImmutableString::downgrade(&clone);
    assert_strong_weak!(&string, 2, 2);
    assert_strong_weak2!(&weak, 2, 2);
    let weak2 = StdImmutableString::downgrade(&string);
    assert_strong_weak!(&string, 2, 3);
    assert_strong_weak2!(&weak2, 2, 3);
    let new_strong = weak.upgrade().unwrap();
    assert_strong_weak2!(&weak2, 3, 3);
    assert_strong_weak!(&string, 3, 3);
    assert_eq!(new_strong.as_str(), TEXT);
    drop(new_strong);
    drop(clone);
    assert_strong_weak!(&string, 1, 3);
    drop(string);
    assert_strong_weak2!(&weak, 0, 2);
    assert_strong_weak2!(&weak2, 0, 2);
    assert!(weak.upgrade().is_none());
    drop(weak);
    assert_strong_weak2!(&weak2, 0, 1);
}

#[rstest]
#[case("")]
#[case("a")]
#[case("ab")]
#[case("abc")]
#[case("A")]
#[case("aB")]
#[case("AbC")]
#[case("Hello, world!")]
fn test_immutable_string_equal(#[case] text: &str) {
    let string = new_string(text);
    let string2 = new_string(text);
    assert_eq!(string, string2);
}

#[rstest]
#[case("", "a")]
#[case("", " ")]
#[case("a", "ab")]
#[case("ab", "abc")]
#[case("A", "a")]
#[case("aB", "AbC")]
#[case("AbC", "Hello, world!")]
fn test_immutable_string_not_equal(#[case] left: &str, #[case] right: &str) {
    let left = new_string(left);
    let right = new_string(right);
    assert_ne!(left, right);
}
