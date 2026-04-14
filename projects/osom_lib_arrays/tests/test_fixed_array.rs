#![cfg(feature = "std")]

use osom_lib_arrays::{
    std::StdFixedArray,
    traits::{ImmutableArray, MutableArray},
};
use osom_lib_primitives::length::Length;

mod array_helpers;

macro_rules! array_builder {
    ( $val: expr ) => {{ StdFixedArray::with_capacity(Length::try_from_i32($val).unwrap()).unwrap() }};
}

#[test]
fn test_std_fixed_array() {
    array_helpers::test_mutable_array(|| array_builder!(100));
}

#[test]
fn test_std_fixed_array_destruction() {
    array_helpers::test_array_destruction(|| array_builder!(100));
}

#[test]
fn test_std_fixed_array_clone() {
    array_helpers::test_array_clone(|| array_builder!(100));
}

#[test]
fn test_std_fixed_array_sized_allocation() {
    let array = StdFixedArray::with_factory(Length::try_from_u32(15).unwrap(), |idx| idx).unwrap();
    assert_eq!(array.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_std_fixed_array_unsafe_allocation() {
    let length = Length::try_from_u32(6).unwrap();
    let mut array = unsafe { StdFixedArray::with_size_uninitialized(length) }.unwrap();
    for i in 0..array.length().as_usize() {
        array.as_slice_mut()[i] = i * i;
    }
    assert_eq!(array.as_slice(), &[0, 1, 4, 9, 16, 25]);
}

#[test]
fn test_std_fixed_array_error_beyond_limit() {
    const LENGTH: usize = 10;
    let mut array = array_builder!(LENGTH as i32);

    for idx in 0..LENGTH {
        array.push(idx * idx);
    }

    for idx in 0..5 {
        assert!(array.try_push(idx * idx).is_err());
    }
}
