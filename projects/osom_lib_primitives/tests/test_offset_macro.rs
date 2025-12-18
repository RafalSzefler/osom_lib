use osom_lib_primitives::macros::make_offset;

macro_rules! build_offset_macro_test {
    ( $name: ident, $value: expr ) => {
        paste::paste! {
            #[test]
            fn [< test_offset_ $name >]() {
                let offset = make_offset!($value);
                assert_eq!(offset.as_i32(), $value);
            }
        }
    };
}

build_offset_macro_test!(a, -5123);
build_offset_macro_test!(b, -2);
build_offset_macro_test!(c, -1);
build_offset_macro_test!(d, 0);
build_offset_macro_test!(e, 1);
build_offset_macro_test!(f, 2);
build_offset_macro_test!(g, 1234321);

const VAL1: i32 = 15;
build_offset_macro_test!(x1, VAL1);

const VAL2: i32 = -13;
build_offset_macro_test!(x2, VAL2);

#[test]
fn test_offset_negative_expression() {
    const VAL: i32 = 17;
    let offset = make_offset!(-VAL);
    assert_eq!(offset.as_i32(), -VAL);
}
