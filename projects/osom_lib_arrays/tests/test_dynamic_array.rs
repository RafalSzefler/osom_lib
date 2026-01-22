#![cfg(feature = "std")]

use osom_lib_arrays::std::StdDynamicArray;

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
