//! This crate defines various primitives that osom
//! libraries use.
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod macros {
    //! Holds several helpful macros related to primitives.

    #[doc(inline)]
    pub use ::osom_lib_primitives_proc_macros::*;
}

pub mod align;
pub mod cresult;
pub mod fraction;
pub mod length;
pub mod offset;
pub mod power_of_two;

pub(crate) mod as_i32;
mod length_ops;
mod offset_ops;

mod checks;
