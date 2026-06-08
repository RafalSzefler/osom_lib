use core::{convert::Infallible, marker::PhantomData, mem::MaybeUninit};

use super::TryClone;

macro_rules! try_clone_copy_impl {
    ($($t:ty),*) => {
        $(
            impl TryClone for $t {
                type Error = Infallible;
                #[inline(always)]
                fn try_clone(&self) -> Result<Self, Self::Error> {
                    Ok(*self)
                }
            }
        )*
    };
}

try_clone_copy_impl!(
    bool,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    (),
    char,
    usize,
    isize
);

impl<T: TryClone> TryClone for Option<T> {
    type Error = <T as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        match self {
            Some(value) => Ok(Some(value.try_clone()?)),
            None => Ok(None),
        }
    }
}

impl<const N: usize, T: TryClone> TryClone for [T; N] {
    type Error = <T as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let mut result = MaybeUninit::<[T; N]>::uninit();
        unsafe {
            let mut ptr = result.as_mut_ptr().cast::<T>();
            for value in self {
                ptr.write(value.try_clone()?);
                ptr = ptr.add(1);
            }
            Ok(result.assume_init())
        }
    }
}

impl<T: TryClone> TryClone for PhantomData<T> {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(PhantomData)
    }
}
