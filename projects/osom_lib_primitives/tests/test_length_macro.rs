use osom_lib_primitives::macros::make_length;

macro_rules! build_length_macro_test {
    ( $name: ident, $value: expr ) => {
        paste::paste! {
            #[test]
            fn [< test_length_ $name >]() {
                let length = make_length!($value);
                assert_eq!(length.as_u32(), $value);
            }
        }
    };
}

build_length_macro_test!(a, 0);
build_length_macro_test!(b, 1);
build_length_macro_test!(c, 2);
build_length_macro_test!(d, 3);
build_length_macro_test!(e, 1234321);

const VAL1: u32 = 15;
build_length_macro_test!(f, VAL1);
