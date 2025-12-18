//! This crate defines various macros for osom libraries.
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
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
            core::hint::assert_unchecked($condition);
        }
    };
    ($condition:expr, $msg: literal) => {
        #[cfg(debug_assertions)]
        {
            assert!($condition, $msg);
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            core::hint::assert_unchecked($condition);
        }
    };
}
