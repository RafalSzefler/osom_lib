#![cfg(feature = "std")]

use osom_lib_arrays::{
    std::StdAlignedDynamicArray,
    traits::{ImmutableArray, MutableArray},
};
use osom_lib_primitives::length::Length;

mod array_helpers;

#[repr(align(16))]
struct HighAlign;

macro_rules! array_builder {
    ( $val: expr ) => {{ StdAlignedDynamicArray::<HighAlign, _>::with_capacity(Length::try_from_i32($val).unwrap()).unwrap() }};
}

#[test]
fn test_alignment() {
    assert!(align_of::<HighAlign>() >= 16);
    let mut array: StdAlignedDynamicArray<HighAlign, u8> = array_builder!(5);
    array.push_array([1, 2, 3]);
    let ptr = array.as_ref().as_ptr();
    assert!(ptr.align_offset(align_of::<HighAlign>()) == 0);
}

#[test]
fn test_std_aligned_dynamic_array() {
    array_helpers::test_mutable_array(|| array_builder!(100));
}

#[test]
fn test_std_aligned_dynamic_array_destruction() {
    array_helpers::test_array_destruction(|| array_builder!(100));
}

#[test]
fn test_std_aligned_dynamic_array_clone() {
    array_helpers::test_array_clone(|| array_builder!(100));
}

#[test]
fn test_std_aligned_dynamic_array_back_and_forth() {
    array_helpers::test_array_back_and_forth(|| array_builder!(600))
}

#[test]
fn test_std_aligned_dynamic_array_sized_allocation() {
    let array =
        StdAlignedDynamicArray::<HighAlign, _>::with_factory(Length::try_from_u32(15).unwrap(), |idx| idx).unwrap();
    assert_eq!(array.as_ref(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_std_aligned_dynamic_array_unsafe_allocation() {
    let length = Length::try_from_u32(6).unwrap();
    let mut array = unsafe { StdAlignedDynamicArray::<HighAlign, _>::with_size_uninitialized(length) }.unwrap();
    for i in 0..array.length().as_usize() {
        array.as_mut()[i] = i * i;
    }
    assert_eq!(array.as_ref(), &[0, 1, 4, 9, 16, 25]);
}
