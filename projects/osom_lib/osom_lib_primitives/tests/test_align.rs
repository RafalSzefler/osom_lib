use osom_lib_primitives::align::{Align, Alignment};
use paste::paste;

pub struct TestAlign<const ALIGN: usize>
where
    Align<ALIGN>: Alignment,
{
    _inner: Align<ALIGN>,
}

macro_rules! test_align {
    ( $align: literal ) => {
        paste! {
            #[test]
            fn [< test_align_ $align >]() {
                assert_eq!(size_of::<TestAlign<$align>>(), 0);
                assert_eq!(align_of::<TestAlign<$align>>(), $align);
            }
        }
    };
}

test_align!(1);
test_align!(2);
test_align!(4);
test_align!(8);
test_align!(16);
test_align!(32);
test_align!(64);
test_align!(128);
test_align!(256);
test_align!(512);
test_align!(1024);
test_align!(2048);
test_align!(4096);
test_align!(8192);
test_align!(16384);
test_align!(32768);
test_align!(65536);
test_align!(131072);
test_align!(262144);
test_align!(524288);
test_align!(1048576);
test_align!(2097152);
test_align!(4194304);
test_align!(8388608);
test_align!(16777216);
test_align!(33554432);
test_align!(67108864);
test_align!(134217728);
test_align!(268435456);
test_align!(536870912);
