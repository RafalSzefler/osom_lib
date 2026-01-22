use osom_lib_arrays::{std::StdInlineArray, traits::MutableArray};

use rstest::rstest;

mod array_helpers;

#[test]
fn test_inline_array() {
    array_helpers::test_mutable_array(StdInlineArray::<10, _>::new);
}

#[test]
fn test_inline_array_destruction() {
    array_helpers::test_array_destruction(StdInlineArray::<10, _>::new);
}

#[rstest]
#[case(StdInlineArray::<1, i32>::new)]
#[case(StdInlineArray::<2, i32>::new)]
#[case(StdInlineArray::<5, i32>::new)]
#[case(StdInlineArray::<10, i32>::new)]
#[case(StdInlineArray::<15, i32>::new)]
#[case(StdInlineArray::<25, i32>::new)]
fn test_inline_array_clone<TArr: MutableArray<i32> + Clone, Builder: FnOnce() -> TArr>(#[case] array_builder: Builder) {
    array_helpers::test_array_clone(array_builder);
}

#[rstest]
#[case(StdInlineArray::<10, i32>::new, 15)]
#[case(StdInlineArray::<10, i32>::new, 11)]
#[case(StdInlineArray::<1, i32>::new, 2)]
#[case(StdInlineArray::<1, i32>::new, 3)]
#[case(StdInlineArray::<1, i32>::new, 15)]
#[case(StdInlineArray::<99, i32>::new, 100)]
fn test_inline_array_overflow<TArr: MutableArray<i32>, Builder: FnOnce() -> TArr>(
    #[case] array_builder: Builder,
    #[case] count: usize,
) {
    let mut array = array_builder();
    let mut tmp_vec = Vec::with_capacity(count);
    for idx in 0..count {
        array.push(idx as i32);
        tmp_vec.push(idx as i32);
        assert_eq!(array.as_slice(), tmp_vec.as_slice());
    }
}
