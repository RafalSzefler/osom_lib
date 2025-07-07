#![cfg(feature = "std_alloc")]

use osom_lib_arrays::Array;
use osom_lib_primitives::Length;

#[inline(always)]
fn new_array<T: Sized + Copy, const N: usize>(array: [T; N]) -> Array<T> {
    Array::from_array(array).unwrap()
}

#[test]
fn test_array() {
    let array = new_array([1, 2, 3]);
    assert_eq!(array.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_array_clone() {
    let array = new_array([1, 2, 3]);
    let array_2 = array.clone();
    assert_eq!(array.as_slice(), &[1, 2, 3]);
    assert_eq!(array_2.as_slice(), &[1, 2, 3]);
    assert_eq!(array.len(), Length::try_from_i32(3).unwrap());
    assert_eq!(array_2.len(), Length::try_from_i32(3).unwrap());
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
    let array: Array<CustomStruct> = Array::from_slice(&custom_vec).unwrap();

    assert_eq!(array.as_slice().len(), 4);
    assert_eq!(array.len().value(), 4);
    assert_eq!(array.as_slice(), custom_vec.as_slice());
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
        let array: Array<CustomDrop> = Array::from_slice(&custom_slice).unwrap();
        assert_eq!(array.as_slice().len(), 4);
        assert_eq!(array.len().value(), 4);
        assert_ref_count!(0);
        drop(custom_slice);
        assert_ref_count!(4);
        drop(array);
        assert_ref_count!(8);
    }

    drop(drops_count);
}
