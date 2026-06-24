use core::convert::Infallible;

use osom_lib_try_clone::{
    TryClone as _,
    macros::{try_clone, try_clone_with_clone},
};

#[derive(Debug, PartialEq, Eq)]
pub struct MyError;

impl From<Infallible> for MyError {
    fn from(_: Infallible) -> Self {
        Self
    }
}

#[try_clone(MyError)]
#[derive(Debug, PartialEq, Eq)]
pub enum MyEnum {
    A,
    B(u8),
    C(i32, bool),
}

#[try_clone_with_clone(MyError)]
#[derive(Debug, PartialEq, Eq)]
pub enum MyEnumWithClone {
    A,
    B(u8),
    C(i32, bool),
}

#[try_clone(MyError)]
#[derive(Debug, PartialEq, Eq)]
pub struct MyStruct {
    pub a: u8,
    pub b: i32,
    pub c: bool,
}

#[try_clone_with_clone(MyError)]
#[derive(Debug, PartialEq, Eq)]
pub struct MyStructWithClone {
    pub a: u8,
    pub b: i32,
    pub c: bool,
}

#[try_clone(MyError)]
pub struct MyStructEmpty;

#[try_clone(MyError)]
pub struct MyStructTuple(u8, i32, bool);

#[test]
fn test_my_enum() {
    let enum1 = MyEnum::A;
    let enum2 = enum1.try_clone().unwrap();
    assert_eq!(enum1, enum2);

    let enum1 = MyEnum::B(1);
    let enum2 = enum1.try_clone().unwrap();
    assert_eq!(enum1, enum2);

    let enum1 = MyEnum::C(2, true);
    let enum2 = enum1.try_clone().unwrap();
    assert_eq!(enum1, enum2);
}

#[test]
fn test_my_enum_with_clone() {
    let enum1 = MyEnumWithClone::A;
    let enum2 = enum1.try_clone().unwrap();
    let enum3 = enum1.clone();
    assert_eq!(enum1, enum2);
    assert_eq!(enum1, enum3);

    let enum1 = MyEnumWithClone::B(1);
    let enum2 = enum1.try_clone().unwrap();
    let enum3 = enum1.clone();
    assert_eq!(enum1, enum2);
    assert_eq!(enum1, enum3);

    let enum1 = MyEnumWithClone::C(2, true);
    let enum2 = enum1.try_clone().unwrap();
    let enum3 = enum1.clone();
    assert_eq!(enum1, enum2);
    assert_eq!(enum1, enum3);
}
