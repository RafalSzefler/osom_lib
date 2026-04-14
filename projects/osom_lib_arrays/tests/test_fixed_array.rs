use osom_lib_arrays::fixed_array::{ConstFixedArray, InlineFixedArray};
use osom_lib_arrays::traits::MutableArray;

use osom_lib_primitives::{length::Length, macros::make_length};
use rstest::rstest;

mod array_helpers;

#[test]
fn test_fixed_array() {
    array_helpers::test_mutable_array(InlineFixedArray::<10, _>::new);
}

#[test]
fn test_fixed_array_destruction() {
    array_helpers::test_array_destruction(InlineFixedArray::<10, _>::new);
}

#[test]
fn test_fixed_array_clone() {
    array_helpers::test_array_clone(InlineFixedArray::<15, _>::new);
}

#[rstest]
#[case(InlineFixedArray::<10, i32>::new, 15)]
#[case(InlineFixedArray::<10, i32>::new, 11)]
#[case(InlineFixedArray::<1, i32>::new, 2)]
#[case(InlineFixedArray::<1, i32>::new, 3)]
#[case(InlineFixedArray::<1, i32>::new, 15)]
#[case(InlineFixedArray::<99, i32>::new, 100)]
fn test_overflow_error<TArr: MutableArray<i32>, Builder: FnOnce() -> TArr>(
    #[case] array_builder: Builder,
    #[case] count: usize,
) {
    let mut array = array_builder();
    let mut has_overflowed = false;
    for idx in 0..count {
        use osom_lib_arrays::errors::ArrayError;

        match array.try_push(idx as i32) {
            Ok(_) => (),
            Err(ArrayError::LengthLimitExceeded) => {
                has_overflowed = true;
                break;
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    assert!(has_overflowed);
}

#[test]
fn test_const_array() {
    let mut array = ConstFixedArray::<10, i32>::new();
    assert_eq!(array.as_slice_const(), &[]);
    assert_eq!(array.as_slice_mut_const(), &[]);
    assert_eq!(array.length(), Length::ZERO);
    assert!(array.is_empty());
    assert!(array.try_pop_const().is_err());

    array.push_const(1);
    assert_eq!(array.as_slice_const(), &[1]);
    assert_eq!(array.as_slice_mut_const(), &[1]);
    assert_eq!(array.length(), Length::ONE);
    assert!(!array.is_empty());

    array.push_slice_const(&[5, -1, 3]);
    assert_eq!(array.as_slice_const(), &[1, 5, -1, 3]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5, -1, 3]);
    assert_eq!(array.length(), make_length!(4));
    assert!(!array.is_empty());

    array.push_slice_const(&[1, 1, 1, 2, 2, 2]);
    assert_eq!(array.as_slice_const(), &[1, 5, -1, 3, 1, 1, 1, 2, 2, 2]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5, -1, 3, 1, 1, 1, 2, 2, 2]);
    assert_eq!(array.length(), make_length!(10));
    assert!(!array.is_empty());

    assert_eq!(array.pop_const(), 2);
    assert_eq!(array.as_slice_const(), &[1, 5, -1, 3, 1, 1, 1, 2, 2]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5, -1, 3, 1, 1, 1, 2, 2]);
    assert_eq!(array.length(), make_length!(9));
    assert!(!array.is_empty());

    assert_eq!(array.pop_const(), 2);
    assert_eq!(array.as_slice_const(), &[1, 5, -1, 3, 1, 1, 1, 2]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5, -1, 3, 1, 1, 1, 2]);
    assert_eq!(array.length(), make_length!(8));
    assert!(!array.is_empty());

    let _ = array.pop_const();
    let _ = array.pop_const();
    let _ = array.pop_const();
    let _ = array.pop_const();
    let _ = array.pop_const();
    assert_eq!(array.as_slice_const(), &[1, 5, -1]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5, -1]);
    assert_eq!(array.length(), make_length!(3));
    assert!(!array.is_empty());

    assert_eq!(array.pop_const(), -1);
    assert_eq!(array.as_slice_const(), &[1, 5]);
    assert_eq!(array.as_slice_mut_const(), &[1, 5]);
    assert_eq!(array.length(), make_length!(2));
    assert!(!array.is_empty());

    assert_eq!(array.pop_const(), 5);
    assert_eq!(array.as_slice_const(), &[1]);
    assert_eq!(array.as_slice_mut_const(), &[1]);
    assert_eq!(array.length(), make_length!(1));
    assert!(!array.is_empty());

    assert_eq!(array.pop_const(), 1);
    assert_eq!(array.as_slice_const(), &[]);
    assert_eq!(array.as_slice_mut_const(), &[]);
    assert_eq!(array.length(), make_length!(0));
    assert!(array.is_empty());

    assert!(array.try_pop_const().is_err());
}
