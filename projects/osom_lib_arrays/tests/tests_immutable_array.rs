#![cfg(feature = "std_alloc")]

use osom_lib_arrays::{ImmutableArray, ImmutableArrayBuilder};
use osom_lib_primitives::Length;
use rstest::rstest;

#[inline(always)]
fn new_array<T: Sized + Copy, const N: usize>(array: [T; N]) -> ImmutableArray<T> {
    ImmutableArray::from_array(array).unwrap()
}

#[test]
fn test_immutable_array() {
    let array = new_array([1, 2, 3]);
    assert_eq!(array.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_immutable_array_clone() {
    let array = new_array([1, 2, 3]);
    macro_rules! assert_strong_weak {
        ($imm:expr, $strong:expr, $weak:expr) => {
            assert_eq!(ImmutableArray::strong_count($imm), $strong);
            assert_eq!(ImmutableArray::weak_count($imm), $weak);
        };
    }
    assert_strong_weak!(&array, 1, 1);
    let clone = array.clone();
    assert_strong_weak!(&array, 2, 1);
    let clone2 = clone.clone();
    assert_strong_weak!(&array, 3, 1);
    drop(clone);
    assert_strong_weak!(&array, 2, 1);
    drop(array);
    assert_strong_weak!(&clone2, 1, 1);
    assert_eq!(clone2.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_custom_struct() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[repr(C)]
    struct CustomStruct {
        pub first: i8,
        pub value: i32,
    }

    let custom_vec = vec![
        CustomStruct { first: 1, value: 1 },
        CustomStruct { first: 2, value: 2 },
        CustomStruct { first: 3, value: 3 },
        CustomStruct { first: 4, value: 4 },
    ];
    let immutable_array: ImmutableArray<CustomStruct> = ImmutableArray::from_slice(&custom_vec).unwrap();

    assert_eq!(immutable_array.as_slice().len(), 4);
    assert_eq!(immutable_array.len().value(), 4);
    assert_eq!(immutable_array.as_slice(), custom_vec.as_slice());
}

#[test]
fn test_custom_drop() {
    struct CustomDrop {
        drops_count: *mut i32,
    }

    impl Clone for CustomDrop {
        fn clone(&self) -> Self {
            Self {
                drops_count: self.drops_count,
            }
        }
    }

    impl CustomDrop {
        pub fn new(drops_count: *mut i32) -> Self {
            Self { drops_count }
        }
    }

    impl Drop for CustomDrop {
        fn drop(&mut self) {
            unsafe {
                *self.drops_count += 1;
            }
        }
    }

    let mut drops_count = Box::new(0);
    let drops_count_ptr = &mut *drops_count;

    {
        macro_rules! assert_ref_count {
            ( $expected:expr ) => {{
                let value = *drops_count_ptr;
                assert_eq!(value, $expected);
            }};
        }
        assert_ref_count!(0);

        let custom_slice = vec![
            CustomDrop::new(drops_count_ptr),
            CustomDrop::new(drops_count_ptr),
            CustomDrop::new(drops_count_ptr),
            CustomDrop::new(drops_count_ptr),
        ];

        assert_ref_count!(0);
        let immutable_array: ImmutableArray<CustomDrop> = ImmutableArray::from_slice(&custom_slice).unwrap();
        assert_eq!(immutable_array.as_slice().len(), 4);
        assert_eq!(immutable_array.len().value(), 4);
        assert_ref_count!(0);
        drop(custom_slice);
        assert_ref_count!(4);
        drop(immutable_array);
        assert_ref_count!(8);
    }

    drop(drops_count);
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[-1, 13])]
#[case(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
#[case(&[1, -5, 3, 4, -87, 6, 7, 8, 9, 3412])]
#[case(&[1, -5, 3, 4, -87, 6, 7, 8, 9, 3412, 1, -5, 3, 4, -87, 6, 7, 8, 9, 3412, 5321, 2, 0, 0, 0, 0, 12312, 0])]
fn test_builder_from_slice(#[case] slice: &[i32]) {
    let mut builder: ImmutableArrayBuilder<_> = ImmutableArrayBuilder::new().unwrap();
    builder.extend_from_slice(slice).unwrap();
    let array = builder.build();
    assert_eq!(array.as_slice(), slice);
}

#[test]
fn test_builder_shrink_1() {
    let mut builder: ImmutableArrayBuilder<bool> = ImmutableArrayBuilder::new().unwrap();
    assert_eq!(builder.len(), Length::ZERO);
    assert!(builder.capacity() > Length::ZERO);
    builder.shrink_to_fit().unwrap();
    assert_eq!(builder.len(), Length::ZERO);
    assert_eq!(builder.capacity(), Length::ZERO);
    let array = builder.build();
    assert_eq!(array.as_slice(), &[]);
    assert_eq!(array.capacity(), Length::ZERO);
}

#[test]
fn test_builder_shrink_2() {
    let mut builder: ImmutableArrayBuilder<bool> = ImmutableArrayBuilder::new().unwrap();
    builder.push(true).unwrap();
    builder.push(false).unwrap();
    builder.push(true).unwrap();
    let expected_len = Length::try_from_i32(3).unwrap();
    assert_eq!(builder.len(), expected_len);
    assert!(builder.capacity() > expected_len);
    builder.shrink_to_fit().unwrap();
    assert_eq!(builder.len(), expected_len);
    assert_eq!(builder.capacity(), expected_len);
    let array = builder.build();
    assert_eq!(array.as_slice(), &[true, false, true]);
    assert_eq!(array.capacity(), expected_len);
}

#[test]
fn test_thread_safety() {
    let mut builder: ImmutableArrayBuilder<i32> = ImmutableArrayBuilder::new().unwrap();
    for i in 1..1000 {
        builder.push(2 * i).unwrap();
    }
    let array = builder.build();

    let mut handles = Vec::new();
    let mut final_sum = 0;
    for idx in 1..10 {
        let clone = array.clone();
        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            let mut local_sum = 1;
            for item in clone.as_slice() {
                local_sum += item * idx
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            local_sum
        });
        handles.push(handle);
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    for handle in handles {
        let value = handle.join().unwrap();
        final_sum += value;
    }

    assert_eq!(final_sum, 44955009);
}
