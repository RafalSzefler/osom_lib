#![cfg(feature = "std")]

use core::borrow::Borrow;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use osom_lib_strings::std::{StdOwnedString, StdOwnedStringBuilder};
use osom_lib_try_clone::TryClone as _;
use rstest::rstest;

#[rstest]
#[case("")]
#[case("foo")]
#[case("AbZasdfqwddeGD2315364. dfq")]
fn test_owned_string_try_from_str(#[case] input: &str) {
    let string = StdOwnedString::try_from_str(input).unwrap();
    assert_eq!(string.as_ref(), input);

    let from_trait: StdOwnedString = input.into();
    assert_eq!(from_trait, string);
}

#[rstest]
#[case("")]
#[case("12345678")]
#[case("123456789")]
fn test_owned_string_short_string_optimization_boundary(#[case] input: &str) {
    let string = StdOwnedString::try_from_str(input).unwrap();
    assert_eq!(string.as_ref(), input);
    assert_eq!(string.as_ref().len(), input.len());
}

#[test]
fn test_owned_string_builder_chunks() {
    let mut builder = StdOwnedStringBuilder::new();
    builder.push_str("abc");
    builder.push_str("xyz");
    builder.push_str("  Lorem ipsum dolor sit amet.");
    let string = builder.build();
    assert_eq!(string.as_ref(), "abcxyz  Lorem ipsum dolor sit amet.");
}

#[test]
fn test_owned_string_builder_try_push_str() {
    let mut builder = StdOwnedStringBuilder::new();
    builder.try_push_str("hello").unwrap();
    builder.try_push_str(", ").unwrap();
    builder.try_push_str("world").unwrap();
    assert_eq!(builder.build().as_ref(), "hello, world");
}

#[test]
fn test_owned_string_builder_default() {
    let builder = StdOwnedStringBuilder::default();
    assert_eq!(builder.build().as_ref(), "");
}

#[test]
fn test_owned_string_clone_and_try_clone() {
    let original = StdOwnedString::try_from_str("clone me").unwrap();

    let cloned = original.clone();
    assert_eq!(cloned, original);

    let try_cloned = original.try_clone().unwrap();
    assert_eq!(try_cloned, original);
}

#[test]
fn test_owned_string_builder_round_trip() {
    let original = StdOwnedString::try_from_str("round trip").unwrap();

    let mut builder: StdOwnedStringBuilder = original.into();
    builder.push_str(" extended");
    let extended: StdOwnedString = builder.build().into();
    assert_eq!(extended.as_ref(), "round trip extended");

    let rebuilt: StdOwnedStringBuilder = extended.into();
    assert_eq!(rebuilt.build().as_ref(), "round trip extended");
}

#[test]
fn test_owned_string_from_builder() {
    let mut builder = StdOwnedStringBuilder::new();
    builder.push_str("from builder");
    let string: StdOwnedString = builder.into();
    assert_eq!(string.as_ref(), "from builder");
}

#[test]
fn test_owned_string_borrow_and_hash() {
    let left = StdOwnedString::try_from_str("hash me").unwrap();
    let right = StdOwnedString::try_from_str("hash me").unwrap();

    fn hash_string<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    assert_eq!(left, right);
    assert_eq!(hash_string(&left), hash_string(&right));

    let borrowed: &str = left.borrow();
    assert_eq!(borrowed, "hash me");
}
