#![cfg(feature = "std")]

use osom_lib_arrays::{std::StdDynamicArray, traits::ImmutableArray};
use osom_lib_primitives::length::Length;

mod array_helpers;

#[test]
fn test_std_dynamic_array() {
    array_helpers::test_mutable_array(StdDynamicArray::new);
}

#[test]
fn test_std_dynamic_array_destruction() {
    array_helpers::test_array_destruction(StdDynamicArray::new);
}

#[test]
fn test_std_dynamic_array_clone() {
    array_helpers::test_array_clone(StdDynamicArray::new);
}

#[test]
fn test_std_dynamic_array_back_and_forth() {
    array_helpers::test_array_back_and_forth(StdDynamicArray::new)
}

#[test]
fn test_std_dynamic_array_sized_allocation() {
    let array = StdDynamicArray::with_factory(Length::try_from_u32(15).unwrap(), |idx| idx).unwrap();
    assert_eq!(array.as_ref(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_std_dynamic_array_unsafe_allocation() {
    let length = Length::try_from_u32(6).unwrap();
    let mut array = unsafe { StdDynamicArray::with_size_uninitialized(length) }.unwrap();
    for i in 0..array.length().as_usize() {
        array.as_mut()[i] = i * i;
    }
    assert_eq!(array.as_ref(), &[0, 1, 4, 9, 16, 25]);
}
