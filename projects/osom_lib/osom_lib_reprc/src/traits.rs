//! Holds the `#[repr(C)]` related traits and implementations, in particular the [`ReprC`] trait.

use core::{
    cell::{RefCell, UnsafeCell},
    convert::Infallible,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ptr::NonNull,
    sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8, AtomicU16, AtomicU32,
        AtomicU64, AtomicUsize,
    },
};

use super::hidden::is_reprc;

/// Ensures that the type implementing it is `#[repr(C)]`.
/// This cannot be guaranteed in general, and therefore we
/// rely on macros to achieve that. This is a marker trait.
///
/// # Safety
///
/// This is an inherently unsafe trait. Any type implementing it
/// has to have `#[repr(C)]` set recursively.
pub unsafe trait ReprC {
    /// This field is used for const checks only.
    const CHECK: ();
}

macro_rules! impl_reprc {
    ( $t: ty ) => {
        unsafe impl ReprC for $t {
            const CHECK: () = ();
        }
    };

    ( $t: ty, $($ta: ty),* ) => {
        impl_reprc!($t);
        impl_reprc!($($ta),*);
    }
}

macro_rules! impl_generic_reprc {
    ( $t: ty ) => {
        unsafe impl<T: ReprC> ReprC for $t {
            const CHECK: () = const {
                is_reprc::<T>();
            };
        }
    };

    ( $t: ty, $($ta: ty),* ) => {
        impl_generic_reprc!($t);
        impl_generic_reprc!($($ta),*);
    }
}

impl_reprc!(
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    f32,
    f64,
    (),
    bool,
    isize,
    usize,
    AtomicBool,
    AtomicI8,
    AtomicU8,
    AtomicI16,
    AtomicU16,
    AtomicI32,
    AtomicU32,
    AtomicI64,
    AtomicU64,
    AtomicIsize,
    AtomicUsize,
    Infallible
);

impl_generic_reprc!(
    AtomicPtr<T>,
    *const T,
    *mut T,
    &T,
    &mut T,
    NonNull<T>,
    ManuallyDrop<T>,
    MaybeUninit<T>
);

unsafe impl<T: ReprC, const N: usize> ReprC for [T; N] {
    const CHECK: () = const {
        is_reprc::<T>();
    };
}

// PhantomData is a special case, that works with any T, since its size is 0
// anyway.
unsafe impl<T> ReprC for PhantomData<T> {
    const CHECK: () = const {
        assert!(size_of::<PhantomData<T>>() == 0);
    };
}

unsafe impl<T: ReprC> ReprC for UnsafeCell<T> {
    const CHECK: () = const {
        is_reprc::<T>();
    };
}

unsafe impl<T: ReprC> ReprC for RefCell<T> {
    const CHECK: () = const {
        is_reprc::<T>();
    };
}
