#![cfg(feature = "std")]
use std::collections::HashMap;

mod common;

#[allow(non_snake_case)]
#[test]
fn test_HashMap_simple() {
    crate::common::test_hash_table_simple(HashMap::default);
}

#[allow(non_snake_case)]
#[test]
fn test_HashMap_iter() {
    crate::common::test_hash_table_iter(HashMap::default);
}

#[allow(non_snake_case)]
#[test]
fn test_HashMap_ownership() {
    crate::common::test_hash_table_ownership(HashMap::default);
}

#[cfg(not(miri))]
#[allow(non_snake_case)]
#[test]
fn test_HashMap_biggg() {
    crate::common::test_hash_table_biggg(HashMap::default);
}
