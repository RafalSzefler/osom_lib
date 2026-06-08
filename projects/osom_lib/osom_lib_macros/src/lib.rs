//! This crate defines various macros for osom libraries.
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

/// Checks the condition in debug mode, but claims it is true in release mode.
#[macro_export]
macro_rules! debug_check_or_release_hint {
    ($condition:expr) => {
        #[cfg(debug_assertions)]
        {
            assert!($condition);
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            ::core::hint::assert_unchecked($condition);
        }
    };
    ($condition:expr, $msg: literal) => {
        #[cfg(debug_assertions)]
        {
            assert!($condition, $msg);
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            ::core::hint::assert_unchecked($condition);
        }
    };
}

/// Implements `From<Infallible> for $ty` by panicking.
#[macro_export]
macro_rules! unreachable_from_infallible {
    ($ty:ty) => {
        impl From<::core::convert::Infallible> for $ty {
            #[inline]
            fn from(_: ::core::convert::Infallible) -> Self {
                unreachable!("From<Infallible> for {} is not possible", stringify!($ty));
            }
        }
    };
}
