use std::marker::PhantomData;

use osom_lib_reprc::traits::ReprC;
use priv_osom_lib_reprc_proc_macros::reprc;

#[reprc]
pub struct MyUser {
    name: [u8; 124],
    age: i64,
}

#[reprc]
pub struct Foo<T>
where
    T: ReprC,
{
    pub val: u32,
    v2: T,
    v3: T,
}

#[reprc]
pub struct FooButInner<T: ReprC> {
    pub val: u32,
    v2: T,
    v3: T,
}

#[reprc]
#[repr(transparent)]
pub struct FooButTransparent<T: ReprC> {
    pub v: T,
}

#[reprc]
#[repr(u8)]
pub enum Baz {
    VAL = 1,
    BAZ(Foo<u8>) = 5,
}

#[reprc]
pub enum Gen<T> {
    VAL,
    PH(PhantomData<T>),
}
